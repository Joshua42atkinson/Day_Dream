import { Channel } from '../constants.js';

// ════════════════════════════════════════════════════════════
// ADVENTURE: "The Feelings Garden" — Emotional Literacy (Ages 4-7)
// ════════════════════════════════════════════════════════════
// A gentle adventure for younger children learning to name
// and recognize their emotions. Uses the garden/nature metaphor.
// Each "feeling" is a flower that grows when you pay attention to it.

export const FEELINGS_GARDEN = {
  id: 'feelings-garden',
  title: 'The Feelings Garden',
  description: 'A gentle adventure where feelings bloom like flowers when you name them.',
  ageRange: '4-7',
  wordCount: 5,
  start: 'gate',

  nodes: {
    gate: {
      id: 'gate',
      title: 'The Garden Gate',
      focusWord: 'Feelings',
      channel: Channel.HEART,
      story: 'A little wooden gate stands in front of a secret garden. You can hear birds singing inside. A sign says: "To enter, just notice how you feel right now."',
      image: '/images/threshold.png',
      depth: 'Close your eyes for a moment. What do you feel in your chest right now? Is it warm? Tight? Fluttery? There is no wrong answer.',
      droneHz: 174.6,
      exits: {
        up:    { label: 'I feel happy — push the gate open!', to: 'sunny-patch', virtue: 'joy' },
        right: { label: 'I feel a little nervous...', to: 'shady-corner', virtue: 'honesty' },
        down:  null,
        left:  null,
      },
    },

    'sunny-patch': {
      id: 'sunny-patch',
      title: 'The Sunny Patch',
      focusWord: 'Joy',
      channel: Channel.HEART,
      story: 'Bright yellow flowers turn their faces toward you like little suns. They seem to be smiling. When you smile back, they glow even brighter.',
      image: '/images/garden.png',
      depth: 'Joy is like sunshine — it grows when you share it. Can you think of someone who makes you feel this warm?',
      droneHz: 220.0,
      exits: {
        up:    { label: 'Dance with the flowers', to: 'big-tree', virtue: 'expression' },
        right: { label: 'Look for more flowers', to: 'rain-puddle', virtue: 'curiosity' },
        down:  null,
        left:  null,
      },
    },

    'shady-corner': {
      id: 'shady-corner',
      title: 'The Shady Corner',
      focusWord: 'Worry',
      channel: Channel.BODY,
      story: 'Under a big leafy tree, small blue flowers curl their petals inward. They look shy. A caterpillar sits nearby, also curled up tight.',
      image: '/images/forest.png',
      depth: 'Sometimes worry makes us want to curl up small, like this caterpillar. What helps you uncurl when you feel worried?',
      droneHz: 136.1,
      exits: {
        up:    { label: 'Sit with the caterpillar quietly', to: 'big-tree', virtue: 'patience' },
        right: { label: 'Hum a little song to the flowers', to: 'rain-puddle', virtue: 'courage' },
        down:  null,
        left:  null,
      },
    },

    'rain-puddle': {
      id: 'rain-puddle',
      title: 'The Rain Puddle',
      focusWord: 'Sadness',
      channel: Channel.BODY,
      story: 'A shallow puddle reflects the sky. Tiny purple flowers grow at its edges, drinking the rain. Even the garden needs rain to grow.',
      image: '/images/canyon.png',
      depth: 'Tears are like rain for our hearts. They help things grow that could not grow without them. Is it okay to feel sad sometimes?',
      droneHz: 174.6,
      exits: {
        up:    { label: 'Watch the reflection in the water', to: 'big-tree', virtue: 'depth' },
        right: { label: 'Splash in the puddle gently', to: 'big-tree', virtue: 'play' },
        down:  null,
        left:  null,
      },
    },

    'big-tree': {
      id: 'big-tree',
      title: 'The Big Old Tree',
      focusWord: 'Brave',
      channel: Channel.ACTION,
      story: 'The biggest tree in the garden has seen a thousand seasons. It has been through storms and sunshine, snow and rain. And it is still here, still growing. So are you.',
      image: '/images/summit.png',
      depth: 'Being brave does not mean you never feel scared. It means you feel scared and you keep growing anyway — just like this tree.',
      droneHz: 261.6,
      exits: { up: null, down: null, left: null, right: null },
    },
  },
};
