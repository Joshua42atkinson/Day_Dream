#!/usr/bin/env python3
"""
ARCANA Builder — Transforms raw text/word lists into VAAM curriculum DAGs.

The curriculum builder pipeline:
  1. EXTRACT: Pull vocabulary from text input
  2. CLASSIFY: Auto-assign Channel, Stage, Symbol per word
  3. CONNECT: Build DAG edges (prerequisites + synergies)
  4. REVIEW: Human approval at every gate
  5. EXPORT: Output curriculum.json ready for the engine

This is "perspective engineering" — the same systematic approach
used in AI dataset curation, applied to human learning content.

Usage:
    # From a word list
    python builder.py --words "Resilience,Patience,Clarity,Bias,Presence"

    # From a text file (extracts vocabulary automatically)
    python builder.py --text chapter1.txt --min-frequency 2

    # Interactive mode with approval gates
    python builder.py --words "..." --interactive
"""

import argparse
import json
import re
import sys
from collections import Counter
from pathlib import Path
from dataclasses import dataclass, asdict

from classifier import classify_word, classify_word_list, WordClassification


# ─── TEXT EXTRACTION ──────────────────────────────────────────

# Common English stop words to filter out
STOP_WORDS = {
    "the", "a", "an", "is", "are", "was", "were", "be", "been", "being",
    "have", "has", "had", "do", "does", "did", "will", "would", "could",
    "should", "may", "might", "shall", "can", "need", "must", "ought",
    "i", "me", "my", "we", "us", "our", "you", "your", "he", "him",
    "his", "she", "her", "it", "its", "they", "them", "their", "this",
    "that", "these", "those", "what", "which", "who", "whom", "where",
    "when", "why", "how", "all", "each", "every", "both", "few", "more",
    "most", "other", "some", "such", "no", "not", "only", "same", "so",
    "than", "too", "very", "just", "about", "above", "after", "again",
    "also", "and", "any", "because", "before", "between", "but", "by",
    "for", "from", "if", "in", "into", "of", "on", "or", "out", "over",
    "own", "per", "then", "to", "under", "up", "with", "as", "at",
}


def extract_vocabulary(
    text: str,
    min_length: int = 4,
    min_frequency: int = 1,
    max_words: int = 50,
) -> list[dict]:
    """
    Extract candidate vocabulary words from raw text.

    Returns list of {word, frequency, contexts} dicts.
    """
    # Tokenize
    words_raw = re.findall(r'\b[A-Za-z]+\b', text)
    words_lower = [w.lower() for w in words_raw if len(w) >= min_length]

    # Filter stop words
    filtered = [w for w in words_lower if w not in STOP_WORDS]

    # Count frequencies
    freq = Counter(filtered)

    # Get context sentences for each word
    sentences = re.split(r'[.!?]+', text)
    contexts = {}
    for word in freq:
        for sent in sentences:
            if re.search(r'\b' + re.escape(word) + r'\b', sent, re.IGNORECASE):
                contexts[word] = sent.strip()[:200]
                break

    # Filter by frequency and sort
    candidates = [
        {"word": word.capitalize(), "frequency": count, "context": contexts.get(word, "")}
        for word, count in freq.most_common()
        if count >= min_frequency
    ]

    return candidates[:max_words]


# ─── DAG CONSTRUCTION ────────────────────────────────────────

def build_edges(
    classifications: list[WordClassification],
) -> dict[str, dict]:
    """
    Build DAG edges between words based on classification proximity.

    Heuristic rules:
    - Words in the same channel with adjacent stages → prerequisite edge
    - Words sharing 2+ theme seeds → synergy edge
    - Lower stage words are prerequisites for higher stage words
    """
    stage_order = {"hero": 0, "outlaw": 1, "edge_lord": 2, "best_self": 3}
    words = {c.word: c for c in classifications}
    edges = {}

    for word, cls in words.items():
        yes_targets = []
        no_targets = []
        synergy_partners = []

        for other_word, other_cls in words.items():
            if other_word == word:
                continue

            same_channel = cls.channel == other_cls.channel
            my_stage = stage_order.get(cls.stage, 0)
            their_stage = stage_order.get(other_cls.stage, 0)

            # Prerequisite: same channel, they're one stage higher
            if same_channel and their_stage == my_stage + 1:
                yes_targets.append(other_word)

            # Synergy: different channel, similar stage
            if not same_channel and abs(my_stage - their_stage) <= 1:
                synergy_partners.append(other_word)

            # No-path: same channel, same or lower stage (lateral movement)
            if same_channel and their_stage <= my_stage and other_word != word:
                if other_word not in yes_targets:
                    no_targets.append(other_word)

        edges[word] = {
            "yes_targets": yes_targets[:2],      # Max 2 forward paths
            "no_targets": no_targets[:2],         # Max 2 lateral paths
            "synergy_partners": synergy_partners[:3],  # Max 3 synergies
        }

    return edges


# ─── MOOD INFERENCE ───────────────────────────────────────────

MOOD_MAP = {
    "mind": {"hero": "curious", "outlaw": "mysterious", "edge_lord": "intense", "best_self": "transcendent"},
    "heart": {"hero": "warm", "outlaw": "passionate", "edge_lord": "raw", "best_self": "luminous"},
    "body": {"hero": "calm", "outlaw": "tense", "edge_lord": "visceral", "best_self": "oceanic"},
    "action": {"hero": "eager", "outlaw": "fierce", "edge_lord": "relentless", "best_self": "sovereign"},
}

def infer_mood(channel: str, stage: str) -> str:
    """Infer a mood from channel + stage combination."""
    return MOOD_MAP.get(channel, {}).get(stage, "neutral")


# ─── QUALITY GATES ────────────────────────────────────────────

def gate_extraction_review(candidates: list[dict]) -> list[dict]:
    """GATE 1: Review extracted vocabulary."""
    print(f"\n{'━'*70}")
    print(f"  GATE 1: VOCABULARY EXTRACTION REVIEW")
    print(f"  {len(candidates)} words extracted")
    print(f"{'━'*70}")

    for i, c in enumerate(candidates):
        print(f"  {i+1:3d}. {c['word']:20s} (freq: {c['frequency']})")

    print(f"\n  Actions:")
    print(f"    [A] Approve all")
    print(f"    [R] Remove words by number (e.g., '3,7,12')")
    print(f"    [+] Add a word")
    print(f"    [Q] Quit")

    while True:
        choice = input("\n  > ").strip().lower()
        if choice in ("a", "approve"):
            return candidates
        elif choice.startswith("r ") or choice.startswith("remove "):
            nums = re.findall(r'\d+', choice)
            indices = {int(n) - 1 for n in nums}
            candidates = [c for i, c in enumerate(candidates) if i not in indices]
            print(f"  Removed {len(indices)} words. {len(candidates)} remaining.")
        elif choice.startswith("+"):
            new_word = choice[1:].strip().capitalize()
            if new_word:
                candidates.append({"word": new_word, "frequency": 1, "context": ""})
                print(f"  Added: {new_word}")
        elif choice in ("q", "quit"):
            return []
        else:
            print("  Enter A, R <numbers>, + <word>, or Q")


def gate_classification_review(
    classifications: list[WordClassification],
) -> list[WordClassification]:
    """GATE 2: Review auto-classifications."""
    print(f"\n{'━'*70}")
    print(f"  GATE 2: CLASSIFICATION REVIEW")
    print(f"{'━'*70}")

    for i, c in enumerate(classifications):
        conf_bar = "█" * int(c.confidence * 10) + "░" * (10 - int(c.confidence * 10))
        print(f"  {i+1:3d}. {c.symbol_icon} {c.word:20s} "
              f"{c.channel.upper():8s} {c.stage:12s} {c.rarity:10s} "
              f"[{conf_bar}] {c.confidence:.0%}")

    low_conf = [c for c in classifications if c.confidence < 0.4]
    if low_conf:
        print(f"\n  ⚠️  {len(low_conf)} words have low confidence (<40%):")
        for c in low_conf:
            print(f"      {c.word}: {c.reasoning}")

    print(f"\n  [A] Approve all  [Q] Quit")
    choice = input("  > ").strip().lower()
    if choice in ("a", "approve"):
        return classifications
    return []


def gate_dag_review(edges: dict) -> bool:
    """GATE 3: Review DAG structure."""
    print(f"\n{'━'*70}")
    print(f"  GATE 3: DAG STRUCTURE REVIEW")
    print(f"{'━'*70}")

    for word, e in edges.items():
        yes = ", ".join(e["yes_targets"]) or "(end)"
        no = ", ".join(e["no_targets"]) or "(end)"
        syn = ", ".join(e["synergy_partners"]) or "(none)"
        print(f"\n  {word}:")
        print(f"    → Yes: {yes}")
        print(f"    ← No:  {no}")
        print(f"    ⚡ Synergy: {syn}")

    print(f"\n  [A] Approve  [Q] Quit")
    choice = input("  > ").strip().lower()
    return choice in ("a", "approve")


# ─── EXPORT ───────────────────────────────────────────────────

def export_curriculum(
    name: str,
    classifications: list[WordClassification],
    edges: dict,
    output_path: Path,
) -> dict:
    """Export a complete curriculum.json."""
    # Find the best start word (lowest stage, Body channel preferred)
    stage_order = {"hero": 0, "outlaw": 1, "edge_lord": 2, "best_self": 3}
    sorted_words = sorted(classifications, key=lambda c: stage_order.get(c.stage, 0))
    start_word = sorted_words[0].word if sorted_words else ""

    words = []
    for cls in classifications:
        word_edges = edges.get(cls.word, {})
        mood = infer_mood(cls.channel, cls.stage)
        words.append({
            "word": cls.word,
            "channel": cls.channel,
            "stage": cls.stage,
            "symbol": cls.symbol,
            "symbol_icon": cls.symbol_icon,
            "rarity": cls.rarity,
            "mood": mood,
            "themes": [],  # To be filled by instructor or LLM
            "story_text": "",  # To be filled by LLM per genre
            "depth_prompt": "",  # To be filled by instructor or LLM
            "yes_targets": word_edges.get("yes_targets", []),
            "no_targets": word_edges.get("no_targets", []),
            "synergy_partners": word_edges.get("synergy_partners", []),
        })

    curriculum = {
        "name": name,
        "start_word": start_word,
        "version": "1.0",
        "words": words,
    }

    output_path.parent.mkdir(parents=True, exist_ok=True)
    with open(output_path, "w") as f:
        json.dump(curriculum, f, indent=2)

    return curriculum


# ─── MAIN PIPELINE ────────────────────────────────────────────

def run_pipeline(
    words: list[str] | None = None,
    text_path: Path | None = None,
    name: str = "Untitled Curriculum",
    output: Path = Path("curriculum_new.json"),
    interactive: bool = True,
    min_frequency: int = 2,
):
    """Run the full builder pipeline with quality gates."""
    print(f"\n{'═'*70}")
    print(f"  ARCANA BUILDER — Curriculum Construction Pipeline")
    print(f"  Name: {name}")
    print(f"  Interactive: {'YES (with gates)' if interactive else 'AUTO'}")
    print(f"{'═'*70}")

    # ─── STAGE 1: Extract ─────────────────────────────────
    if words:
        candidates = [{"word": w.strip().capitalize(), "frequency": 1, "context": ""}
                      for w in words]
    elif text_path and text_path.exists():
        text = text_path.read_text()
        candidates = extract_vocabulary(text, min_frequency=min_frequency)
        print(f"\n  📖 Extracted {len(candidates)} candidate words from {text_path.name}")
    else:
        print("  ❌ No input provided. Use --words or --text")
        return

    if interactive:
        candidates = gate_extraction_review(candidates)
        if not candidates:
            print("  Aborted at extraction gate.")
            return

    # ─── STAGE 2: Classify ────────────────────────────────
    print(f"\n  🔬 Classifying {len(candidates)} words...")
    contexts = {c["word"]: c.get("context", "") for c in candidates}
    word_list = [c["word"] for c in candidates]
    classifications = classify_word_list(word_list, contexts)

    if interactive:
        classifications = gate_classification_review(classifications)
        if not classifications:
            print("  Aborted at classification gate.")
            return

    # ─── STAGE 3: Connect ─────────────────────────────────
    print(f"\n  🔗 Building DAG edges...")
    edges = build_edges(classifications)

    if interactive:
        approved = gate_dag_review(edges)
        if not approved:
            print("  Aborted at DAG review gate.")
            return

    # ─── STAGE 4: Export ──────────────────────────────────
    curriculum = export_curriculum(name, classifications, edges, output)

    print(f"\n{'═'*70}")
    print(f"  ✅ CURRICULUM BUILT SUCCESSFULLY")
    print(f"  Output: {output}")
    print(f"  Words: {len(curriculum['words'])}")
    print(f"  Start: {curriculum['start_word']}")
    print(f"{'═'*70}")
    print(f"\n  Next steps:")
    print(f"  1. Edit {output} to add themes, story_text, depth_prompt")
    print(f"  2. Run: python forge.py --dry-run  (preview card prompts)")
    print(f"  3. Run: python forge.py --all      (generate card art)")


# ─── CLI ──────────────────────────────────────────────────────

def main():
    parser = argparse.ArgumentParser(
        description="ARCANA Builder — Curriculum → VAAM DAG",
    )
    parser.add_argument("--words", type=str,
                        help="Comma-separated word list")
    parser.add_argument("--text", type=str,
                        help="Path to text file to extract vocabulary from")
    parser.add_argument("--name", type=str, default="Untitled Curriculum",
                        help="Curriculum name")
    parser.add_argument("--output", type=str, default="curriculum_new.json",
                        help="Output path for curriculum JSON")
    parser.add_argument("--interactive", action="store_true", default=True,
                        help="Enable quality gates (default: True)")
    parser.add_argument("--auto", action="store_true",
                        help="Skip quality gates (auto-approve all)")
    parser.add_argument("--min-frequency", type=int, default=2,
                        help="Min word frequency for text extraction")

    args = parser.parse_args()

    words = args.words.split(",") if args.words else None
    text_path = Path(args.text) if args.text else None
    output = Path(args.output)
    interactive = not args.auto

    run_pipeline(
        words=words,
        text_path=text_path,
        name=args.name,
        output=output,
        interactive=interactive,
        min_frequency=args.min_frequency,
    )


if __name__ == "__main__":
    main()
