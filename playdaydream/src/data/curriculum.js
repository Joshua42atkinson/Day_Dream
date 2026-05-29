// ════════════════════════════════════════════════════════════
// DAYDREAM — The Great Game: Adventure Data
// ════════════════════════════════════════════════════════════
//
// These are dungeons. Each node is a room. Each word is a spell.
// The player's path IS the combat log. The choices ARE the skill tree.
//
// RULE: No vague feelings. No "what does it mean?" Every depth
// field must contain a NUMBER. HP. DC. Damage dice. Stat bonus.

export { Channel, CHANNEL_COLORS, CHANNEL_QUESTIONS, Mastery } from './constants.js';
import { Channel } from './constants.js';

// ════════════════════════════════════════════════════════════
// ADVENTURE 1: THE STAT-CHECK GAUNTLET
// ════════════════════════════════════════════════════════════
// A proving ground. 4 floors. Each floor tests one stat.
// Pass the DC or lose HP. Beat all 4 to level up.
// Starting HP: 20. If you hit 0, you respawn at the entrance.

const STAT_GAUNTLET = {
  id: 'stat-gauntlet',
  title: 'The Stat-Check Gauntlet',
  description: '4 floors. 4 stats. Pass the DC or take damage. Beat floor 4 to gain +1 to any channel and unlock your first Word-Spell.',
  ageRange: '8+',
  difficulty: 'Tutorial',
  rewards: ['+1 Channel Point', 'Word Unlock: Presence', 'XP: +100'],
  start: 'entrance',
  nodes: {
    entrance: {
      id: 'entrance',
      title: 'Floor 0: The Entrance',
      focusWord: 'None',
      channel: Channel.BODY,
      story: 'You stand at the base of a stone tower. A sign reads: "Pass all 4 floors to prove your stats. Die and respawn. No shame." Your character sheet shows: HP 20/20, Mind 0, Heart 0, Body 0, Action 0. You have 0 Words. A glowing door opens to Floor 1.',
      image: '/images/threshold.png',
      depth: 'TUTORIAL: Your 4 stats are Mind (spell accuracy), Heart (social/persuasion), Body (HP + defense), Action (speed + damage). Every character starts at 0 in all stats. Complete dungeons to increase them. Choose a door.',
      droneHz: 136.1,
      loot: ['Potion of Minor Healing (restores 5 HP)'],
      exits: {
        up:    { label: 'Enter Floor 1: The Mind Chamber (DC 8)', to: 'mind-floor', virtue: 'initiative' },
        right: { label: 'Enter Floor 2: The Heart Chamber (DC 8)', to: 'heart-floor', virtue: 'courage' },
        down:  null,
        left:  null,
      },
    },
    'mind-floor': {
      id: 'mind-floor',
      title: 'Floor 1: The Mind Chamber',
      focusWord: 'Clarity',
      channel: Channel.MIND,
      story: 'A room of shifting tiles. A riddle appears on the wall: "I have cities, but no houses. I have mountains, but no trees. I have water, but no fish." A 10-second timer begins. A dart trap clicks above your head. DC 8 Mind check to solve before the darts fire.',
      image: '/images/forest.png',
      depth: 'MECHANIC: Mind Check. Roll 1d6 + Mind stat. Need 8+ to pass. Current Mind: 0. You need to roll 8+ on a d6. That\'s impossible. But wait — you can spend your Healing Potion to gain +2 Mind for this check. Or you can take the darts (lose 3 HP) and brute-force the door. Your call.',
      droneHz: 174.6,
      loot: ['Map Fragment: Floor 2 location'],
      exits: {
        up:    { label: 'Roll Mind Check (1d6+0). Need 8+. If fail: -3 HP', to: 'body-floor', virtue: 'clarity' },
        right: { label: 'Drink Potion (+2 Mind for this check). Roll 1d6+2. Need 8+', to: 'body-floor', virtue: 'clarity' },
        left:  { label: 'Take the darts (-3 HP) and kick the door down', to: 'body-floor', virtue: 'resilience' },
        down:  null,
      },
    },
    'heart-floor': {
      id: 'heart-floor',
      title: 'Floor 2: The Heart Chamber',
      focusWord: 'Trust',
      channel: Channel.HEART,
      story: 'A goblin merchant blocks the exit. He holds a lever. "Pay 5 gold or I drop the gate and you start over." You have 0 gold. But you notice his armor is rusty. He\'s nervous. DC 8 Heart check to intimidate, persuade, or bluff.',
      image: '/images/garden.png',
      depth: 'MECHANIC: Heart Check. Roll 1d6 + Heart stat. Need 8+. Current Heart: 0. You cannot pass without help. But look at your inventory — you have a Map Fragment from Floor 1. You can bluff: "I already mapped this whole tower. I know where your stash is." That gives +3 to the Heart check. Roll 1d6+3. Need 8+.',
      droneHz: 196.0,
      loot: ['Goblin Merchant\'s Key'],
      exits: {
        up:    { label: 'Intimidate: "I know where your stash is." Roll 1d6+3. Need 8+', to: 'body-floor', virtue: 'trust' },
        right: { label: 'Persuade: "I\'ll bring you real gold from the top." Roll 1d6+0. Need 8+ (50% fail)', to: 'body-floor', virtue: 'joy' },
        down:  { label: 'Fight him. No check. You win but lose 5 HP.', to: 'body-floor', virtue: 'resilience' },
        left:  null,
      },
    },
    'body-floor': {
      id: 'body-floor',
      title: 'Floor 3: The Body Chamber',
      focusWord: 'Resilience',
      channel: Channel.BODY,
      story: 'A spiked pit spans the room. Rotating blades swing from the ceiling. On the far side, a chest glows with the Word Sigil for PRESENCE. The platform sequence is visible: press the plates in order or take damage. DC 10 Body check to time your jumps perfectly.',
      image: '/images/canyon.png',
      depth: 'MECHANIC: Body Check = HP + Defense. Roll 1d6 + Body stat. Need 10+. Current Body: 0. Impossible without taking damage. BUT: every 1 HP you spend gives +1 to the roll. You can spend 4 HP to roll 1d6+4. That gives you a 50% chance. Or spend 6 HP for 1d6+6 = 83% chance. The chest contains PRESENCE. Worth it?',
      droneHz: 220.0,
      loot: ['Word Sigil: Presence', 'Potion of Minor Healing'],
      exits: {
        up:    { label: 'Spend 4 HP. Roll 1d6+4. Need 10+ (50% pass)', to: 'action-floor', virtue: 'resilience' },
        right: { label: 'Spend 6 HP. Roll 1d6+6. Need 10+ (83% pass)', to: 'action-floor', virtue: 'resilience' },
        down:  { label: 'Spend 0 HP. Roll 1d6+0. Need 10+ (17% pass, but if you fail you lose ANOTHER 4 HP)', to: 'action-floor', virtue: 'patience' },
        left:  null,
      },
    },
    'action-floor': {
      id: 'action-floor',
      title: 'Floor 4: The Action Chamber',
      focusWord: 'Initiative',
      channel: Channel.ACTION,
      story: 'The final floor. A training dummy hangs from chains. It has HP: 10. You must deal 10 damage in 3 rounds or the floor floods with gas and you respawn. Action stat = damage bonus per hit. Each round, roll 1d6 + Action. Current Action: 0. You deal 1d6 damage per round. Average 3.5 × 3 = 10.5. It\'s tight.',
      image: '/images/summit.png',
      depth: 'FINAL CHECK: Action = Speed + Damage. You have 3 rounds. Each round: roll 1d6 + Action (currently 0). Total must reach 10. If you have PRESENCE from Floor 3, you get one re-roll. If you have the Goblin Key from Floor 2, the dummy is already cracked (-2 HP). Track your rolls. Total ≥ 10 = WIN. < 10 = respawn at entrance, keep loot.',
      droneHz: 261.6,
      loot: ['+1 Channel Point (apply to any stat)', 'XP: +100', 'Word Unlock: Presence'],
      exits: {
        up:    { label: 'Round 1: Roll 1d6+0. Round 2: Roll 1d6+0. Round 3: Roll 1d6+0. Total must be 10+', to: 'victory', virtue: 'initiative' },
        right: { label: 'If you have PRESENCE: re-roll your lowest round once', to: 'victory', virtue: 'clarity' },
        down:  { label: 'If you have Goblin Key: dummy starts at 8 HP (needs 8 damage, not 10)', to: 'victory', virtue: 'cunning' },
        left:  null,
      },
    },
    victory: {
      id: 'victory',
      title: 'VICTORY: Gauntlet Cleared',
      focusWord: 'Level Up',
      channel: Channel.MIND,
      story: 'The tower shakes. Stone grinds against stone. A pedestal rises from the floor. On it: a glowing +1 token and the Word Crystal for PRESENCE. You feel the word settle into your mind like a loaded weapon. Your character sheet updates: +1 to any channel. Presence is now CASTABLE. XP: +100. Level: 1.',
      image: '/images/summit.png',
      depth: 'REWARDS: +1 Channel Point (add to Mind, Heart, Body, or Action). Word: PRESENCE added to Spellbook. Presence effect: +2 Perception, detect hidden exits, one re-roll per dungeon. XP: +100. You are now Level 1. Next dungeon unlocks: The Word-Crystal Mine. Return to Threshold.',
      droneHz: 261.6,
      loot: ['+1 Channel Point', 'Word Crystal: Presence', 'XP: +100'],
      exits: { up: null, down: null, left: null, right: null },
    },
  },
};

// ════════════════════════════════════════════════════════════
// ADVENTURE 2: THE WORD-CRYSTAL MINE
// ════════════════════════════════════════════════════════════
// Goblins stole Word-Crystals. 5 rooms. Combat, traps, loot.
// Enemies have HP. You have HP. Numbers decide everything.

const WORD_CRYSTAL_MINE = {
  id: 'word-crystal-mine',
  title: 'The Word-Crystal Mine',
  description: 'Goblins raided the Arcane Mine and stole Word-Crystals that power spells. Infiltrate. Fight. Recover the crystals. Each crystal unlocks a new Word-Spell.',
  ageRange: '10+',
  difficulty: 'Apprentice',
  rewards: ['Word Crystal: Forge', 'Word Crystal: Courage', 'XP: +200'],
  start: 'mine-entrance',
  nodes: {
    'mine-entrance': {
      id: 'mine-entrance',
      title: 'Mine Entrance: Two Goblins',
      focusWord: 'Presence',
      channel: Channel.BODY,
      story: 'A mine shaft gapes in the mountain. Two goblin guards sit by a cookfire, playing dice. Goblin A: HP 6, AC 8. Goblin B: HP 5, AC 7. They haven\'t seen you. The shaft splits: left goes to the barracks, right goes to the crystal vault.',
      image: '/images/threshold.png',
      depth: 'COMBAT STATS: HP = 10 + (Body × 3). AC = 8 + Body. At Body 0: HP 10, AC 8. Attack roll: 1d6 + Action. Hit if roll ≥ enemy AC. Damage: 1d4 + Action. Against Goblin A (HP 6, AC 8): you need 1d6+0 ≥ 8. That\'s a 33% hit chance. Two hits kills him. Or sneak past with Presence: DC 6.',
      droneHz: 136.1,
      loot: ['Goblin Dagger: +1 Action (equipped)'],
      exits: {
        up:    { label: 'Sneak past — Presence check DC 6. Roll 1d6 + Mind (or re-roll with Presence word)', to: 'crystal-vault', virtue: 'presence' },
        right: { label: 'Attack Goblin A first (surprise round: auto-hit, roll 1d4 damage)', to: 'barracks', virtue: 'initiative' },
        down:  null,
        left:  null,
      },
    },
    barracks: {
      id: 'barracks',
      title: 'The Barracks: Four Goblins',
      focusWord: 'Resilience',
      channel: Channel.BODY,
      story: 'You kicked down the door. Four goblins leap up from straw beds. Chief Gruk: HP 10, AC 9. Three grunts: HP 5 each, AC 7. They surround you. Chief attacks first: roll 1d6, hit you on 8+. If hit: you take 1d4 damage. Then your turn. You can attack one target.',
      image: '/images/forest.png',
      depth: 'COMBAT ROUND: (1) Chief Gruk attacks you. He rolls 1d6. If ≥ your AC (8), you take 1d4 damage. (2) You attack one goblin. Roll 1d6 + Action (+1 if you have Goblin Dagger). If ≥ their AC, roll 1d4 + Action for damage. (3) Surviving grunts attack. Each rolls 1d6 vs your AC. Each hit = 1d3 damage. TIP: Kill grunts first. Less incoming damage.',
      droneHz: 174.6,
      loot: ['Goblin Chief\'s Key', 'Healing Draught (restore 4 HP)'],
      exits: {
        up:    { label: 'Attack a grunt first (AC 7, easiest to hit). Roll 1d6+0 vs AC 7. Damage: 1d4', to: 'crystal-vault', virtue: 'initiative' },
        right: { label: 'Attack Chief Gruk (AC 9, harder but he\'s the biggest threat). Roll 1d6+0 vs AC 9', to: 'crystal-vault', virtue: 'courage' },
        down:  { label: 'Drink Healing Draught (+4 HP) before fighting', to: 'barracks', virtue: 'patience' },
        left:  null,
      },
    },
    'crystal-vault': {
      id: 'crystal-vault',
      title: 'The Crystal Vault: Puzzle Lock',
      focusWord: 'Clarity',
      channel: Channel.MIND,
      story: 'A vault door of black iron. Three crystal sockets glow: red, blue, green. The plaque reads: "Forge requires heat. Courage requires fear. Clarity requires fog. Place them in order of creation." Wrong order triggers a lightning trap: 1d6 damage.',
      image: '/images/garden.png',
      depth: 'PUZZLE: Three sockets. Three crystals. You must place them in the correct order. The plaque gives clues. Heat (red) = Forge = Action. Fear (green) = Courage = Heart. Fog (blue) = Clarity = Mind. Order of creation: Action (doing), then Heart (feeling), then Mind (knowing). Solution: RED, GREEN, BLUE. Wrong answer = 1d6 lightning damage. You can skip the puzzle by spending the Chief\'s Key.',
      droneHz: 196.0,
      loot: ['Word Crystal: Forge', 'Word Crystal: Courage'],
      exits: {
        up:    { label: 'Place crystals: RED (Forge/Action), GREEN (Courage/Heart), BLUE (Clarity/Mind)', to: 'deep-shaft', virtue: 'clarity' },
        right: { label: 'Use Chief\'s Key to bypass puzzle (no damage, no check)', to: 'deep-shaft', virtue: 'cunning' },
        down:  { label: 'Guess randomly. 1/6 chance correct. 5/6 chance: take 1d6 lightning damage', to: 'deep-shaft', virtue: 'initiative' },
        left:  null,
      },
    },
    'deep-shaft': {
      id: 'deep-shaft',
      title: 'The Deep Shaft: The Cave-In',
      focusWord: 'Resilience',
      channel: Channel.ACTION,
      story: 'The mine groans. Dust falls from the ceiling. A cave-in starts behind you. Rocks tumble down the shaft. Ahead: a rope bridge over a chasm. Below: broken crystals and darkness. Behind: certain crushing. A goblin war-drone charges from the far side of the bridge. HP 12, AC 10. It\'s blocking your escape.',
      image: '/images/canyon.png',
      depth: 'TIMED COMBAT: The cave-in gives you 3 rounds before you take 2d6 crush damage (probably lethal). The War-Drone has HP 12, AC 10. You need 1d6 + Action ≥ 10 to hit. With Goblin Dagger (+1): need 1d6+1 ≥ 10. Still hard. BUT: if you have FORGE crystal, you can throw it as a bomb (1d8 damage, one use). If you have COURAGE crystal, you auto-pass the fear check and get +2 Action this round. Choose your crystal.',
      droneHz: 220.0,
      loot: ['War-Drone Plate (AC +1 armor)'],
      exits: {
        up:    { label: 'Throw FORGE crystal as bomb: 1d8 damage to drone. Then roll 1d6+0 to finish it.', to: 'surface', virtue: 'forge' },
        right: { label: 'Channel COURAGE: +2 Action this round. Roll 1d6+2 vs AC 10. Damage: 1d4+2', to: 'surface', virtue: 'courage' },
        down:  { label: 'Fight normally: roll 1d6+0 vs AC 10. You have 3 rounds before cave-in kills you.', to: 'surface', virtue: 'resilience' },
        left:  null,
      },
    },
    surface: {
      id: 'surface',
      title: 'The Surface: Dawn Escape',
      focusWord: 'Forge',
      channel: Channel.ACTION,
      story: 'You stumble into cold air. The mine collapses behind you. In your pack: two Word Crystals. FORGE hums with heat. COURAGE pulses like a heartbeat. Your character sheet updates. Words Owned: 3 (Presence, Forge, Courage). Level: 2. You feel the crystals settle into your vocabulary like weapons into a sheath.',
      image: '/images/summit.png',
      depth: 'REWARDS: FORGE = +2 to Action checks. Can craft improvised weapons in any dungeon. COURAGE = immune to fear effects. +1 to all checks when HP is below 5. XP: +200. Level 2 reached. +1 Channel Point to spend. Next dungeon: The Hall of a Thousand Faces (unlocked at Level 2).',
      droneHz: 261.6,
      loot: ['Word Crystal: Forge', 'Word Crystal: Courage', 'XP: +200', 'Level Up: 2'],
      exits: { up: null, down: null, left: null, right: null },
    },
  },
};

// ════════════════════════════════════════════════════════════
// ADVENTURE REGISTRY
// ════════════════════════════════════════════════════════════

export const ADVENTURES = {
  [STAT_GAUNTLET.id]: STAT_GAUNTLET,
  [WORD_CRYSTAL_MINE.id]: WORD_CRYSTAL_MINE,
};

// ─── HELPERS ─────────────────────────────────────────────
export function getAdventure(id) {
  return ADVENTURES[id] || null;
}

export function getDefaultAdventure() {
  return ADVENTURES['stat-gauntlet'];
}

export function getAllAdventures() {
  return Object.values(ADVENTURES);
}
