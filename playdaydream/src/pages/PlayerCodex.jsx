import { useNavigate } from 'react-router-dom';
import { Symbols, ARCANA, Archetypes } from '../data/arcana';
import { CHANNEL_COLORS } from '../data/constants';

// ════════════════════════════════════════════════════════════
// PLAYER'S CODEX — The Great Game Handbook
// ════════════════════════════════════════════════════════════
// Borrowed from Bertrand's PlayerHandbook "Tome" concept.
// Each tome explains a facet of the Great Game framework.

const TOMES = [
  {
    id: 'tome1',
    icon: '☆',
    title: 'Tome I: The Premise',
    subtitle: 'What is The Great Game?',
    sections: [
      {
        heading: 'You Are the Player',
        content: 'In The Great Game, you do not read a story. You live one. You create your own character, forge your own deck of power words, and walk paths that only you can walk. Every choice shapes who you become.',
      },
      {
        heading: 'Words Are Spells',
        content: 'Vocabulary is not memorization. It is the fulcrum of understanding. Each word you encounter is a spell — a lens that refracts reality. The ARCANA contains twenty such spells, each tied to a channel of being.',
      },
      {
        heading: 'The Four Channels',
        content: 'Mind (◆) — Thought, clarity, wonder. Heart (◇) — Emotion, courage, empathy. Body (△) — Presence, patience, rest. Action (○) — Forge, creation, initiative. Every word belongs to one. Every choice strengthens one.',
      },
    ],
  },
  {
    id: 'tome2',
    icon: '◆',
    title: 'Tome II: The ARCANA',
    subtitle: 'The Twenty Words of Power',
    sections: [
      {
        heading: 'The Symbol System',
        content: `${Symbols.STONE} Stone = noun / entity. ${Symbols.SPARK} Spark = verb / action. ${Symbols.PRISM} Prism = quality / lens. ${Symbols.VOID} Void = abstract concept. ${Symbols.STAR} Star = keystone / anchor. Every word carries a symbol that reveals its nature.`,
      },
      {
        heading: 'The Stages of Mastery',
        content: 'Each word has three stages. Stage I: Encountered — you have seen it. Stage II: Experienced — you have used it. Stage III: Owned — it is part of you. The journey from Encountered to Owned is the heart of the game.',
      },
      {
        heading: 'Synergies',
        content: 'Some words resonate together. When both Resilience and Patience appear in your deck, you gain the Steadfast bonus. When Courage and Vulnerability align, you become Wholehearted. These are not random — they are the geometry of character.',
      },
    ],
  },
  {
    id: 'tome3',
    icon: '◇',
    title: 'Tome III: The Player',
    subtitle: 'Character, Deck, and Emergent Class',
    sections: [
      {
        heading: 'Creating Your Character',
        content: 'Name yourself. Choose an archetype: The Oracle (mind), The Bard (heart), The Cultivator (body), The Templar (action), or The Architect (balanced). Set your channel attunement. Your character sheet tracks every step of your journey.',
      },
      {
        heading: 'Building Your Deck',
        content: 'Your deck is your directional prompt system. Up to twenty words. Each word shapes the AI storyteller\'s narration. A deck heavy in Mind words yields contemplative journeys. A Heart-heavy deck yields emotional depth. You are the architect of your experience.',
      },
      {
        heading: 'Emergent Class',
        content: 'As you play, your channel attunement shifts. The dominant channel determines your emergent class. An Oracle who ventures into Heart territory may become a Sage. A Templar who embraces Body may become a Warden. Class is not chosen. It is revealed.',
      },
    ],
  },
  {
    id: 'tome4',
    icon: '△',
    title: 'Tome IV: The Journey',
    subtitle: 'Authoring, Playing, and the Oral Tradition',
    sections: [
      {
        heading: 'Authoring a Journey',
        content: 'A journey is a word-DAG: a directed graph of story nodes. Each node has a focus word, a scene, choices, and a depth question. Use the Mad Libs slots to weave your word into narrative. Export as StoryGraph JSON. Share with the world.',
      },
      {
        heading: 'The Oral Tradition',
        content: 'Put on headphones. Close your eyes. The AI storyteller speaks directly into your ears. You navigate by voice: "up," "right," "deeper." No screen. No distraction. Just you, the words, and the path. This is how stories were told before screens existed.',
      },
      {
        heading: 'Trail Review',
        content: 'After each journey, review your trail. Which words did you encounter? Which channels did you strengthen? What is your SpellBook? What is your emergent class? The trail is the mirror. It shows you who you are becoming.',
      },
    ],
  },
  {
    id: 'tome5',
    icon: '○',
    title: 'Tome V: The Codex',
    subtitle: 'Privacy, Sovereignty, and the Local-First Oath',
    sections: [
      {
        heading: 'Your Data Is Yours',
        content: 'Daydream runs entirely on your device. Your character, your deck, your trail — all stored in your browser\'s localStorage. No accounts. No cloud. No tracking. We cannot see your data. We do not want to. Your journey belongs to you.',
      },
      {
        heading: 'The Local AI Oath',
        content: 'The storyteller AI runs on your own hardware — StepAudio R1.1 or Nemotron via LM Studio. Your conversations never leave your machine. This is not a limitation. It is a promise. In an age of surveillance, privacy is the most powerful spell.',
      },
      {
        heading: 'GPLv3 License',
        content: 'Daydream is free software. The code is open. The framework is yours to extend, modify, and share. The Great Game belongs to everyone who plays it.',
      },
    ],
  },
];

function TomeSection({ tome, index }) {
  return (
    <div className="tome-section animate-fade-in-up" style={{ animationDelay: `${index * 0.15}s` }}>
      <div className="flex items-center gap-2" style={{ marginBottom: '4px' }}>
        <span style={{ fontSize: '1.4rem', color: 'var(--gold)' }}>{tome.icon}</span>
        <h2 className="tome-title">{tome.title}</h2>
      </div>
      <p className="tome-subtitle">{tome.subtitle}</p>
      {tome.sections.map((section, i) => (
        <div key={i} style={{ marginBottom: '16px' }}>
          <h3
            className="font-heading"
            style={{
              fontSize: '0.95rem',
              color: 'var(--ink-primary)',
              marginBottom: '6px',
              letterSpacing: '0.02em',
            }}
          >
            {section.heading}
          </h3>
          <p
            className="font-serif"
            style={{
              fontSize: '0.9rem',
              lineHeight: 1.7,
              color: 'var(--ink-secondary)',
            }}
          >
            {section.content}
          </p>
        </div>
      ))}
    </div>
  );
}

function ArcanaGrid() {
  const channels = { mind: '◆', heart: '◇', body: '△', action: '○' };
  return (
    <div className="parchment-panel" style={{ marginTop: '20px' }}>
      <h3 className="font-heading text-gold" style={{ fontSize: '1rem', marginBottom: '16px', textAlign: 'center' }}>
        The Twenty Words
      </h3>
      <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fill, minmax(140px, 1fr))', gap: '8px' }}>
        {ARCANA.map((word) => {
          const ch = word.channel.toLowerCase();
          const color = {
            mind: 'var(--mind)',
            heart: 'var(--heart)',
            body: 'var(--body)',
            action: 'var(--action)',
          }[ch] || 'var(--ink-muted)';
          return (
            <div
              key={word.word}
              style={{
                padding: '10px 12px',
                borderRadius: 'var(--radius-md)',
                background: 'var(--void-surface)',
                border: `1px solid ${color}20`,
                display: 'flex',
                alignItems: 'center',
                gap: '8px',
              }}
            >
              <span style={{ color, fontSize: '0.9rem' }}>{word.symbol}</span>
              <div>
                <div style={{ fontSize: '0.8rem', fontWeight: 600, color: 'var(--ink-primary)' }}>
                  {word.word}
                </div>
                <div style={{ fontSize: '0.6rem', color: 'var(--ink-muted)', textTransform: 'uppercase', letterSpacing: '0.1em' }}>
                  {ch}
                </div>
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
}

export default function PlayerCodex() {
  const navigate = useNavigate();

  return (
    <div className="scrollable-page">
      <div className="starfield-bg" />
      <div className="hero-glow" />

      <div className="relative z-10" style={{ maxWidth: '680px', margin: '0 auto', padding: '3rem 1.5rem 4rem' }}>
        {/* Header */}
        <header style={{ textAlign: 'center', marginBottom: '3rem', animation: 'fadeInUp 0.8s var(--ease-out-expo) forwards' }}>
          <p
            className="font-sans tracking-wider"
            style={{
              fontSize: '0.65rem',
              color: 'var(--gold-dim)',
              textTransform: 'uppercase',
              marginBottom: '12px',
            }}
          >
            A Perspective Enhancement Module
          </p>
          <h1
            className="font-heading"
            style={{
              fontSize: 'clamp(2rem, 6vw, 3.2rem)',
              color: 'var(--ink-bright)',
              lineHeight: 1.1,
              letterSpacing: '0.02em',
              marginBottom: '12px',
            }}
          >
            The Player's <span style={{ color: 'var(--gold)', fontStyle: 'italic' }}>Codex</span>
          </h1>
          <p
            className="font-serif"
            style={{
              fontSize: 'clamp(0.9rem, 2vw, 1.1rem)',
              color: 'var(--ink-muted)',
              fontStyle: 'italic',
              maxWidth: '400px',
              margin: '0 auto',
              lineHeight: 1.6,
            }}
          >
            The rules, words, and wisdom of The Great Game. Read before you play.
          </p>

          <div className="symbol-divider" style={{ marginTop: '24px' }}>
            <span>☆</span>
          </div>
        </header>

        {/* Arcana Quick Reference */}
        <div className="animate-fade-in-up" style={{ animationDelay: '0.2s', marginBottom: '3rem' }}>
          <ArcanaGrid />
        </div>

        {/* Tomes */}
        <div className="stagger-children">
          {TOMES.map((tome, i) => (
            <TomeSection key={tome.id} tome={tome} index={i} />
          ))}
        </div>

        {/* Footer CTA */}
        <div
          className="parchment-panel animate-fade-in-up"
          style={{ marginTop: '3rem', textAlign: 'center', animationDelay: '0.8s' }}
        >
          <p className="font-serif" style={{ fontSize: '0.95rem', color: 'var(--ink-secondary)', marginBottom: '16px', fontStyle: 'italic' }}>
            "The story does not exist until you walk it."
          </p>
          <button className="btn btn-primary" onClick={() => navigate('/create/character')}>
            <span>◆</span> Begin Your Journey
          </button>
        </div>

        {/* Back */}
        <div style={{ textAlign: 'center', marginTop: '2rem' }}>
          <button className="btn btn-ghost" onClick={() => navigate('/')} style={{ fontSize: '0.75rem' }}>
            ← Return to the Threshold
          </button>
        </div>
      </div>
    </div>
  );
}
