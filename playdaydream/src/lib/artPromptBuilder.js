// ════════════════════════════════════════════════════════════
// SCENE ART PROMPT BUILDER
// ════════════════════════════════════════════════════════════
// Ports the ARCANA prompt system from prompts.py to JavaScript.
// Builds cinematic scene prompts for LongCat-Image via ComfyUI.

const CHANNEL_STYLES = {
  mind: {
    colorPalette: 'emerald green, silver, deep teal, moonlight blue',
    elements: 'crystalline structures, geometric patterns, ancient libraries, floating sigils',
    lighting: 'cool ethereal glow, moonlight, phosphorescent, bioluminescent fog',
    moodBase: 'intellectual, mysterious, contemplative, vast',
    composition: 'grand architectural scale, infinite corridors, towering shelves',
  },
  heart: {
    colorPalette: 'warm orange, golden amber, deep crimson, sunset rose',
    elements: 'flame, phoenix feathers, warm hearths, blooming flowers, swirling embers',
    lighting: 'warm golden hour light, candlelight, ember glow, hearth-fire',
    moodBase: 'passionate, tender, courageous, intimate',
    composition: 'close human scale, warm enclosures, organic flowing shapes',
  },
  body: {
    colorPalette: 'deep blue, ocean teal, soft violet, moss green',
    elements: 'still water, ancient stones, mountain peaks, roots, moss, bark',
    lighting: 'soft ambient twilight, underwater caustics, gentle dawn, dappled forest light',
    moodBase: 'grounding, peaceful, intuitive, ancient',
    composition: 'natural landscape scale, organic curves, water and stone balance',
  },
  action: {
    colorPalette: 'rich gold, bronze, molten amber, forge orange',
    elements: 'forge, anvil, lightning, construction, gears, sparks, molten metal',
    lighting: 'dramatic rim lighting, sparks, forge-fire glow, high contrast chiaroscuro',
    moodBase: 'kinetic, powerful, determined, purposeful',
    composition: 'dynamic diagonal lines, motion blur, asymmetrical energy, heroic scale',
  },
};

const MOOD_ATMOSPHERE = {
  calm: 'serene, gentle mist, soft focus background, peaceful atmosphere, meditative stillness',
  mysterious: 'deep shadows, swirling fog, hidden depths, enigmatic atmosphere, veiled secrets',
  warm: 'golden sunlight, comfortable warmth, gentle radiance, hearth glow, welcoming',
  tense: 'dramatic clouds, strong wind, high contrast, electric atmosphere, impending storm',
  transcendent: 'above the clouds, infinite sky, divine light, cosmic scale, celestial wonder',
  neutral: 'balanced lighting, clear atmosphere, open space, natural clarity',
};

// A cinematic scene prompt — NOT a trading card. For adventure backgrounds.
export function buildScenePrompt({
  word,
  channel = 'body',
  mood = 'neutral',
  storyHint = '',
  themes = [],
}) {
  const ch = CHANNEL_STYLES[channel.toLowerCase()] || CHANNEL_STYLES.body;
  const atm = MOOD_ATMOSPHERE[mood.toLowerCase()] || MOOD_ATMOSPHERE.neutral;
  const themeStr = themes.length ? themes.join(', ') : 'universal themes';

  const scene = storyHint
    ? `${storyHint}. The concept of ${word} is woven into every element.`
    : `A cinematic scene embodying the concept of ${word}, evoking ${themeStr}.`;

  const prompt = (
    `${scene} ` +
    `Visual style: highly detailed digital matte painting, cinematic composition, rule of thirds. ` +
    `Elements: ${ch.elements}. ` +
    `Color palette: ${ch.colorPalette}. ` +
    `Lighting: ${ch.lighting}. ` +
    `Atmosphere: ${atm}. ` +
    `Composition: ${ch.composition}. ` +
    `Mood: ${ch.moodBase}. ` +
    `8K render, sharp focus, rich detail, dramatic depth of field.`
  );

  const negativePrompt = (
    'blurry, deformed, low quality, watermark, signature, text, letters, words, ' +
    'misspelled text, extra fingers, oversaturated, plastic texture, ' +
    'generic stock photo, cartoon, anime, amateur, amateurish, low resolution, ' +
    'busy composition, cluttered, messy, flat lighting'
  );

  return { prompt, negativePrompt };
}

// Build a ComfyUI LongCat-Image workflow JSON (matches comfyui_client.py)
export function buildLongcatWorkflow({
  prompt,
  negativePrompt = '',
  width = 1024,
  height = 1024,
  steps = 25,
  cfg = 4.5,
  seed = -1,
  modelName = 'LongCat-Image',
  savePrefix = 'daydream',
}) {
  const finalSeed = seed < 0 ? Math.floor(Math.random() * 2 ** 32) : seed;

  return {
    prompt: {
      '1': {
        class_type: 'LongCatImageModelLoader',
        inputs: {
          model_path: modelName,
          dtype: 'bfloat16',
        },
      },
      '2': {
        class_type: 'LongCatImageTextToImage',
        inputs: {
          longcat_pipeline: ['1', 0],
          prompt,
          negative_prompt: negativePrompt,
          width,
          height,
          steps,
          guidance_scale: cfg,
          seed: finalSeed,
          cpu_offload: 'true',
          sage_attn: 'false',
        },
      },
      '3': {
        class_type: 'SaveImage',
        inputs: {
          images: ['2', 0],
          filename_prefix: savePrefix,
        },
      },
    },
  };
}

// Auto-extract a scene description from Mad Libs story text
export function extractSceneFromStory(storyText) {
  if (!storyText) return '';
  // Remove bracketed template variables, keep the rest as a scene description
  const cleaned = storyText
    .replace(/\[.*?\]/g, '')
    .replace(/\s+/g, ' ')
    .trim();
  return cleaned;
}

// Derive mood from story text heuristics
export function deriveMood(storyText) {
  if (!storyText) return 'neutral';
  const t = storyText.toLowerCase();
  if (t.includes('storm') || t.includes('fear') || t.includes('danger') || t.includes('dark')) return 'tense';
  if (t.includes('calm') || t.includes('peace') || t.includes('still') || t.includes('quiet')) return 'calm';
  if (t.includes('warm') || t.includes('love') || t.includes('heart') || t.includes('home')) return 'warm';
  if (t.includes('mystery') || t.includes('shadow') || t.includes('unknown') || t.includes('fog')) return 'mysterious';
  if (t.includes('sky') || t.includes('above') || t.includes('infinite') || t.includes('light')) return 'transcendent';
  return 'neutral';
}

// Full pipeline: story text → prompt → workflow
export function buildSceneFromNode(node) {
  const storyHint = extractSceneFromStory(node.story);
  const mood = deriveMood(node.story);
  const { prompt, negativePrompt } = buildScenePrompt({
    word: node.focusWord || 'The Unknown',
    channel: node.channel || 'body',
    mood,
    storyHint,
    themes: node.themes || [],
  });

  const workflow = buildLongcatWorkflow({
    prompt,
    negativePrompt,
    width: 1024,
    height: 1024,
    steps: 25,
    cfg: 4.5,
    savePrefix: `daydream_${node.focusWord?.toLowerCase() || 'scene'}_${Date.now()}`,
  });

  return { prompt, negativePrompt, workflow };
}
