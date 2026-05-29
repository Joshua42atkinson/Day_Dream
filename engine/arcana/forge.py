#!/usr/bin/env python3
"""
ARCANA Forge — The Card Generation Pipeline

A production-quality pipeline for generating ARCANA power word cards
using LongCat-Image via ComfyUI. Features:

  - QUALITY GATES: Every stage requires human approval before proceeding
  - MANIFEST TRACKING: JSON manifest records every card's status
  - BATCH CONTROL: Generate one card at a time or in batches
  - IDEMPOTENT: Re-running skips already-approved cards
  - DRY RUN: Preview all prompts without touching the GPU

Pipeline Stages:
  1. PROMPT REVIEW    — Review generated prompts before sending to GPU
  2. GENERATION       — Submit to ComfyUI, wait for result
  3. ART REVIEW       — Human evaluates generated art (approve/reject/regenerate)
  4. MANIFEST UPDATE  — Approved cards are logged with metadata

Usage:
    # Dry run — preview prompts only (safe during training)
    python forge.py --dry-run

    # Generate one card at a time with approval gates
    python forge.py --word Presence

    # Generate all cards in curriculum
    python forge.py --all

    # Review and approve/reject generated cards
    python forge.py --review
"""

import argparse
import json
import sys
import time
from datetime import datetime, timezone
from pathlib import Path
from dataclasses import dataclass, asdict

# Local imports
from prompts import build_card_prompt, build_all_prompts
from comfyui_client import ComfyUIClient, GenerationResult


# ─── PATHS ────────────────────────────────────────────────────

ARCANA_DIR = Path(__file__).parent
CARDS_DIR = ARCANA_DIR / "cards"
MANIFEST_PATH = ARCANA_DIR / "manifest.json"
CURRICULUM_PATH = ARCANA_DIR / "curriculum.json"
REJECTED_DIR = ARCANA_DIR / "rejected"


# ─── MANIFEST SCHEMA ─────────────────────────────────────────

@dataclass
class CardEntry:
    """A single card in the ARCANA manifest."""
    word: str
    channel: str
    stage: str
    mastery: str
    status: str  # "pending" | "prompt_approved" | "generated" | "approved" | "rejected"
    prompt: str = ""
    negative_prompt: str = ""
    image_path: str = ""
    seed: int = -1
    generation_time_s: float = 0.0
    generated_at: str = ""
    approved_at: str = ""
    rejection_reason: str = ""
    attempt_count: int = 0


class Manifest:
    """
    The ARCANA manifest — tracks every card's lifecycle.
    Persisted as JSON for human readability and version control.
    """

    def __init__(self, path: Path = MANIFEST_PATH):
        self.path = path
        self.cards: dict[str, CardEntry] = {}
        self.load()

    def load(self):
        """Load manifest from disk, or initialize empty."""
        if self.path.exists():
            with open(self.path) as f:
                data = json.load(f)
            for key, entry in data.get("cards", {}).items():
                self.cards[key] = CardEntry(**entry)

    def save(self):
        """Persist manifest to disk."""
        self.path.parent.mkdir(parents=True, exist_ok=True)
        data = {
            "version": "1.0",
            "updated_at": datetime.now(timezone.utc).isoformat(),
            "total_cards": len(self.cards),
            "approved": sum(1 for c in self.cards.values() if c.status == "approved"),
            "pending": sum(1 for c in self.cards.values() if c.status != "approved"),
            "cards": {k: asdict(v) for k, v in self.cards.items()},
        }
        with open(self.path, "w") as f:
            json.dump(data, f, indent=2)

    def get_card(self, word: str, mastery: str = "owned") -> CardEntry:
        """Get or create a card entry."""
        key = f"{word.lower()}_{mastery}"
        if key not in self.cards:
            self.cards[key] = CardEntry(
                word=word, channel="", stage="", mastery=mastery,
                status="pending"
            )
        return self.cards[key]

    def set_card(self, word: str, mastery: str, entry: CardEntry):
        """Update a card entry."""
        key = f"{word.lower()}_{mastery}"
        self.cards[key] = entry
        self.save()


# ─── QUALITY GATES ────────────────────────────────────────────

def gate_prompt_review(word: str, prompt: str, negative: str) -> bool:
    """
    GATE 1: Human reviews the generated prompt before GPU submission.
    Returns True if approved, False if rejected.
    """
    print(f"\n{'━'*70}")
    print(f"  QUALITY GATE 1: PROMPT REVIEW")
    print(f"  Word: {word}")
    print(f"{'━'*70}")
    print(f"\n  PROMPT:")
    print(f"  {prompt}")
    print(f"\n  NEGATIVE:")
    print(f"  {negative}")
    print(f"\n{'━'*70}")

    while True:
        choice = input("  [A]pprove / [R]eject / [E]dit prompt? ").strip().lower()
        if choice in ("a", "approve"):
            return True
        elif choice in ("r", "reject"):
            return False
        elif choice in ("e", "edit"):
            print("  (Edit the prompt in your editor, then paste the new version)")
            # Future: open $EDITOR
            return False
        else:
            print("  Please enter A, R, or E.")


def gate_art_review(word: str, image_path: str) -> str:
    """
    GATE 2: Human evaluates the generated card art.
    Returns: "approved" | "rejected" | "regenerate"
    """
    print(f"\n{'━'*70}")
    print(f"  QUALITY GATE 2: ART REVIEW")
    print(f"  Word: {word}")
    print(f"  Image: {image_path}")
    print(f"{'━'*70}")
    print(f"\n  Please view the generated image at:")
    print(f"  file://{image_path}")
    print()

    while True:
        choice = input(
            "  [A]pprove / [R]eject / Re[G]enerate (new seed)? "
        ).strip().lower()
        if choice in ("a", "approve"):
            return "approved"
        elif choice in ("r", "reject"):
            reason = input("  Rejection reason (optional): ").strip()
            return f"rejected:{reason}"
        elif choice in ("g", "regenerate"):
            return "regenerate"
        else:
            print("  Please enter A, R, or G.")


# ─── PIPELINE STAGES ─────────────────────────────────────────

def stage_dry_run(curriculum: dict, mastery: str = "owned"):
    """
    DRY RUN: Preview all prompts without GPU usage.
    Safe to run during active fine-tuning.
    """
    all_prompts = build_all_prompts(curriculum, mastery=mastery)

    print(f"\n{'═'*70}")
    print(f"  ARCANA FORGE — DRY RUN")
    print(f"  Curriculum: {curriculum.get('name', 'unnamed')}")
    print(f"  Words: {len(all_prompts)}")
    print(f"  Mastery Level: {mastery}")
    print(f"  GPU Required: NO (preview only)")
    print(f"{'═'*70}")

    for i, p in enumerate(all_prompts, 1):
        print(f"\n{'─'*70}")
        print(f"  [{i}/{len(all_prompts)}] {p['word']} ({p['channel'].upper()})")
        print(f"{'─'*70}")
        print(f"  PROMPT:")
        # Word-wrap the prompt for readability
        words = p["prompt"].split()
        line = "  "
        for w in words:
            if len(line) + len(w) > 68:
                print(line)
                line = "  " + w
            else:
                line += " " + w
        if line.strip():
            print(line)

    print(f"\n{'═'*70}")
    print(f"  DRY RUN COMPLETE — {len(all_prompts)} prompts previewed")
    print(f"  Run with --all to generate, or --word <name> for one card")
    print(f"{'═'*70}\n")


def stage_generate_card(
    word_def: dict,
    client: ComfyUIClient,
    manifest: Manifest,
    mastery: str = "owned",
    auto_approve_prompt: bool = False,
    max_attempts: int = 3,
) -> bool:
    """
    Full pipeline for a single card: prompt → review → generate → review.

    Args:
        word_def: Word definition dict from curriculum
        client: ComfyUI client instance
        manifest: Manifest for tracking
        mastery: Mastery level for art style
        auto_approve_prompt: Skip prompt review gate (for batch mode)
        max_attempts: Max generation attempts before giving up

    Returns:
        True if card was approved, False otherwise
    """
    word = word_def["word"]
    channel = word_def.get("channel", "body")
    stage = word_def.get("stage", "hero")

    # Check if already approved
    entry = manifest.get_card(word, mastery)
    if entry.status == "approved":
        print(f"  ✅ {word} ({mastery}) — already approved, skipping")
        return True

    # Build prompt
    prompts = build_card_prompt(
        word=word,
        channel=channel,
        mood=word_def.get("mood", "neutral"),
        themes=word_def.get("themes", []),
        mastery=mastery,
        story_hint=word_def.get("story_text", ""),
    )

    entry.prompt = prompts["prompt"]
    entry.negative_prompt = prompts["negative_prompt"]
    entry.channel = channel
    entry.stage = stage

    # ─── GATE 1: Prompt Review ────────────────────────────
    if not auto_approve_prompt:
        approved = gate_prompt_review(word, entry.prompt, entry.negative_prompt)
        if not approved:
            entry.status = "rejected"
            entry.rejection_reason = "Prompt rejected by reviewer"
            manifest.set_card(word, mastery, entry)
            print(f"  ❌ {word} — prompt rejected")
            return False

    entry.status = "prompt_approved"
    manifest.set_card(word, mastery, entry)

    # ─── GENERATION + GATE 2: Art Review ──────────────────
    for attempt in range(1, max_attempts + 1):
        print(f"\n  🎨 Generating {word} (attempt {attempt}/{max_attempts})...")

        # Pre-flight check
        gpu = client.get_gpu_status()
        if not gpu.get("available"):
            print("  ⚠️  ComfyUI not available. Is it running on :8188?")
            return False

        vram_used = gpu.get("vram_used_pct", 0)
        if vram_used > 80:
            print(f"  ⚠️  GPU VRAM {vram_used}% used — another process may be active")
            confirm = input("  Continue anyway? [y/N] ").strip().lower()
            if confirm != "y":
                return False

        # Generate
        save_prefix = f"arcana_{word.lower()}_{mastery}"
        result = client.generate_image(
            prompt=entry.prompt,
            negative_prompt=entry.negative_prompt,
            output_dir=CARDS_DIR,
            width=1024,
            height=1024,
            steps=25,
            cfg=4.5,
            save_prefix=save_prefix,
        )

        entry.attempt_count = attempt
        entry.generation_time_s = result.elapsed_seconds

        if not result.success:
            print(f"  ❌ Generation failed: {result.error}")
            entry.status = "rejected"
            entry.rejection_reason = result.error
            manifest.set_card(word, mastery, entry)
            continue

        entry.image_path = result.image_paths[0] if result.image_paths else ""
        entry.generated_at = datetime.now(timezone.utc).isoformat()
        entry.status = "generated"
        manifest.set_card(word, mastery, entry)

        print(f"  ✅ Generated in {result.elapsed_seconds:.1f}s → {entry.image_path}")

        # ─── GATE 2: Art Review ───────────────────────────
        verdict = gate_art_review(word, entry.image_path)

        if verdict == "approved":
            entry.status = "approved"
            entry.approved_at = datetime.now(timezone.utc).isoformat()
            manifest.set_card(word, mastery, entry)
            print(f"  👑 {word} ({mastery}) — APPROVED")
            return True
        elif verdict == "regenerate":
            print(f"  🔄 Regenerating with new seed...")
            # Move current to rejected dir
            if entry.image_path and Path(entry.image_path).exists():
                REJECTED_DIR.mkdir(parents=True, exist_ok=True)
                rej_name = f"{word.lower()}_{mastery}_attempt{attempt}.png"
                Path(entry.image_path).rename(REJECTED_DIR / rej_name)
            continue
        else:
            # Rejected
            reason = verdict.split(":", 1)[1] if ":" in verdict else ""
            entry.status = "rejected"
            entry.rejection_reason = reason
            manifest.set_card(word, mastery, entry)
            if entry.image_path and Path(entry.image_path).exists():
                REJECTED_DIR.mkdir(parents=True, exist_ok=True)
                rej_name = f"{word.lower()}_{mastery}_rejected.png"
                Path(entry.image_path).rename(REJECTED_DIR / rej_name)
            print(f"  ❌ {word} — art rejected: {reason}")
            return False

    print(f"  ❌ {word} — max attempts ({max_attempts}) reached")
    return False


def stage_review(manifest: Manifest):
    """
    Review all cards in the manifest — show status summary.
    """
    print(f"\n{'═'*70}")
    print(f"  ARCANA MANIFEST REVIEW")
    print(f"{'═'*70}")

    if not manifest.cards:
        print("  No cards in manifest yet. Run --dry-run first.")
        return

    statuses = {}
    for key, card in sorted(manifest.cards.items()):
        status = card.status
        statuses[status] = statuses.get(status, 0) + 1

        icon = {
            "pending": "⬜",
            "prompt_approved": "📝",
            "generated": "🖼️",
            "approved": "✅",
            "rejected": "❌",
        }.get(status, "❓")

        time_str = ""
        if card.generation_time_s > 0:
            time_str = f" ({card.generation_time_s:.1f}s)"

        path_str = ""
        if card.image_path:
            path_str = f"\n           file://{card.image_path}"

        print(f"\n  {icon} {card.word} [{card.channel.upper()}/{card.mastery}]"
              f" — {status}{time_str}{path_str}")

        if card.rejection_reason:
            print(f"           Reason: {card.rejection_reason}")

    print(f"\n{'─'*70}")
    print(f"  Summary:")
    for status, count in sorted(statuses.items()):
        print(f"    {status}: {count}")
    print(f"  Total: {len(manifest.cards)}")
    print(f"{'═'*70}\n")


# ─── CURRICULUM LOADER ────────────────────────────────────────

def load_curriculum(path: Path = CURRICULUM_PATH) -> dict:
    """
    Load curriculum from JSON file, or fall back to hardcoded demo.
    """
    if path.exists():
        with open(path) as f:
            return json.load(f)

    # Fallback: hardcoded demo curriculum (Bias & Mirrors)
    return {
        "name": "Bias & Mirrors",
        "start_word": "Presence",
        "words": [
            {
                "word": "Presence",
                "channel": "body", "stage": "hero",
                "mood": "calm",
                "themes": ["awareness", "grounding"],
                "story_text": "A still stone archway with warm light spilling through, "
                              "ancient and grounding. A single figure stands in silhouette",
                "synergy_partners": ["Patience"],
            },
            {
                "word": "Bias",
                "channel": "mind", "stage": "edge_lord",
                "mood": "mysterious",
                "themes": ["self-awareness", "critical-thinking"],
                "story_text": "Dark glass trees reflecting warped, distorted versions "
                              "of the viewer. Fractured mirrors floating in fog",
                "synergy_partners": ["Clarity"],
            },
            {
                "word": "Patience",
                "channel": "body", "stage": "hero",
                "mood": "warm",
                "themes": ["growth", "stillness"],
                "story_text": "A walled garden at golden hour, fountain at center, "
                              "birdsong implied by scattered feathers on the water",
                "synergy_partners": ["Presence", "Resilience"],
            },
            {
                "word": "Resilience",
                "channel": "action", "stage": "outlaw",
                "mood": "tense",
                "themes": ["courage", "persistence"],
                "story_text": "A narrow bridge over a vast canyon, wind visible as "
                              "golden streaks. A single figure mid-step leaning into gale",
                "synergy_partners": ["Patience"],
            },
            {
                "word": "Clarity",
                "channel": "mind", "stage": "best_self",
                "mood": "transcendent",
                "themes": ["wisdom", "resolution"],
                "story_text": "Above the clouds, the world visible below as a vast map. "
                              "Crystal-clear atmosphere, infinite visibility",
                "synergy_partners": ["Bias"],
            },
        ],
    }


def save_demo_curriculum():
    """Write the demo curriculum to curriculum.json for future editing."""
    curriculum = load_curriculum()
    CURRICULUM_PATH.parent.mkdir(parents=True, exist_ok=True)
    with open(CURRICULUM_PATH, "w") as f:
        json.dump(curriculum, f, indent=2)
    print(f"  📜 Demo curriculum saved to {CURRICULUM_PATH}")


# ─── CLI ENTRY POINT ─────────────────────────────────────────

def main():
    parser = argparse.ArgumentParser(
        description="ARCANA Forge — Power Word Card Generator",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  python forge.py --dry-run              Preview all prompts (no GPU)
  python forge.py --word Presence        Generate one card with gates
  python forge.py --all                  Generate all cards with gates
  python forge.py --review               Review manifest status
  python forge.py --save-curriculum      Save demo curriculum.json
  python forge.py --mastery encountered  Generate sketch-style cards
        """,
    )
    parser.add_argument("--dry-run", action="store_true",
                        help="Preview prompts only — no GPU usage")
    parser.add_argument("--word", type=str,
                        help="Generate a single word card")
    parser.add_argument("--all", action="store_true",
                        help="Generate all cards in curriculum")
    parser.add_argument("--review", action="store_true",
                        help="Review manifest status")
    parser.add_argument("--save-curriculum", action="store_true",
                        help="Save demo curriculum to curriculum.json")
    parser.add_argument("--mastery", type=str, default="owned",
                        choices=["encountered", "experienced", "owned", "mastered"],
                        help="Mastery level for art style")
    parser.add_argument("--auto-approve-prompts", action="store_true",
                        help="Skip prompt review gate (batch mode)")
    parser.add_argument("--comfyui-url", type=str,
                        default="http://127.0.0.1:8188",
                        help="ComfyUI server URL")

    args = parser.parse_args()

    # Load curriculum
    curriculum = load_curriculum()
    manifest = Manifest()

    if args.save_curriculum:
        save_demo_curriculum()
        return

    if args.review:
        stage_review(manifest)
        return

    if args.dry_run:
        stage_dry_run(curriculum, mastery=args.mastery)
        return

    # GPU operations — check ComfyUI first
    client = ComfyUIClient(args.comfyui_url)
    gpu = client.get_gpu_status()

    if not gpu.get("available"):
        print("\n  ❌ ComfyUI is not running on", args.comfyui_url)
        print("  Start it with: cd ~/ComfyUI && source venv/bin/activate && python main.py")
        sys.exit(1)

    print(f"\n  🖥️  GPU: {gpu.get('gpu', 'unknown')}")
    print(f"  💾 VRAM: {gpu.get('vram_free_gb', '?')} GB free "
          f"/ {gpu.get('vram_total_gb', '?')} GB total "
          f"({gpu.get('vram_used_pct', '?')}% used)")

    if gpu.get("vram_used_pct", 0) > 50:
        print("\n  ⚠️  WARNING: GPU memory is over 50% used.")
        print("  Another process (Unsloth?) may be active.")
        confirm = input("  Proceed with generation? [y/N] ").strip().lower()
        if confirm != "y":
            print("  Aborted. Run --dry-run instead.")
            sys.exit(0)

    if args.word:
        # Single card
        word_def = None
        for w in curriculum.get("words", []):
            if w["word"].lower() == args.word.lower():
                word_def = w
                break

        if not word_def:
            print(f"\n  ❌ Word '{args.word}' not found in curriculum")
            print(f"  Available: {[w['word'] for w in curriculum.get('words', [])]}")
            sys.exit(1)

        stage_generate_card(
            word_def, client, manifest,
            mastery=args.mastery,
            auto_approve_prompt=args.auto_approve_prompts,
        )

    elif args.all:
        # All cards
        words = curriculum.get("words", [])
        approved = 0
        total = len(words)

        print(f"\n  🔨 ARCANA FORGE — Generating {total} cards")
        print(f"  Mastery: {args.mastery}")
        print(f"  Quality Gates: {'AUTO' if args.auto_approve_prompts else 'MANUAL'}")

        for i, word_def in enumerate(words, 1):
            print(f"\n{'━'*70}")
            print(f"  [{i}/{total}] {word_def['word']}")
            print(f"{'━'*70}")

            success = stage_generate_card(
                word_def, client, manifest,
                mastery=args.mastery,
                auto_approve_prompt=args.auto_approve_prompts,
            )
            if success:
                approved += 1

        print(f"\n{'═'*70}")
        print(f"  FORGE COMPLETE: {approved}/{total} cards approved")
        print(f"  Manifest: {MANIFEST_PATH}")
        print(f"{'═'*70}\n")

    else:
        parser.print_help()


if __name__ == "__main__":
    main()
