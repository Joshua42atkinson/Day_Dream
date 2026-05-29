// ════════════════════════════════════════════════════════════
// DAYDREAM — Shared Constants
// ════════════════════════════════════════════════════════════
// Extracted to break circular deps between curriculum.js and adventures.

// ─── THE FOUR CHANNELS (from The Great Game) ─────────────
export const Channel = Object.freeze({
  MIND:   'mind',    // 🟢 The Sage — analytical
  HEART:  'heart',   // 🟠 The Mystic — emotional
  BODY:   'body',    // 🔵 The Healer — somatic
  ACTION: 'action',  // 🟡 The Builder — manifesting
});

export const CHANNEL_COLORS = {
  [Channel.MIND]:   { primary: '#4a9e6e', bg: 'rgba(74,158,110,0.08)' },
  [Channel.HEART]:  { primary: '#d4783c', bg: 'rgba(212,120,60,0.08)' },
  [Channel.BODY]:   { primary: '#4a7eb5', bg: 'rgba(74,126,181,0.08)' },
  [Channel.ACTION]: { primary: '#c4a43c', bg: 'rgba(196,164,60,0.08)' },
};

export const CHANNEL_QUESTIONS = {
  [Channel.MIND]:   'What does this mean?',
  [Channel.HEART]:  'Where is the love here?',
  [Channel.BODY]:   'What is my body telling me?',
  [Channel.ACTION]: 'How do I make this real?',
};

// ─── MASTERY LEVELS (The Great Recycler) ─────────────────
export const Mastery = Object.freeze({
  ENCOUNTERED: 'encountered',  // 🔮 Saw the card
  EXPERIENCED: 'experienced',  // ⚡ Felt the word in context
  OWNED:       'owned',        // 🌟 Connected to personal meaning
  MASTERED:    'mastered',     // 👑 Sees the web of meaning
});
