#!/usr/bin/env python3
"""
ARCANA Prompt Builder — Generates LongCat-Image prompts for power word cards.

Each word-spell gets a prompt tailored to its Channel (Mind/Heart/Body/Action)
and its mood. The prompt is structured for LongCat's strengths:
  - Qwen2.5-VL text encoder (rich semantic understanding)
  - Character-level text rendering (use "quotes" for in-image text)
  - Reward-model trained (natural textures, no plastic look)

Usage:
    from prompts import build_card_prompt
    prompt = build_card_prompt("Resilience", "action", "tense", 
                                ["courage", "persistence"])
"""

# Channel → visual style mapping
CHANNEL_STYLES = {
    "mind": {
        "color_palette": "emerald green, silver, deep teal",
        "element": "crystalline structures, geometric patterns, ancient libraries",
        "lighting": "cool ethereal glow, moonlight, phosphorescent",
        "mood_base": "intellectual, mysterious, contemplative",
        "border_desc": "emerald green ornate border with runic inscriptions",
    },
    "heart": {
        "color_palette": "warm orange, golden amber, deep crimson",
        "element": "flame, phoenix feathers, warm hearths, blooming flowers",
        "lighting": "warm golden hour light, candlelight, ember glow",
        "mood_base": "passionate, tender, courageous",
        "border_desc": "warm orange ornate border with flowing flame motifs",
    },
    "body": {
        "color_palette": "deep blue, ocean teal, soft violet",
        "element": "still water, ancient stones, mountain peaks, roots",
        "lighting": "soft ambient twilight, underwater caustics, gentle dawn",
        "mood_base": "grounding, peaceful, intuitive",
        "border_desc": "deep blue ornate border with water and stone patterns",
    },
    "action": {
        "color_palette": "rich gold, bronze, molten amber",
        "element": "forge, anvil, lightning, construction, gears",
        "lighting": "dramatic rim lighting, sparks, forge-fire glow",
        "mood_base": "kinetic, powerful, determined",
        "border_desc": "golden ornate border with forge-hammer motifs",
    },
}

# Mood → atmospheric overlay
MOOD_ATMOSPHERE = {
    "calm": "serene, gentle mist, soft focus background, peaceful atmosphere",
    "mysterious": "deep shadows, swirling fog, hidden depths, enigmatic atmosphere",
    "warm": "golden sunlight, comfortable warmth, gentle radiance",
    "tense": "dramatic clouds, strong wind, high contrast, electric atmosphere",
    "transcendent": "above the clouds, infinite sky, divine light, cosmic scale",
    "neutral": "balanced lighting, clear atmosphere, open space",
}

# Mastery level → art style variation
MASTERY_STYLES = {
    "encountered": {
        "style": "pencil sketch, charcoal drawing, partially revealed",
        "detail": "minimal detail, mysterious silhouette, rough edges",
        "suffix": "sketch art, monochrome with hints of color, unfinished",
    },
    "experienced": {
        "style": "watercolor painting, emerging colors, semi-transparent",
        "detail": "moderate detail, colors bleeding at edges, dreamy",
        "suffix": "watercolor art, soft edges, emerging vibrancy",
    },
    "owned": {
        "style": "detailed digital painting, rich saturated colors",
        "detail": "intricate detail, sharp focus, fully realized",
        "suffix": "professional digital art, TCG card illustration, sharp focus",
    },
    "mastered": {
        "style": "legendary illustration, gold leaf accents, divine light",
        "detail": "extraordinary detail, particle effects, luminous aura",
        "suffix": "legendary TCG card art, gold frame, epic composition, masterwork",
    },
}


def build_card_prompt(
    word: str,
    channel: str,
    mood: str,
    themes: list[str],
    mastery: str = "owned",
    story_hint: str = "",
) -> dict[str, str]:
    """
    Build a LongCat-Image prompt for an ARCANA power word card.

    Args:
        word: The vocabulary word (e.g., "Resilience")
        channel: One of "mind", "heart", "body", "action"
        mood: The emotional atmosphere (e.g., "tense", "calm")
        themes: Thematic tags (e.g., ["courage", "persistence"])
        mastery: Mastery level for art style variation
        story_hint: Optional story text to inspire the scene

    Returns:
        dict with "prompt" and "negative_prompt" keys
    """
    ch = CHANNEL_STYLES.get(channel.lower(), CHANNEL_STYLES["body"])
    atm = MOOD_ATMOSPHERE.get(mood.lower(), MOOD_ATMOSPHERE["neutral"])
    ms = MASTERY_STYLES.get(mastery.lower(), MASTERY_STYLES["owned"])

    # Theme keywords
    theme_str = ", ".join(themes) if themes else "universal"

    # Scene description from story hint or generated
    scene = story_hint if story_hint else f"A symbolic scene evoking {theme_str}"

    prompt = (
        f'A {ch["border_desc"]} trading card game illustration. '
        f'{scene}. '
        f'The word "{word.upper()}" is rendered in stylized text, '
        f"integrated naturally into the composition. "
        f'Visual elements: {ch["element"]}. '
        f'Color palette: {ch["color_palette"]}. '
        f'Lighting: {ch["lighting"]}. '
        f"Atmosphere: {atm}. "
        f'{ms["style"]}, {ms["detail"]}. '
        f'{ms["suffix"]}, 1024x1024'
    )

    negative_prompt = (
        "blurry, deformed, low quality, watermark, signature, "
        "text artifacts, misspelled text, extra fingers, "
        "oversaturated, plastic texture, generic stock photo"
    )

    return {
        "prompt": prompt,
        "negative_prompt": negative_prompt,
    }


def build_all_prompts(curriculum: dict, mastery: str = "owned") -> list[dict]:
    """
    Build prompts for every word in a curriculum definition.

    Args:
        curriculum: Dict with "words" list (each word has word, channel,
                    mood, themes, story_text fields)
        mastery: Mastery level for all cards (or override per word)

    Returns:
        List of dicts with word name + prompt + negative_prompt
    """
    results = []
    for word_def in curriculum.get("words", []):
        prompts = build_card_prompt(
            word=word_def["word"],
            channel=word_def.get("channel", "body"),
            mood=word_def.get("mood", "neutral"),
            themes=word_def.get("themes", []),
            mastery=mastery,
            story_hint=word_def.get("story_text", ""),
        )
        results.append({
            "word": word_def["word"],
            "channel": word_def.get("channel", "body"),
            **prompts,
        })
    return results


if __name__ == "__main__":
    # Demo: generate prompts for the Bias & Mirrors curriculum
    demo = {
        "words": [
            {
                "word": "Presence",
                "channel": "body",
                "mood": "calm",
                "themes": ["awareness", "grounding"],
                "story_text": "A still stone archway with warm light spilling through, "
                              "ancient and grounding. A single figure stands in silhouette",
            },
            {
                "word": "Bias",
                "channel": "mind",
                "mood": "mysterious",
                "themes": ["self-awareness", "critical-thinking"],
                "story_text": "Dark glass trees reflecting warped, distorted versions "
                              "of the viewer. Fractured mirrors floating in fog",
            },
            {
                "word": "Patience",
                "channel": "body",
                "mood": "warm",
                "themes": ["growth", "stillness"],
                "story_text": "A walled garden at golden hour, fountain at center, "
                              "birdsong implied by scattered feathers on the water",
            },
            {
                "word": "Resilience",
                "channel": "action",
                "mood": "tense",
                "themes": ["courage", "persistence"],
                "story_text": "A narrow bridge over a vast canyon, wind visible as "
                              "golden streaks. A single figure mid-step, leaning into the gale",
            },
            {
                "word": "Clarity",
                "channel": "mind",
                "mood": "transcendent",
                "themes": ["wisdom", "resolution"],
                "story_text": "Above the clouds, the world visible below as a vast map. "
                              "Crystal-clear atmosphere, infinite visibility",
            },
        ]
    }

    all_prompts = build_all_prompts(demo)
    for p in all_prompts:
        print(f"\n{'='*60}")
        print(f"WORD: {p['word']} ({p['channel'].upper()})")
        print(f"{'='*60}")
        print(f"PROMPT:\n{p['prompt']}")
        print(f"\nNEGATIVE:\n{p['negative_prompt']}")
