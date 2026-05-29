// ════════════════════════════════════════════════════════════
// DAYDREAM — The Great Game: Adventure Data
// ════════════════════════════════════════════════════════════
//
// These are dungeons. Each node is a room. Each word is a spell.
// The player's path IS the combat log. The choices ARE the skill tree.
//
// No therapy. No self-reflection homework. Just game.

export { Channel, CHANNEL_COLORS, CHANNEL_QUESTIONS, Mastery } from './constants.js';
import { Channel } from './constants.js';

// ════════════════════════════════════════════════════════════
// ADVENTURE 1: THE HALL OF A THOUSAND FACES
// ════════════════════════════════════════════════════════════
// A dungeon tower where mirrors spawn Reflection enemies.
// Cast spells. Loot shards. Survive.

const HALL_OF_FACES = {
  id: 'hall-of-faces',
  title: 'The Hall of a Thousand Faces',
  description: 'A cursed tower where every reflection is an enemy. Cast your Word-Spells. Loot Mirror Shards. Reach the Apex or become another face on the wall.',
  ageRange: '10+',
  difficulty: 'Apprentice',
  rewards: ['Mirror Shard x3', 'Word Sigil: Clarity'],
  start: 'foyer',
  nodes: {
    foyer: {
      id: 'foyer',
      title: 'The Foyer of Lies',
      focusWord: 'Presence',
      channel: Channel.BODY,
      story: 'You push through oak doors into a hall of mirrors. Every surface reflects — but none show the room behind you. Only faces. Hundreds of faces. Yours is among them. A silver voice echoes: "To enter, you must first be seen." A Wisp of Doubt materializes from a cracked mirror. Combat begins.',
      image: '/images/threshold.png',
      depth: 'Combat Tip: Presence is your Perception stat. At 2+, you detect hidden exits. At 4+, you see the Wisp\'s weakness before it attacks. How many exits do you see?',
      droneHz: 136.1,
      loot: ['Cracked Mirror Shard'],
      exits: {
        up:    { label: 'Dash toward the spiral staircase', to: 'glass-forest', virtue: 'initiative' },
        right: { label: 'Shatter the nearest mirror — force a new path', to: 'quiet-chamber', virtue: 'resilience' },
        down:  null,
        left:  null,
      },
    },
    'glass-forest': {
      id: 'glass-forest',
      title: 'The Glass Forest',
      focusWord: 'Bias',
      channel: Channel.MIND,
      story: 'Mirrors stand like trees in a twisted grove. Each shows a different version of you — taller, crueler, more afraid. The reflections step OUT of the glass. Three Mirror Selves block the path. They use your own spells against you. "We are what you believe," they say in unison.',
      image: '/images/forest.png',
      depth: 'Combat Tip: Bias is your Spell Resistance. The Mirror Selves copy your highest stat. If Mind is your strongest channel, they cast illusions. Switch to a Body or Action spell to break their pattern. What spell do you prepare?',
      droneHz: 174.6,
      loot: ['Glass Fang', 'Wisp Essence'],
      exits: {
        up:    { label: 'Cast CLARITY — reveal which reflection is the original', to: 'apex', virtue: 'clarity' },
        right: { label: 'Cast RESILIENCE — tank their combined assault and push through', to: 'stone-bridge', virtue: 'resilience' },
        left:  { label: 'Cast PATIENCE — wait for them to argue among themselves', to: 'quiet-chamber', virtue: 'patience' },
        down:  { label: 'Cast CURIOSITY — ask them which of you is real. Trick them.', to: 'stone-bridge', virtue: 'curiosity' },
      },
    },
    'quiet-chamber': {
      id: 'quiet-chamber',
      title: 'The Quiet Chamber',
      focusWord: 'Patience',
      channel: Channel.BODY,
      story: 'A hidden room behind the shattered mirror. No enemies here — just a pedestal with a glowing Word Sigil and a mechanical door with three rotating dials. The dials click softly. A plaque reads: "The patient see what haste ignores." The door is a puzzle lock. Wrong answers drain 10 HP.',
      image: '/images/garden.png',
      depth: 'Puzzle Tip: Patience is your Puzzle-solving stat. At 2+, you get one hint. At 4+, the dials slow down so you can read the symbols. The plaque hints at the sequence: dawn, noon, dusk. Which order opens the door?',
      droneHz: 196.0,
      loot: ['Word Sigil: Patience', 'Healing Draught'],
      exits: {
        up:    { label: 'Solve the dial puzzle — dawn, noon, dusk', to: 'apex', virtue: 'clarity' },
        right: { label: 'Smash the mechanism — brute force the lock', to: 'stone-bridge', virtue: 'resilience' },
        down:  { label: 'Rest and study the sigil first', to: 'quiet-chamber', virtue: 'patience' },
        left:  null,
      },
    },
    'stone-bridge': {
      id: 'stone-bridge',
      title: 'The Falling Bridge',
      focusWord: 'Resilience',
      channel: Channel.ACTION,
      story: 'A stone bridge spans a chasm of broken mirrors below. Halfway across, the bridge CRACKS. A Shard Golem climbs from the depths — a construct of shattered glass with fists like daggers. "Fall," it commands. The bridge shakes. Behind you, the entrance collapses. Forward is the only way.',
      image: '/images/canyon.png',
      depth: 'Combat Tip: Resilience is your HP and Armor. At 3+, you can absorb one blow without staggering. At 5+, the bridge breaking actually gives you debris to throw as improvised weapons. How do you use the falling bridge against the Golem?',
      droneHz: 220.0,
      loot: ['Golem Core', 'Reinforced Mirror Shard'],
      exits: {
        up:    { label: 'Leap across the gap — channel Resilience to stick the landing', to: 'apex', virtue: 'resilience' },
        right: { label: 'Grab falling debris and throw it at the Golem', to: 'apex', virtue: 'initiative' },
        down:  { label: 'Let the bridge fall and ride it down like a ramp', to: 'apex', virtue: 'courage' },
        left:  { label: 'Cast PRESENCE — find the Golem\'s blind spot', to: 'quiet-chamber', virtue: 'presence' },
      },
    },
    apex: {
      id: 'apex',
      title: 'The Apex Mirror',
      focusWord: 'Clarity',
      channel: Channel.MIND,
      story: 'The top of the tower. One mirror remains — massive, flawless, humming with power. Your reflection does not move when you do. It smiles. "You have climbed through yourself," it says. "Now claim what you earned." The mirror shatters outward — not inward — and a Word Sigil floats where the glass was. You feel the word settle into your mind like a key into a lock.',
      image: '/images/summit.png',
      depth: 'Reward: CLARITY permanently added to your Spellbook. This word now grants +2 to Mind checks in all future dungeons. Your character sheet updates. Total Words Owned: 1. Return to the Threshold to select your next dungeon.',
      droneHz: 261.6,
      loot: ['Word Sigil: Clarity', 'Mirror Crown', 'XP: +500'],
      exits: { up: null, down: null, left: null, right: null },
    },
  },
};

// ════════════════════════════════════════════════════════════
// ADVENTURE 2: THE EMBER WARRENS
// ════════════════════════════════════════════════════════════
// Goblin-infested caverns beneath the Rust Mountains.
// Stealth, combat, traps. The goblins are planning a raid.
// Stop them or die trying.

const EMBER_WARRENS = {
  id: 'ember-warns',
  title: 'The Ember Warrens',
  description: 'Goblin tunnels beneath the Rust Mountains. They\'re massing for a surface raid. Infiltrate. Disrupt. Escape with your hide intact.',
  ageRange: '10+',
  difficulty: 'Apprentice',
  rewards: ['Goblin Ear x5', 'Word Sigil: Forge'],
  start: 'tunnel-mouth',
  nodes: {
    'tunnel-mouth': {
      id: 'tunnel-mouth',
      title: 'The Tunnel Mouth',
      focusWord: 'Presence',
      channel: Channel.BODY,
      story: 'A crack in the mountain drips orange light. Goblin chatter echoes from within. Two guards argue over a roasted rat. They haven\'t seen you yet. The tunnel splits ahead — left smells like cooking fire, right smells like iron and oil.',
      image: '/images/threshold.png',
      depth: 'Stealth Tip: Presence is your Sneak stat. At 2+, you bypass one guard. At 4+, you hear the password they\'re arguing about. At 1 or below, they spot you immediately. What\'s your approach?',
      droneHz: 136.1,
      loot: ['Goblin Dagger (rusty)'],
      exits: {
        up:    { label: 'Sneak past — stick to the shadows', to: 'cookfire', virtue: 'presence' },
        right: { label: 'Charge in — take them by surprise', to: 'forge-chamber', virtue: 'courage' },
        down:  null,
        left:  null,
      },
    },
    cookfire: {
      id: 'cookfire',
      title: 'The Cookfire Cavern',
      focusWord: 'Joy',
      channel: Channel.HEART,
      story: 'A dozen goblins crowd around a fire pit, passing a wineskin and singing a song — badly. A map is pinned to the wall with a knife. It shows the surface village and attack routes. A goblin chief, fatter than the rest, holds the key to the deeper tunnels around his neck.',
      image: '/images/garden.png',
      depth: 'Social Combat: Joy is your Charisma stat. At 2+, you can blend in and listen. At 4+, you can join their song and earn trust. At 5+, you convince them you\'re a visiting chieftain from another warren. How do you get that key?',
      droneHz: 174.6,
      loot: ['Warren Key', 'Surface Raid Map'],
      exits: {
        up:    { label: 'Pick the chief\'s pocket while they sing', to: 'deep-warren', virtue: 'cunning' },
        right: { label: 'Join the song — earn their trust', to: 'deep-warren', virtue: 'joy' },
        left:  { label: 'Steal the map and slip away', to: 'forge-chamber', virtue: 'curiosity' },
        down:  null,
      },
    },
    'forge-chamber': {
      id: 'forge-chamber',
      title: 'The Forge Chamber',
      focusWord: 'Forge',
      channel: Channel.ACTION,
      story: 'Heat blasts from a crude forge. Three goblin smiths hammer blades while a shaman chants over molten metal. They\'re arming for war. Crates of weapons stack against the walls — enough to equip a small army. The shaman sees you. "Surface rat," he hisses. The smiths grab hammers.',
      image: '/images/forest.png',
      depth: 'Combat Tip: Forge is your Craft/Destruction stat. At 2+, you can sabotage the forge to explode. At 4+, you craft an improvised weapon from their own materials. At 5+, you intimidate them by showing you know their forge better than they do. What do you do?',
      droneHz: 196.0,
      loot: ['Goblin-forged Blade', 'Shaman\'s Fire Dust'],
      exits: {
        up:    { label: 'Overturn the forge — molten metal floods the chamber', to: 'deep-warren', virtue: 'forge' },
        right: { label: 'Grab a fresh blade and fight your way through', to: 'deep-warren', virtue: 'courage' },
        down:  { label: 'Cast PRESENCE — find the ventilation shaft', to: 'cookfire', virtue: 'presence' },
        left:  null,
      },
    },
    'deep-warren': {
      id: 'deep-warren',
      title: 'The Deep Warren',
      focusWord: 'Trust',
      channel: Channel.HEART,
      story: 'The deepest cavern. A goblin prisoner — not a goblin at all, but a surface scout captured days ago — hangs in a cage. "They\'re moving tonight," she whispers. "The whole warren. Hundreds." She can show you the escape tunnel if you free her. But she might be a trap. The shaman\'s chant grows louder behind you.',
      image: '/images/canyon.png',
      depth: 'Dilemma: Trust is your Ally/Loyalty stat. Free her and she opens the escape tunnel. Leave her and you must fight through the entire warren alone. At 3+ Trust, she also gives you a poisoned blade. At 1 Trust, she stabs you as soon as the cage opens. What\'s your call?',
      droneHz: 220.0,
      loot: ['Poisoned Goblin Blade', 'Escape Route Map'],
      exits: {
        up:    { label: 'Free her — we escape together', to: 'surface', virtue: 'trust' },
        right: { label: 'Leave her — I move faster alone', to: 'surface', virtue: 'caution' },
        down:  { label: 'Interrogate her first — verify her story', to: 'cookfire', virtue: 'clarity' },
        left:  null,
      },
    },
    surface: {
      id: 'surface',
      title: 'The Surface — Dawn Break',
      focusWord: 'Courage',
      channel: Channel.ACTION,
      story: 'You burst from the tunnel into cold mountain air. Behind you, the warren collapses — either from the forge explosion or from your own sabotage. The village below still sleeps, unaware. You hold the raid map. You hold a goblin blade. You hold the word FORGE, hot as the forge itself, now part of your vocabulary.',
      image: '/images/summit.png',
      depth: 'Reward: FORGE permanently added to your Spellbook. +3 to Action checks. +1 to Crafting rolls. You can now arm yourself in any dungeon. Total Words Owned: 2. Your character level increases. Return to the Threshold when ready.',
      droneHz: 261.6,
      loot: ['Word Sigil: Forge', 'Goblin Raid Map', 'XP: +750'],
      exits: { up: null, down: null, left: null, right: null },
    },
  },
};

// ════════════════════════════════════════════════════════════
// ADVENTURE REGISTRY
// ════════════════════════════════════════════════════════════

export const ADVENTURES = {
  [HALL_OF_FACES.id]: HALL_OF_FACES,
  [EMBER_WARRENS.id]: EMBER_WARRENS,
};

// ─── HELPERS ─────────────────────────────────────────────
export function getAdventure(id) {
  return ADVENTURES[id] || null;
}

export function getDefaultAdventure() {
  return ADVENTURES['hall-of-faces'];
}

export function getAllAdventures() {
  return Object.values(ADVENTURES);
}
