import { useState } from 'react';

// ════════════════════════════════════════════════════════════
// ReflectionModal — Socratic Lesson Completion
// ════════════════════════════════════════════════════════════
// Not a quiz. A mirror. The player reflects, and the reflection
// becomes part of their character sheet.

export default function ReflectionModal({ lesson, onComplete, onClose }) {
  const [reflection, setReflection] = useState('');
  const [submitted, setSubmitted] = useState(false);

  const handleSubmit = () => {
    if (!reflection.trim()) return;
    setSubmitted(true);
    // Persist reflection in character sheet (future: store in localStorage as trail entries)
    try {
      const key = 'daydream_reflections';
      const existing = JSON.parse(localStorage.getItem(key) || '[]');
      existing.push({
        lesson: lesson.heading,
        question: lesson.socraticQuestion,
        answer: reflection.trim(),
        at: new Date().toISOString(),
      });
      localStorage.setItem(key, JSON.stringify(existing.slice(-50))); // Keep last 50
    } catch {}
    onComplete();
  };

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center"
      style={{
        background: 'rgba(6,6,12,0.92)',
        backdropFilter: 'blur(12px)',
        padding: '16px',
      }}
      onClick={onClose}
    >
      <div
        className="parchment-panel"
        style={{
          maxWidth: '520px',
          width: '100%',
          maxHeight: '80vh',
          overflowY: 'auto',
          animation: 'fadeInUp 0.5s var(--ease-out-expo) forwards',
        }}
        onClick={(e) => e.stopPropagation()}
      >
        {/* Header */}
        <div style={{ textAlign: 'center', marginBottom: '20px' }}>
          <span style={{ fontSize: '1.2rem', color: 'var(--gold)' }}>◇</span>
          <h3
            className="font-heading"
            style={{ fontSize: '1rem', color: 'var(--gold)', marginTop: '8px', letterSpacing: '0.04em' }}
          >
            Socratic Reflection
          </h3>
          <p className="font-serif" style={{ fontSize: '0.8rem', color: 'var(--ink-muted)', fontStyle: 'italic', marginTop: '4px' }}>
            The mirror does not answer. It asks.
          </p>
        </div>

        {/* Question */}
        <div
          style={{
            padding: '16px',
            borderRadius: 'var(--radius-md)',
            background: 'var(--void-surface)',
            border: '1px solid rgba(201,168,76,0.1)',
            marginBottom: '16px',
          }}
        >
          <p className="font-serif" style={{ fontSize: '0.95rem', color: 'var(--ink-primary)', lineHeight: 1.7 }}>
            {lesson.socraticQuestion}
          </p>
        </div>

        {!submitted ? (
          <>
            <textarea
              className="font-serif"
              value={reflection}
              onChange={(e) => setReflection(e.target.value)}
              placeholder="Write your reflection here. There is no wrong answer. Only your truth."
              style={{
                width: '100%',
                minHeight: '120px',
                padding: '14px',
                borderRadius: 'var(--radius-md)',
                border: '1px solid rgba(201,168,76,0.15)',
                background: 'var(--void-raised)',
                color: 'var(--ink-primary)',
                fontSize: '0.9rem',
                lineHeight: 1.6,
                resize: 'vertical',
                outline: 'none',
                boxSizing: 'border-box',
                fontFamily: "'Cormorant Garamond', Georgia, serif",
              }}
            />
            <div className="flex items-center gap-2" style={{ marginTop: '16px', flexWrap: 'wrap' }}>
              <button
                className="btn btn-primary"
                onClick={handleSubmit}
                disabled={!reflection.trim()}
                style={{ opacity: reflection.trim() ? 1 : 0.4 }}
              >
                <span>✦</span> Commit to Memory
              </button>
              <button className="btn btn-ghost" onClick={onClose}>
                <span>✕</span> Close Without Saving
              </button>
            </div>
          </>
        ) : (
          <div style={{ textAlign: 'center', padding: '20px 0' }}>
            <span style={{ fontSize: '2rem', color: 'var(--success)' }}>✦</span>
            <h4
              className="font-heading"
              style={{ fontSize: '1rem', color: 'var(--success)', marginTop: '12px' }}
            >
              Reflection Recorded
            </h4>
            <p className="font-serif" style={{ fontSize: '0.85rem', color: 'var(--ink-muted)', marginTop: '8px', fontStyle: 'italic' }}>
              Your words are now part of your character's trail.
            </p>
            <button className="btn btn-primary" onClick={onClose} style={{ marginTop: '16px' }}>
              Continue
            </button>
          </div>
        )}
      </div>
    </div>
  );
}
