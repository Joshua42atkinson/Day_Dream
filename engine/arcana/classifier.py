#!/usr/bin/env python3
"""
ARCANA Classifier — Auto-assigns Channel, Stage, Symbol, and Rarity to words.

Uses heuristic NLP analysis (no GPU required) to classify vocabulary:
  - Channel: Mind/Heart/Body/Action based on semantic domain
  - Stage: Hero/Outlaw/EdgeLord/BestSelf based on complexity + frequency
  - Symbol: Stone/Spark/Prism/Void/Star based on grammar role
  - Rarity: Common/Uncommon/Rare/Legendary derived from Stage

This is the "perspective engineering" layer — same principles as
AI fine-tuning hyperparameter selection, but for human learners.
"""

import re
from dataclasses import dataclass

# ─── SEMANTIC DOMAIN LEXICONS ─────────────────────────────────
# These are seed words that anchor each channel's semantic space.
# A word is classified by proximity to these domains.

MIND_SEEDS = {
    "think", "analyze", "reason", "logic", "theory", "concept", "abstract",
    "pattern", "framework", "model", "hypothesis", "evidence", "proof",
    "calculate", "deduce", "infer", "classify", "categorize", "structure",
    "system", "method", "strategy", "evaluate", "critique", "bias",
    "perspective", "insight", "clarity", "wisdom", "knowledge", "understand",
    "metacognition", "reflection", "awareness", "consciousness", "paradigm",
    "synthesis", "analysis", "cognition", "schema", "algorithm", "data",
}

HEART_SEEDS = {
    "feel", "love", "empathy", "compassion", "courage", "hope", "joy",
    "grief", "anger", "fear", "trust", "bond", "connect", "relate",
    "emotion", "passion", "devotion", "loyalty", "forgive", "gratitude",
    "vulnerability", "intimacy", "belonging", "community", "care",
    "nurture", "inspire", "motivate", "spirit", "soul", "beauty",
    "harmony", "kindness", "warmth", "tenderness", "faith", "desire",
    "longing", "sorrow", "wonder", "awe", "reverence", "celebrate",
}

BODY_SEEDS = {
    "body", "breathe", "ground", "sense", "touch", "feel", "move",
    "rest", "sleep", "eat", "grow", "root", "earth", "water",
    "stone", "mountain", "ocean", "river", "tree", "seed", "soil",
    "presence", "stillness", "patience", "calm", "peace", "quiet",
    "weight", "gravity", "balance", "center", "anchor", "settle",
    "somatic", "visceral", "intuition", "instinct", "rhythm", "pulse",
    "health", "heal", "nourish", "sustain", "endure", "persist",
}

ACTION_SEEDS = {
    "build", "create", "make", "forge", "craft", "construct", "design",
    "act", "do", "execute", "implement", "launch", "start", "begin",
    "lead", "guide", "direct", "manage", "organize", "plan", "decide",
    "fight", "defend", "protect", "serve", "sacrifice", "commit",
    "work", "effort", "discipline", "practice", "train", "master",
    "change", "transform", "innovate", "pioneer", "explore", "discover",
    "resilience", "determination", "perseverance", "tenacity", "grit",
}

# ─── GRAMMAR PATTERNS ────────────────────────────────────────
# Simple heuristic POS detection (no spaCy dependency required)

NOUN_SUFFIXES = [
    "tion", "sion", "ment", "ness", "ity", "ence", "ance", "ism",
    "ist", "ure", "dom", "ship", "hood", "ology", "phy",
]

VERB_SUFFIXES = [
    "ate", "ize", "ise", "ify", "en",
]

ADJ_SUFFIXES = [
    "ous", "ive", "ful", "less", "able", "ible", "ent", "ant",
    "ial", "ical", "ic",
]

ABSTRACT_MARKERS = [
    "meta", "self", "inter", "trans", "super", "hyper",
]


@dataclass
class WordClassification:
    """Complete classification for a vocabulary word."""
    word: str
    channel: str       # "mind" | "heart" | "body" | "action"
    stage: str         # "hero" | "outlaw" | "edge_lord" | "best_self"
    symbol: str        # "stone" | "spark" | "prism" | "void" | "star"
    symbol_icon: str   # ◆ ◇ △ ○ ☆
    rarity: str        # "common" | "uncommon" | "rare" | "legendary"
    confidence: float  # 0.0-1.0 classification confidence
    reasoning: str     # Human-readable explanation


def classify_channel(word: str, context: str = "") -> tuple[str, float, str]:
    """
    Classify a word into a Channel based on semantic proximity.
    Returns (channel, confidence, reasoning).
    """
    word_lower = word.lower()
    text = f"{word_lower} {context.lower()}"
    tokens = set(re.findall(r'\w+', text))

    scores = {
        "mind": len(tokens & MIND_SEEDS),
        "heart": len(tokens & HEART_SEEDS),
        "body": len(tokens & BODY_SEEDS),
        "action": len(tokens & ACTION_SEEDS),
    }

    # Direct match bonus
    if word_lower in MIND_SEEDS:
        scores["mind"] += 5
    if word_lower in HEART_SEEDS:
        scores["heart"] += 5
    if word_lower in BODY_SEEDS:
        scores["body"] += 5
    if word_lower in ACTION_SEEDS:
        scores["action"] += 5

    total = sum(scores.values())
    if total == 0:
        return "body", 0.25, "No semantic match — defaulting to Body (grounding)"

    best = max(scores, key=scores.get)
    confidence = scores[best] / max(total, 1)

    return best, min(confidence, 1.0), f"Semantic proximity: {scores}"


def classify_stage(
    word: str, syllable_count: int = 0, frequency_rank: int = 0
) -> tuple[str, str]:
    """
    Classify a word's Stage based on complexity indicators.
    Returns (stage, reasoning).
    """
    word_lower = word.lower()

    # Estimate syllable count if not provided
    if syllable_count == 0:
        vowels = len(re.findall(r'[aeiouy]+', word_lower))
        syllable_count = max(vowels, 1)

    # Check for abstract/meta markers
    has_abstract = any(word_lower.startswith(m) for m in ABSTRACT_MARKERS)
    is_compound = "-" in word or len(word) > 12

    if has_abstract or is_compound:
        return "edge_lord", f"Abstract/compound marker detected"
    elif syllable_count >= 4:
        return "outlaw", f"{syllable_count} syllables — moderate complexity"
    elif syllable_count >= 5 or frequency_rank > 5000:
        return "best_self", f"High complexity or low frequency"
    else:
        return "hero", f"{syllable_count} syllables — foundational"


def classify_symbol(word: str) -> tuple[str, str, str]:
    """
    Classify a word's grammar Symbol using suffix heuristics.
    Returns (symbol_name, symbol_icon, reasoning).
    """
    word_lower = word.lower()

    # Check abstract first
    if any(word_lower.startswith(m) for m in ABSTRACT_MARKERS):
        return "void", "○", "Abstract prefix detected"

    # Check suffixes
    for suffix in VERB_SUFFIXES:
        if word_lower.endswith(suffix):
            return "spark", "◇", f"Verb suffix -{suffix}"

    for suffix in ADJ_SUFFIXES:
        if word_lower.endswith(suffix):
            return "prism", "△", f"Adjective suffix -{suffix}"

    for suffix in NOUN_SUFFIXES:
        if word_lower.endswith(suffix):
            return "stone", "◆", f"Noun suffix -{suffix}"

    # Default: check if it "feels" like a noun (most vocabulary words are)
    if word[0].isupper():
        return "star", "☆", "Capitalized — proper noun or key term"

    return "stone", "◆", "Default classification (noun-like)"


def classify_word(word: str, context: str = "") -> WordClassification:
    """
    Full classification pipeline for a single word.
    """
    channel, confidence, ch_reason = classify_channel(word, context)
    stage, st_reason = classify_stage(word)
    symbol, icon, sym_reason = classify_symbol(word)

    rarity_map = {
        "hero": "common",
        "outlaw": "uncommon",
        "edge_lord": "rare",
        "best_self": "legendary",
    }

    return WordClassification(
        word=word,
        channel=channel,
        stage=stage,
        symbol=symbol,
        symbol_icon=icon,
        rarity=rarity_map.get(stage, "common"),
        confidence=confidence,
        reasoning=f"Channel: {ch_reason} | Stage: {st_reason} | Symbol: {sym_reason}",
    )


def classify_word_list(
    words: list[str], contexts: dict[str, str] | None = None
) -> list[WordClassification]:
    """Classify a batch of words. Contexts is optional {word: context_sentence}."""
    contexts = contexts or {}
    return [classify_word(w, contexts.get(w, "")) for w in words]


# ─── CLI ──────────────────────────────────────────────────────

if __name__ == "__main__":
    # Demo: classify the Bias & Mirrors curriculum
    demo_words = [
        "Presence", "Bias", "Patience", "Resilience", "Clarity",
        # Extended test set
        "Metacognition", "Empathy", "Vulnerability", "Self-awareness",
        "Homeostasis", "Mitochondria", "Photosynthesis", "Ecosystem",
    ]

    print(f"\n{'═'*75}")
    print(f"  ARCANA CLASSIFIER — Word Analysis")
    print(f"{'═'*75}")

    for c in classify_word_list(demo_words):
        print(f"\n  {c.symbol_icon} {c.word}")
        print(f"    Channel: {c.channel.upper():8s}  Stage: {c.stage:12s}  "
              f"Rarity: {c.rarity:10s}  Conf: {c.confidence:.0%}")
        print(f"    {c.reasoning}")
