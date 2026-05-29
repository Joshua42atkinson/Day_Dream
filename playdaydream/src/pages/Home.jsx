import { useNavigate } from 'react-router-dom';
import { useRef, useState } from 'react';
import { getAllAdventures } from '../data/curriculum';
import { adaptStoryGraph } from '../adapters/storygraph';
import { useCharacter } from '../hooks/useCharacter';
import { useSettings } from '../hooks/useSettings';
import './Home.css';

// ════════════════════════════════════════════════════════════
// HOME — The Threshold (Landing Page)
// ════════════════════════════════════════════════════════════
// Cinematic hero + character preview + adventure cards.
// LitRPG themed: "You stand at the threshold. What will you become?"

function CharacterPreview() {
  const navigate = useNavigate();
  const { characters } = useCharacter();
  const { settings } = useSettings();
  const activeId = settings.activeCharacterId;
  const char = activeId ? characters.find((c) => c.id === activeId) : null;

  if (!char) return null;

  const dominant = Object.entries(char.attunement || {}).sort((a, b) => b[1] - a[1])[0];
  const domChannel = dominant?.[0] || 'mind';
  const domValue = dominant?.[1] || 0;

  const channelColors = {
    mind: { color: '#06b6d4', glow: 'rgba(6,182,212,0.15)' },
    heart: { color: '#f472b6', glow: 'rgba(244,114,182,0.15)' },
    body: { color: '#10b981', glow: 'rgba(16,185,129,0.15)' },
    action: { color: '#f59e0b', glow: 'rgba(245,158,11,0.15)' },
  };
  const cc = channelColors[domChannel] || channelColors.mind;

  return (
    <div
      className="animate-fade-in-up"
      style={{
        animationDelay: '0.3s',
        background: 'linear-gradient(135deg, var(--void-surface), var(--void))',
        border: `1px solid ${cc.color}30`,
        borderRadius: 'var(--radius-lg)',
        padding: '20px',
        marginBottom: '28px',
        display: 'flex',
        alignItems: 'center',
        gap: '16px',
        cursor: 'pointer',
        transition: 'all 0.3s var(--ease-out-expo)',
      }}
      onClick={() => navigate('/create/character')}
    >
      {/* Orb avatar */}
      <div
        className="animate-orb-pulse"
        style={{
          width: '64px',
          height: '64px',
          borderRadius: '50%',
          background: `radial-gradient(circle, ${cc.glow} 0%, transparent 70%)`,
          border: `1px solid ${cc.color}40`,
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'center',
          flexShrink: 0,
        }}
      >
        <span className="font-heading" style={{ fontSize: '1.6rem', color: cc.color }}>
          {char.archetypeIcon || '✦'}
        </span>
      </div>

      <div style={{ flex: 1, minWidth: 0 }}>
        <div className="flex items-center gap-2" style={{ marginBottom: '4px' }}>
          <span className="font-heading" style={{ fontSize: '0.95rem', color: 'var(--ink-primary)' }}>
            {char.name}
          </span>
          <span
            className="level-badge"
            style={{ fontSize: '0.6rem', minWidth: '20px', height: '20px', padding: '0 6px' }}
          >
            Lv.{char.level || 1}
          </span>
        </div>
        <div style={{ fontSize: '0.75rem', color: 'var(--ink-muted)', marginBottom: '6px' }}>
          {char.emergentClass || char.archetypeName || 'Wanderer'}
        </div>
        {/* Attunement mini-bars */}
        <div className="flex items-center gap-2">
          {Object.entries(char.attunement || {}).map(([ch, val]) => (
            <div key={ch} style={{ display: 'flex', alignItems: 'center', gap: '3px' }}>
              <div
                style={{
                  width: '20px',
                  height: '3px',
                  borderRadius: '2px',
                  background: 'var(--void-raised)',
                  overflow: 'hidden',
                }}
              >
                <div
                  style={{
                    width: `${val * 100}%`,
                    height: '100%',
                    background: channelColors[ch]?.color || '#64748b',
                    borderRadius: '2px',
                    transition: 'width 0.6s var(--ease-out-expo)',
                  }}
                />
              </div>
            </div>
          ))}
        </div>
      </div>

      <div style={{ fontSize: '0.75rem', color: 'var(--ink-faint)', flexShrink: 0 }}>
        {char.deck?.length || 0} ◆
      </div>
    </div>
  );
}

export default function Home() {
  const navigate = useNavigate();
  const adventures = getAllAdventures();
  const fileInputRef = useRef(null);
  const [uploadError, setUploadError] = useState(null);

  const handleFileSelect = (e) => {
    const file = e.target.files?.[0];
    if (!file) return;
    setUploadError(null);
    const reader = new FileReader();
    reader.onload = (ev) => {
      try {
        const json = JSON.parse(ev.target.result);
        const adventure = adaptStoryGraph(json);
        sessionStorage.setItem('daydream_custom_adventure', JSON.stringify(adventure));
        navigate('/custom');
      } catch (err) {
        setUploadError(err.message || 'Invalid StoryGraph JSON');
      }
    };
    reader.readAsText(file);
  };

  return (
    <div className="scrollable-page">
      {/* Starfield background */}
      <div className="starfield-bg" />
      <div className="hero-glow" />

      <div className="relative z-10" style={{ maxWidth: '720px', margin: '0 auto', padding: '3rem 1.5rem 2rem' }}>
        {/* ── CINEMATIC HERO ─────────────────────────── */}
        <header style={{ textAlign: 'center', marginBottom: '2.5rem' }}>
          <div className="animate-fade-in">
            <p
              className="font-sans tracking-wider"
              style={{
                fontSize: '0.6rem',
                color: 'var(--gold-dim)',
                textTransform: 'uppercase',
                letterSpacing: '0.35em',
                marginBottom: '16px',
              }}
            >
              A Sovereign LitRPG Experience
            </p>
          </div>

          <h1
            className="font-heading animate-fade-in-up"
            style={{
              fontSize: 'clamp(2.5rem, 8vw, 4.5rem)',
              fontWeight: 400,
              color: 'var(--ink-bright)',
              lineHeight: 1.05,
              letterSpacing: '0.02em',
              marginBottom: '12px',
              animationDelay: '0.1s',
            }}
          >
            Day<span style={{ color: 'var(--gold)', fontStyle: 'italic' }}>dream</span>
          </h1>

          <p
            className="font-serif animate-fade-in-up"
            style={{
              fontSize: 'clamp(0.95rem, 2.5vw, 1.2rem)',
              fontStyle: 'italic',
              color: 'var(--ink-muted)',
              lineHeight: 1.6,
              maxWidth: '26rem',
              margin: '0 auto 20px',
              animationDelay: '0.2s',
            }}
          >
            Where words are spells and the path you choose is the lesson.
          </p>

          {/* Symbol divider */}
          <div className="symbol-divider animate-fade-in" style={{ animationDelay: '0.3s' }}>
            <span>◆ ◇ △ ○ ☆</span>
          </div>
        </header>

        {/* ── CHARACTER PREVIEW ──────────────────────── */}
        <CharacterPreview />

        {/* ── THE GREAT GAME ACTIONS ─────────────────── */}
        <div
          className="animate-fade-in-up"
          style={{
            animationDelay: '0.4s',
            display: 'grid',
            gridTemplateColumns: 'repeat(2, 1fr)',
            gap: '10px',
            marginBottom: '32px',
          }}
        >
          {[
            { label: 'Create Character', icon: '✦', path: '/create/character', color: '#06b6d4', desc: 'Forge your identity' },
            { label: 'Build Deck', icon: '◈', path: '/create/deck', color: '#a855f7', desc: 'Choose your words' },
            { label: 'Author Journey', icon: '✎', path: '/create/journey', color: '#22d3ee', desc: 'Write the path' },
            { label: 'Codex', icon: '☆', path: '/codex', color: '#c9a84c', desc: 'Learn the rules' },
          ].map((btn) => (
            <button
              key={btn.path}
              onClick={() => navigate(btn.path)}
              style={{
                padding: '16px 12px',
                borderRadius: 'var(--radius-lg)',
                border: `1px solid ${color}20`,
                background: 'linear-gradient(135deg, var(--void-surface), var(--void))',
                color: 'var(--ink-primary)',
                cursor: 'pointer',
                textAlign: 'left',
                transition: 'all 0.3s var(--ease-out-expo)',
                display: 'flex',
                flexDirection: 'column',
                gap: '6px',
              }}
              onMouseEnter={(e) => {
                e.currentTarget.style.borderColor = `${btn.color}50`;
                e.currentTarget.style.transform = 'translateY(-2px)';
                e.currentTarget.style.boxShadow = `0 8px 24px ${btn.color}10`;
              }}
              onMouseLeave={(e) => {
                e.currentTarget.style.borderColor = `${btn.color}20`;
                e.currentTarget.style.transform = 'translateY(0)';
                e.currentTarget.style.boxShadow = 'none';
              }}
            >
              <span style={{ fontSize: '1.3rem' }}>{btn.icon}</span>
              <span style={{ fontSize: '0.85rem', fontWeight: 600, letterSpacing: '0.02em' }}>{btn.label}</span>
              <span style={{ fontSize: '0.65rem', color: 'var(--ink-muted)' }}>{btn.desc}</span>
            </button>
          ))}
        </div>

        {/* ── ADVENTURE CARDS ────────────────────────── */}
        <div
          className="animate-fade-in-up"
          style={{ animationDelay: '0.5s', marginBottom: '8px' }}
        >
          <div className="flex items-center gap-2" style={{ marginBottom: '16px' }}>
            <span style={{ fontSize: '0.7rem', color: 'var(--gold-dim)' }}>◇</span>
            <span
              className="font-sans tracking-wide"
              style={{ fontSize: '0.65rem', color: 'var(--ink-muted)', textTransform: 'uppercase' }}
            >
              Available Journeys
            </span>
          </div>
        </div>

        <div className="adventure-grid" style={{ marginBottom: '2rem' }}>
          {adventures.map((adv, i) => {
            const firstNode = adv.nodes[adv.start];
            const previewImage = firstNode?.image || '/images/threshold.png';
            const wordList = Object.values(adv.nodes)
              .map((n) => n.focusWord)
              .filter(Boolean);
            const nodeCount = Object.keys(adv.nodes).length;

            return (
              <button
                key={adv.id}
                className="adventure-card animate-fade-in-up"
                style={{ animationDelay: `${0.6 + i * 0.1}s` }}
                onClick={() => navigate(`/play/${adv.id}`)}
              >
                <div
                  className="adventure-card-bg"
                  style={{ backgroundImage: `url('${previewImage}')` }}
                />
                <div className="adventure-card-overlay" />
                <div className="adventure-card-content">
                  <div className="flex items-center gap-2" style={{ marginBottom: '8px' }}>
                    {adv.ageRange && (
                      <span className="adventure-age">{adv.ageRange}</span>
                    )}
                    <span
                      style={{
                        fontSize: '0.6rem',
                        color: 'var(--ink-muted)',
                        fontFamily: 'var(--font-sans)',
                      }}
                    >
                      {nodeCount} nodes
                    </span>
                  </div>
                  <h2 className="adventure-card-title">{adv.title}</h2>
                  <p className="adventure-card-desc">{adv.description}</p>
                  <div className="adventure-words">
                    {wordList.map((w) => (
                      <span key={w} className="spell-tag">
                        {w}
                      </span>
                    ))}
                  </div>
                  <button
                    className="audio-play-btn"
                    onClick={(e) => {
                      e.stopPropagation();
                      navigate(`/audio/${adv.id}`);
                    }}
                    style={{
                      marginTop: '12px',
                      padding: '8px 16px',
                      borderRadius: '8px',
                      border: '1px solid rgba(201,168,76,0.25)',
                      background: 'rgba(201,168,76,0.08)',
                      color: 'var(--gold)',
                      fontSize: '0.75rem',
                      fontFamily: 'var(--font-sans)',
                      fontWeight: 500,
                      cursor: 'pointer',
                      display: 'flex',
                      alignItems: 'center',
                      gap: '6px',
                      transition: 'all 0.2s ease',
                    }}
                    onMouseEnter={(e) => {
                      e.currentTarget.style.background = 'rgba(201,168,76,0.15)';
                      e.currentTarget.style.borderColor = 'rgba(201,168,76,0.4)';
                    }}
                    onMouseLeave={(e) => {
                      e.currentTarget.style.background = 'rgba(201,168,76,0.08)';
                      e.currentTarget.style.borderColor = 'rgba(201,168,76,0.25)';
                    }}
                  >
                    🎧 Oral Tradition
                  </button>
                </div>
              </button>
            );
          })}
        </div>

        {/* ── UPLOAD SECTION ─────────────────────────── */}
        <div className="upload-section animate-fade-in-up" style={{ animationDelay: '0.9s' }}>
          <p className="upload-label">Made your own journey?</p>
          <button className="upload-btn" onClick={() => fileInputRef.current?.click()}>
            Load StoryGraph JSON
          </button>
          <input
            ref={fileInputRef}
            type="file"
            accept=".json,application/json"
            style={{ display: 'none' }}
            onChange={handleFileSelect}
          />
          {uploadError && <p className="upload-error">{uploadError}</p>}
        </div>

        {/* ── FOOTER ─────────────────────────────────── */}
        <footer className="home-footer animate-fade-in" style={{ animationDelay: '1s' }}>
          <p className="font-sans" style={{ fontSize: '0.65rem', color: 'var(--ink-faint)' }}>
            No accounts. No tracking. Your adventure stays on your device. <span style={{ color: 'var(--gold-dim)' }}>◆</span>
          </p>
        </footer>
      </div>
    </div>
  );
}
