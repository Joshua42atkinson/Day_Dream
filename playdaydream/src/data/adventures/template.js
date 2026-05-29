// ════════════════════════════════════════════════════════════
// ADVENTURE TEMPLATE — Copy this file to create a new course
// ════════════════════════════════════════════════════════════
//
// Each adventure is a graph of "rooms" (slides).
// Each room has:
//   - A focus word (the vocabulary being taught)
//   - A story (the narrative context that gives the word meaning)
//   - A depth question (Socratic reflection — shown on double-tap)
//   - Up to 4 exits (swipe directions that lead to other rooms)
//
// The child's PATH through the graph IS the assessment.
// No grades, no scores — just the journey.
//
// TIPS:
//   - Keep stories to 2-3 sentences. Children skim.
//   - Depth questions should be experiential, not definitional.
//     BAD:  "What does resilience mean?"
//     GOOD: "Can you think of a time you kept going when it was hard?"
//   - Images go in /public/images/adventures/[your-adventure-id]/
//   - droneHz sets the ambient tone (lower = heavier, higher = lighter)
//     Common frequencies: 136 (Om), 174 (gentle), 196 (warm), 220 (bright), 261 (clear)

import { Channel } from '../constants.js';

export const MY_ADVENTURE = {
  id: 'my-adventure-id',          // URL-safe, lowercase with dashes
  title: 'My Adventure Title',     // What parents see in the picker
  description: 'A one-sentence pitch for this adventure.',
  ageRange: '6-10',                // Suggested age range
  wordCount: 5,                    // How many focus words
  start: 'first-room',            // ID of the starting room

  nodes: {
    'first-room': {
      id: 'first-room',
      title: 'The Beginning',
      focusWord: 'Wonder',
      channel: Channel.HEART,      // MIND | HEART | BODY | ACTION
      story: 'Your story text here. Keep it short and vivid.',
      image: '/images/adventures/my-adventure-id/first.png',
      depth: 'Your Socratic question here. Make it personal.',
      droneHz: 174.6,
      exits: {
        up:    { label: 'Choice A text', to: 'second-room', virtue: 'curiosity' },
        right: { label: 'Choice B text', to: 'third-room',  virtue: 'courage' },
        down:  null,  // null = no exit in this direction
        left:  null,
      },
    },

    'second-room': {
      id: 'second-room',
      title: 'The Middle',
      focusWord: 'Growth',
      channel: Channel.ACTION,
      story: 'What happens next in the story...',
      image: '/images/adventures/my-adventure-id/second.png',
      depth: 'A deeper question about this word...',
      droneHz: 220.0,
      exits: {
        up:    { label: 'Go to the end', to: 'final-room', virtue: 'depth' },
        right: null,
        down:  null,
        left:  { label: 'Go back and reconsider', to: 'first-room', virtue: 'caution' },
      },
    },

    'third-room': {
      id: 'third-room',
      title: 'The Other Path',
      focusWord: 'Courage',
      channel: Channel.BODY,
      story: 'An alternate path through the story...',
      image: '/images/adventures/my-adventure-id/third.png',
      depth: 'What does it feel like to choose the harder path?',
      droneHz: 196.0,
      exits: {
        up:    { label: 'Press forward', to: 'final-room', virtue: 'courage' },
        right: null,
        down:  null,
        left:  null,
      },
    },

    'final-room': {
      id: 'final-room',
      title: 'The Arrival',
      focusWord: 'Clarity',
      channel: Channel.MIND,
      story: 'The end of the journey. What has changed?',
      image: '/images/adventures/my-adventure-id/final.png',
      depth: 'Look back at the path you chose. What does it tell you about yourself?',
      droneHz: 261.6,
      exits: { up: null, down: null, left: null, right: null }, // No exits = end screen
    },
  },
};
