import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { Symbols, ARCANA } from '../data/arcana';
import { useCodexProgress } from '../hooks/useCodexProgress';
import ReflectionModal from '../components/ReflectionModal';

// ════════════════════════════════════════════════════════════
// THE ACADEMY OF THE GREAT GAME
// ════════════════════════════════════════════════════════════
// Not a handbook. A metaphysical class.
// Each Tome is a course. Each section is a lesson.
// You do not read the rules. You are initiated into them.
// Completion requires reflection — the Socratic mirror.

const TOMES = [
  {
    id: 'tome1',
    icon: '☆',
    title: 'Class I: The Nature of the Game',
    subtitle: 'You do not play it. You are it.',
    sections: [
      {
        heading: 'Lesson 1: You Are the Player',
        content: 'In The Great Game, you do not read a story. You live one. You create your own character, forge your own deck of power words, and walk paths that only you can walk. Every choice shapes who you become. The Game is not separate from life. It is a lens that makes the invisible visible.',
        socraticQuestion: 'If your life were a game, what would be your current "level," and what skill are you grinding right now?',
      },
      {
        heading: 'Lesson 2: Words Are Spells',
        content: 'Vocabulary is not memorization. It is the fulcrum of understanding. Each word you encounter is a spell — a lens that refracts reality. The ARCANA contains twenty such spells, each tied to a channel of being. When you speak a word with intention, you cast it. When you write it, you bind it. When you live it, you become it.',
        socraticQuestion: 'What is one word that, when you hear it, feels like it is speaking directly to something unnameable inside you?',
      },
      {
        heading: 'Lesson 3: The Four Channels of Being',
        content: 'Mind (◇) — Thought, clarity, wonder. The channel of inquiry and pattern-recognition. Heart (◇) — Emotion, courage, empathy. The channel of connection and risk. Body (△) — Presence, patience, rest. The channel of embodiment and boundary. Action (○) — Forge, creation, initiative. The channel of transformation and will. Every word belongs to one. Every choice strengthens one. A life out of balance is a game played on hard mode.',
        socraticQuestion: 'Which of the Four Channels feels most neglected in your daily life, and what would change if you honored it?',
      },
    ],
  },
  {
    id: 'tome2',
    icon: '◆',
    title: 'Class II: The ARCANA & The Symbol System',
    subtitle: 'The geometry of meaning',
    sections: [
      {
        heading: 'Lesson 1: The Five Symbols',
        content: `${Symbols.STONE} Stone = noun / entity / that which endures. ${Symbols.SPARK} Spark = verb / action / that which transforms. ${Symbols.PRISM} Prism = quality / lens / that which refracts. ${Symbols.VOID} Void = abstract concept / that which cannot be held. ${Symbols.STAR} Star = keystone / anchor / that which guides. Every word carries a symbol that reveals its nature. The symbols are not arbitrary. They are the alphabet of the soul.`,
        socraticQuestion: 'If you had to assign a symbol to your current emotional state, which would it be and why?',
      },
      {
        heading: 'Lesson 2: The Three Stages of Mastery',
        content: 'Each word has three stages. Stage I: Encountered — you have seen it, but it has not yet seen you. Stage II: Experienced — you have used it in the furnace of circumstance. Stage III: Owned — it is part of your cellular vocabulary. You no longer think the word. You are the word. The journey from Encountered to Owned is the heart of the game. Most people live their entire lives with only a handful of Owned words. The Academy exists to change that.',
        socraticQuestion: 'Name a word you have "Owned" — one that has changed your behavior, not just your vocabulary.',
      },
      {
        heading: 'Lesson 3: Synergies & Resonance',
        content: 'Some words resonate together. When both Resilience and Patience appear in your deck, you gain the Steadfast bonus. When Courage and Vulnerability align, you become Wholehearted. These are not random — they are the geometry of character. The universe rewards coherence. A deck with synergies is not stronger because of math. It is stronger because it is true.',
        socraticQuestion: 'What two qualities in your life, when combined, create an effect greater than the sum of their parts?',
      },
    ],
  },
  {
    id: 'tome3',
    icon: '◇',
    title: 'Class III: The Player — Character & Class',
    subtitle: 'Identity is not chosen. It is revealed.',
    sections: [
      {
        heading: 'Lesson 1: Creating Your Character',
        content: 'Name yourself. This is not vanity. It is the first spell. Choose an archetype: The Oracle (Mind), The Bard (Heart), The Cultivator (Body), The Templar (Action), or The Architect (Balanced). Set your channel attunement. Your character sheet tracks every step of your journey. But remember: the character sheet is not the character. It is the map. The territory is your life.',
        socraticQuestion: 'If you were an NPC in someone else\'s story, what one-line description would appear when they hovered over you?',
      },
      {
        heading: 'Lesson 2: Building Your Deck',
        content: 'Your deck is your directional prompt system. Up to twenty words. Each word shapes the AI storyteller\'s narration. A deck heavy in Mind words yields contemplative journeys. A Heart-heavy deck yields emotional depth. An Action-heavy deck forces you to choose. You are the architect of your experience. But the deck also architects you. Choose carefully.',
        socraticQuestion: 'What five words would you put in your "daily deck" to steer your inner narrator toward who you want to become?',
      },
      {
        heading: 'Lesson 3: Emergent Class',
        content: 'As you play, your channel attunement shifts. The dominant channel determines your emergent class. An Oracle who ventures into Heart territory may become a Sage. A Templar who embraces Body may become a Warden. Class is not chosen. It is revealed. You do not declare yourself a Bard. The Game recognizes you as one. This is existential LitRPG: the system does not assign you a class. Your choices write it into the code of the universe.',
        socraticQuestion: 'What "class" are you currently grinding toward, and what daily action is the XP for that class?',
      },
    ],
  },
  {
    id: 'tome4',
    icon: '△',
    title: 'Class IV: The Journey — Authoring & The Oral Tradition',
    subtitle: 'You are both the author and the protagonist',
    sections: [
      {
        heading: 'Lesson 1: Authoring a Journey',
        content: 'A journey is a word-DAG: a directed graph of story nodes. Each node has a focus word, a scene, choices, and a depth question. Use the Mad Libs slots to weave your word into narrative. Export as StoryGraph JSON. Share with the world. But the true author is not the one who writes the nodes. It is the one who walks them. Every journey you author is a map of your own psyche.',
        socraticQuestion: 'What is a recurring "scene" in your life that you wish you could rewrite, and what word would be its focus?',
      },
      {
        heading: 'Lesson 2: The Oral Tradition',
        content: 'Put on headphones. Close your eyes. The AI storyteller speaks directly into your ears. You navigate by voice: "up," "right," "deeper." No screen. No distraction. Just you, the words, and the path. This is how stories were told before screens existed. This is how wisdom was transmitted before books. The Oral Tradition is not a feature. It is a return.',
        socraticQuestion: 'When was the last time you received wisdom without any visual input — just voice, presence, and attention?',
      },
      {
        heading: 'Lesson 3: Trail Review & The Mirror',
        content: 'After each journey, review your trail. Which words did you encounter? Which channels did you strengthen? What is your SpellBook? What is your emergent class? The trail is the mirror. It shows you who you are becoming. The Great Game does not grade you. It reflects you. The reflection is the reward. The reflection is the punishment. The reflection is the truth.',
        socraticQuestion: 'If you could see a complete "trail" of your last year — every choice, every word, every channel — what would surprise you most?',
      },
    ],
  },
  {
    id: 'tome5',
    icon: '○',
    title: 'Class V: Sovereignty & The Local-First Oath',
    subtitle: 'Privacy is the most powerful spell',
    sections: [
      {
        heading: 'Lesson 1: Your Data Is Your Soul',
        content: 'Daydream runs entirely on your device. Your character, your deck, your trail — all stored in your browser\'s localStorage. No accounts. No cloud. No tracking. We cannot see your data. We do not want to. Your journey belongs to you. In an age where attention is mined and behavior is predicted, owning your own data is a revolutionary act. It is also a sacred one.',
        socraticQuestion: 'What piece of your personal data, if it were truly private and yours alone, would reveal the most about who you actually are?',
      },
      {
        heading: 'Lesson 2: The Local AI Covenant',
        content: 'The storyteller AI runs on your own hardware — StepAudio R1.1 or Nemotron via LM Studio. Your conversations never leave your machine. This is not a limitation. It is a promise. In an age of surveillance, privacy is the most powerful spell. The covenant between you and your machine is ancient: it serves you. It does not serve its makers. It does not serve advertisers. It serves the Game, and the Game serves you.',
        socraticQuestion: 'What would you say to an AI if you knew no one else would ever hear the conversation?',
      },
      {
        heading: 'Lesson 3: The Open Source Vow',
        content: 'Daydream is free software under GPLv3. The code is open. The framework is yours to extend, modify, and share. The Great Game belongs to everyone who plays it. But ownership is not the same as mastery. The vow is this: if you learn from the Game, teach another. If you build upon it, share what you build. Knowledge that is hoarded rots. Knowledge that flows grows. This is the final lesson of the Academy. It is also the first.',
        socraticQuestion: 'What is one piece of wisdom you have gained that you have not yet shared with anyone, and what stops you?',
      },
    ],
  },
];

// ─── Components ────────────────────────────────────────────

function ProgressBar({ pct, label }) {
  return (
    <div style={{ marginBottom: '8px' }}>
      <div className="flex items-center justify-between" style={{ marginBottom: '4px' }}>
        <span className="font-sans" style={{ fontSize: '0.65rem', color: 'var(--ink-muted)', textTransform: 'uppercase', letterSpacing: '0.1em' }}>
          {label}
        </span>
        <span className="font-heading" style={{ fontSize: '0.7rem', color: 'var(--gold)' }}>{pct}%</span>
      </div>
      <div style={{ width: '100%', height: '4px', background: 'var(--void-raised)', borderRadius: 'var(--radius-full)', overflow: 'hidden' }}>
        <div style={{ width: `${pct}%`, height: '100%', background: 'var(--gold)', borderRadius: 'var(--radius-full)', transition: 'width 1s var(--ease-out-expo)' }} />
      </div>
    </div>
  );
}

function LessonCard({ section, tomeId, lessonIndex, isCompleted, isLocked, onComplete }) {
  const [showReflect, setShowReflect] = useState(false);

  if (isLocked) {
    return (
      <div style={{
        padding: '14px 16px',
        borderRadius: 'var(--radius-md)',
        background: 'var(--void-raised)',
        border: '1px solid rgba(255,255,255,0.04)',
        opacity: 0.4,
        marginBottom: '12px',
      }}>
        <div className="flex items-center gap-2">
          <span style={{ fontSize: '0.8rem', color: 'var(--ink-faint)' }}>🔒</span>
          <span className="font-sans" style={{ fontSize: '0.75rem', color: 'var(--ink-faint)' }}>
            {section.heading}
          </span>
        </div>
      </div>
    );
  }

  return (
    <>
      <div style={{
        padding: '16px',
        borderRadius: 'var(--radius-md)',
        background: 'var(--void-surface)',
        border: isCompleted ? '1px solid rgba(16,185,129,0.15)' : '1px solid rgba(201,168,76,0.1)',
        marginBottom: '12px',
        transition: 'all 0.3s ease',
      }}>
        <div className="flex items-center gap-2" style={{ marginBottom: '8px' }}>
          <span style={{ fontSize: '0.9rem', color: isCompleted ? 'var(--success)' : 'var(--gold)' }}>
            {isCompleted ? '✦' : '◌'}
          </span>
          <h3 className="font-heading" style={{ fontSize: '0.9rem', color: 'var(--ink-primary)', letterSpacing: '0.02em' }}>
            {section.heading}
          </h3>
        </div>
        <p className="font-serif" style={{ fontSize: '0.85rem', lineHeight: 1.7, color: 'var(--ink-secondary)', marginBottom: '12px' }}>
          {section.content}
        </p>
        {!isCompleted ? (
          <button className="btn btn-primary" style={{ fontSize: '0.75rem', padding: '8px 14px' }} onClick={() => setShowReflect(true)}>
            <span>◇</span> Reflect & Complete
          </button>
        ) : (
          <span className="font-sans" style={{ fontSize: '0.7rem', color: 'var(--success)' }}>
            ✦ Reflection recorded — {XP_PER_LESSON} XP earned
          </span>
        )}
      </div>

      {showReflect && (
        <ReflectionModal
          lesson={section}
          onComplete={() => { onComplete(tomeId, lessonIndex); setShowReflect(false); }}
          onClose={() => setShowReflect(false)}
        />
      )}
    </>
  );
}

function TomeCard({ tome, index, progress }) {
  const unlocked = progress.isTomeUnlocked(index);
  const completedInTome = tome.sections.filter((_, i) => progress.isLessonCompleted(tome.id, i)).length;
  const tomePct = tome.sections.length > 0 ? Math.round((completedInTome / tome.sections.length) * 100) : 0;

  return (
    <div className="tome-section animate-fade-in-up" style={{
      animationDelay: `${index * 0.15}s`,
      opacity: unlocked ? 1 : 0.5,
    }}>
      <div className="flex items-center gap-2" style={{ marginBottom: '4px' }}>
        <span style={{ fontSize: '1.4rem', color: unlocked ? 'var(--gold)' : 'var(--ink-faint)' }}>
          {unlocked ? tome.icon : '🔒'}
        </span>
        <h2 className="tome-title" style={{ opacity: unlocked ? 1 : 0.5 }}>{tome.title}</h2>
      </div>
      <p className="tome-subtitle" style={{ opacity: unlocked ? 1 : 0.4 }}>{tome.subtitle}</p>

      {unlocked && (
        <div style={{ marginBottom: '16px' }}>
          <ProgressBar pct={tomePct} label={`Class Progress`} />
        </div>
      )}

      {tome.sections.map((section, i) => (
        <LessonCard
          key={i}
          section={section}
          tomeId={tome.id}
          lessonIndex={i}
          isCompleted={progress.isLessonCompleted(tome.id, i)}
          isLocked={!unlocked}
          onComplete={progress.completeLesson}
        />
      ))}
    </div>
  );
}

function EnrollmentGate({ onEnroll }) {
  return (
    <div className="relative z-10 flex flex-col items-center justify-center min-h-screen" style={{ padding: '2rem 1.5rem', textAlign: 'center' }}>
      <div className="hero-glow" style={{ opacity: 0.6 }} />
      <span style={{ fontSize: '3rem', color: 'var(--gold)', marginBottom: '16px' }}>☆</span>
      <h1 className="font-heading" style={{ fontSize: 'clamp(2rem, 6vw, 3.2rem)', color: 'var(--ink-bright)', lineHeight: 1.1, marginBottom: '12px' }}>
        The Academy of<br /><span style={{ color: 'var(--gold)', fontStyle: 'italic' }}>The Great Game</span>
      </h1>
      <p className="font-serif" style={{ fontSize: 'clamp(0.9rem, 2.5vw, 1.1rem)', color: 'var(--ink-muted)', maxWidth: '420px', lineHeight: 1.7, marginBottom: '32px', fontStyle: 'italic' }}>
        You are about to enroll in a metaphysical class.
        These are not rules. They are laws of the universe, written in the language of play.
        Each lesson ends with a mirror. The mirror does not judge. It reveals.
      </p>
      <button className="btn btn-primary" onClick={onEnroll}>
        <span>◈</span> Enroll at the Academy
      </button>
      <p className="font-sans" style={{ fontSize: '0.6rem', color: 'var(--ink-faint)', marginTop: '16px', letterSpacing: '0.1em', textTransform: 'uppercase' }}>
        No account required. Your progress lives on your device.
      </p>
    </div>
  );
}

function AcademyDashboard({ progress }) {
  const navigate = useNavigate();

  return (
    <div className="relative z-10" style={{ maxWidth: '680px', margin: '0 auto', padding: '3rem 1.5rem 4rem' }}>
      {/* Header */}
      <header style={{ textAlign: 'center', marginBottom: '2.5rem', animation: 'fadeInUp 0.8s var(--ease-out-expo) forwards' }}>
        <p className="font-sans tracking-wider" style={{ fontSize: '0.65rem', color: 'var(--gold-dim)', textTransform: 'uppercase', marginBottom: '8px' }}>
          Academy of The Great Game
        </p>
        <h1 className="font-heading" style={{ fontSize: 'clamp(2rem, 6vw, 3rem)', color: 'var(--ink-bright)', lineHeight: 1.1, marginBottom: '12px' }}>
          The Player's <span style={{ color: 'var(--gold)', fontStyle: 'italic' }}>Codex</span>
        </h1>
        <p className="font-serif" style={{ fontSize: 'clamp(0.85rem, 2vw, 1rem)', color: 'var(--ink-muted)', fontStyle: 'italic', maxWidth: '380px', margin: '0 auto', lineHeight: 1.6 }}>
          A metaphysical course in five classes. Complete the reflections. Unlock the truths.
        </p>
      </header>

      {/* Stats */}
      <div className="parchment-panel" style={{ marginBottom: '2rem' }}>
        <div className="flex items-center gap-3" style={{ marginBottom: '12px' }}>
          <div style={{
            width: '48px',
            height: '48px',
            borderRadius: '50%',
            border: '2px solid var(--gold)',
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            flexShrink: 0,
          }}>
            <span className="font-heading" style={{ fontSize: '1rem', color: 'var(--gold)' }}>
              {progress.overallPct}
            </span>
          </div>
          <div style={{ flex: 1 }}>
            <ProgressBar pct={progress.overallPct} label={`Academy Progress — ${progress.completedCount} / ${progress.totalLessons} lessons`} />
            <div className="flex items-center gap-3" style={{ marginTop: '6px' }}>
              <span className="font-sans" style={{ fontSize: '0.7rem', color: 'var(--gold)' }}>
                {progress.xp} XP
              </span>
              <span className="font-sans" style={{ fontSize: '0.7rem', color: 'var(--ink-faint)' }}>
                {progress.enrolled && progress.startedAt ? `Enrolled ${new Date(progress.startedAt).toLocaleDateString()}` : ''}
              </span>
            </div>
          </div>
        </div>
      </div>

      {/* Arcana Quick Ref */}
      <div className="parchment-panel" style={{ marginBottom: '2rem' }}>
        <h3 className="font-heading text-gold" style={{ fontSize: '0.9rem', marginBottom: '12px', textAlign: 'center' }}>
          The Twenty Words
        </h3>
        <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fill, minmax(130px, 1fr))', gap: '6px' }}>
          {ARCANA.map((word) => {
            const ch = word.channel.toLowerCase();
            const color = { mind: 'var(--mind)', heart: 'var(--heart)', body: 'var(--body)', action: 'var(--action)' }[ch] || 'var(--ink-muted)';
            return (
              <div key={word.word} style={{
                padding: '8px 10px',
                borderRadius: 'var(--radius-sm)',
                background: 'var(--void-raised)',
                border: `1px solid ${color}15`,
                display: 'flex',
                alignItems: 'center',
                gap: '6px',
              }}>
                <span style={{ color, fontSize: '0.8rem' }}>{word.symbol}</span>
                <div>
                  <div style={{ fontSize: '0.75rem', fontWeight: 600, color: 'var(--ink-primary)' }}>{word.word}</div>
                  <div style={{ fontSize: '0.55rem', color: 'var(--ink-muted)', textTransform: 'uppercase', letterSpacing: '0.1em' }}>{ch}</div>
                </div>
              </div>
            );
          })}
        </div>
      </div>

      {/* Classes / Tomes */}
      <div className="stagger-children">
        {TOMES.map((tome, i) => (
          <TomeCard key={tome.id} tome={tome} index={i} progress={progress} />
        ))}
      </div>

      {/* Completion CTA */}
      {progress.overallPct === 100 && (
        <div className="parchment-panel animate-fade-in-up" style={{ marginTop: '2rem', textAlign: 'center', border: '1px solid rgba(16,185,129,0.2)', background: 'rgba(16,185,129,0.04)' }}>
          <span style={{ fontSize: '2rem', color: 'var(--success)' }}>✦</span>
          <h3 className="font-heading" style={{ fontSize: '1.1rem', color: 'var(--success)', marginTop: '8px' }}>
            Academy Complete
          </h3>
          <p className="font-serif" style={{ fontSize: '0.9rem', color: 'var(--ink-secondary)', margin: '8px 0 16px', fontStyle: 'italic' }}>
            You have reflected upon every lesson. The mirror is now yours. Go, and walk what you have learned.
          </p>
          <button className="btn btn-primary" onClick={() => navigate('/create/character')}>
            <span>◆</span> Begin Your Journey
          </button>
        </div>
      )}

      {/* Footer */}
      <div style={{ textAlign: 'center', marginTop: '2rem' }}>
        <button className="btn btn-ghost" onClick={() => navigate('/')} style={{ fontSize: '0.75rem' }}>
          ← Return to the Threshold
        </button>
      </div>
    </div>
  );
}

export default function PlayerCodex() {
  const progress = useCodexProgress(TOMES);

  return (
    <div className="scrollable-page">
      <div className="starfield-bg" />
      {!progress.enrolled ? (
        <EnrollmentGate onEnroll={progress.enroll} />
      ) : (
        <AcademyDashboard progress={progress} />
      )}
    </div>
  );
}
