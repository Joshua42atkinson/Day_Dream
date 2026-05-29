// ════════════════════════════════════════════════════════════
// DAYDREAM — Curriculum Data Model
// ════════════════════════════════════════════════════════════
//
// Architecture: No backend. The graph IS the game.
// Like RuneScape: runs in browser, AI model downloads once.

// Re-export shared constants so other files can import from curriculum.js
export { Channel, CHANNEL_COLORS, CHANNEL_QUESTIONS, Mastery } from './constants.js';
import { Channel } from './constants.js';
import { FEELINGS_GARDEN } from './adventures/feelings-garden.js';

// ─── THE DEMO ADVENTURE: "Bias & Mirrors" ────────────────
//
// Each node is a "room" in the MUD. Each word is a spell.
// The student's path through the DAG IS the assessment.


export const ADVENTURES = {
  'bias-and-mirrors': {
    id: 'bias-and-mirrors',
    title: 'Bias & Mirrors',
    description: 'A journey through self-reflection, where every mirror shows a different truth.',
    ageRange: '8-14',
    wordCount: 5,
    start: 'threshold',
    nodes: {
      threshold: {
        id: 'threshold',
        title: 'The Threshold',
        focusWord: 'Presence',
        channel: Channel.BODY,
        story: 'You stand before an ancient stone archway. Warm light spills through from the other side. The air is still. Something waits for you to choose.',
        image: '/images/threshold.png',
        depth: 'What does it mean to be present? Not yesterday, not tomorrow — just here, at this archway, with the light on your face.',
        droneHz: 136.1,
        exits: {
          up:    { label: 'Step through with curiosity', to: 'forest', virtue: 'curiosity' },
          right: { label: 'Touch the stone and listen', to: 'garden', virtue: 'courage' },
          down:  null,
          left:  null,
        },
      },
      forest: {
        id: 'forest',
        title: 'The Glass Forest',
        focusWord: 'Bias',
        channel: Channel.MIND,
        story: "Dark glass trees reflect a warped version of you. A whisper echoes: 'You will fail here, just as you have before.' The reflection shows what you fear — not what is true.",
        image: '/images/forest.png',
        depth: 'The whisper uses your own voice. Why do we believe the worst stories we tell about ourselves? Where did you first learn that story?',
        droneHz: 174.6,
        exits: {
          up:    { label: 'Look closer at the reflection', to: 'summit', virtue: 'curiosity' },
          right: { label: 'Speak back to the whisper', to: 'canyon', virtue: 'courage' },
          left:  { label: 'Sit quietly in the moss', to: 'garden', virtue: 'caution' },
          down:  { label: 'Ask: why does this voice know my name?', to: 'canyon', virtue: 'depth' },
        },
      },
      garden: {
        id: 'garden',
        title: 'The Quiet Garden',
        focusWord: 'Patience',
        channel: Channel.BODY,
        story: 'A walled garden filled with sunlight and birdsong. A fountain murmurs at the center. Nothing here demands anything of you. You can simply be.',
        image: '/images/garden.png',
        depth: "Patience isn't waiting. It's the ability to be at peace while things unfold. What grows when you stop pulling at it?",
        droneHz: 196.0,
        exits: {
          up:    { label: 'Explore what grows here', to: 'summit', virtue: 'curiosity' },
          right: { label: 'Follow the path beyond the wall', to: 'canyon', virtue: 'courage' },
          down:  { label: 'Stay and listen to the water', to: 'summit', virtue: 'depth' },
          left:  null,
        },
      },
      canyon: {
        id: 'canyon',
        title: 'The Stone Bridge',
        focusWord: 'Resilience',
        channel: Channel.ACTION,
        story: 'A narrow bridge over a deep canyon. The wind pushes hard. Across the gap, golden light breaks through clouds. Every step forward is a choice to keep going.',
        image: '/images/canyon.png',
        depth: "Resilience isn't about not falling. It's about what you do with the wind. Can you lean into it instead of fighting it?",
        droneHz: 220.0,
        exits: {
          up:    { label: 'Cross the bridge', to: 'summit', virtue: 'curiosity' },
          right: { label: "Run across — don't look down", to: 'summit', virtue: 'courage' },
          down:  { label: 'Look into the canyon below', to: 'summit', virtue: 'depth' },
          left:  { label: 'Go back to solid ground', to: 'garden', virtue: 'caution' },
        },
      },
      summit: {
        id: 'summit',
        title: 'The Summit',
        focusWord: 'Clarity',
        channel: Channel.MIND,
        story: "Above the clouds. The world stretches out below — every path you've walked visible from here. You understand now: there were no wrong choices. Only the ones you made.",
        image: '/images/summit.png',
        depth: "Clarity isn't knowing the answer. It's seeing the question clearly for the first time.",
        droneHz: 261.6,
        exits: { up: null, down: null, left: null, right: null },
      },
    },
  },
  // ─── Register additional adventures here ────────────────
  [FEELINGS_GARDEN.id]: FEELINGS_GARDEN,
};

// ─── HELPERS ─────────────────────────────────────────────
export function getAdventure(id) {
  return ADVENTURES[id] || null;
}

export function getDefaultAdventure() {
  return ADVENTURES['bias-and-mirrors'];
}

export function getAllAdventures() {
  return Object.values(ADVENTURES);
}
