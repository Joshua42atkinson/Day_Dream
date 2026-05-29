import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { Archetypes } from '../data/arcana';
import { Channel, CHANNEL_COLORS } from '../data/constants';

// ─── Styles ────────────────────────────────────────────────
const styles = {
  page: {
    minHeight: '100vh',
    background: 'linear-gradient(180deg, #0a0a0f 0%, #12121f 100%)',
    color: '#e2e8f0',
    fontFamily: "'Inter', system-ui, sans-serif",
    padding: '24px 16px',
    maxWidth: '600px',
    margin: '0 auto',
  },
  header: {
    marginBottom: '32px',
  },
  title: {
    fontFamily: "'Cormorant Garamond', serif",
    fontSize: '32px',
    fontWeight: 600,
    color: '#06b6d4',
    margin: '0 0 8px 0',
  },
  subtitle: {
    fontSize: '14px',
    color: '#94a3b8',
  },
  label: {
    fontSize: '12px',
    textTransform: 'uppercase',
    letterSpacing: '2px',
    color: '#06b6d4',
    marginBottom: '12px',
    fontWeight: 600,
    display: 'block',
  },
  input: {
    width: '100%',
    padding: '14px 16px',
    borderRadius: '12px',
    border: '1px solid rgba(255,255,255,0.1)',
    background: 'rgba(255,255,255,0.05)',
    color: '#e2e8f0',
    fontSize: '16px',
    fontFamily: "'Inter', sans-serif",
    outline: 'none',
    boxSizing: 'border-box',
  },
  archetypeGrid: {
    display: 'grid',
    gridTemplateColumns: 'repeat(2, 1fr)',
    gap: '10px',
    marginBottom: '24px',
  },
  archetypeCard: {
    background: 'rgba(255,255,255,0.05)',
    border: '1px solid rgba(255,255,255,0.1)',
    borderRadius: '16px',
    padding: '16px',
    cursor: 'pointer',
    textAlign: 'left',
    transition: 'all 0.2s',
  },
  archetypeActive: {
    borderColor: '#06b6d4',
    background: 'rgba(6,182,212,0.1)',
  },
  archetypeName: {
    fontSize: '16px',
    fontWeight: 600,
    color: '#e2e8f0',
    marginBottom: '4px',
  },
  archetypeChannel: {
    fontSize: '12px',
    color: '#94a3b8',
    marginBottom: '8px',
  },
  archetypeDesc: {
    fontSize: '13px',
    color: '#94a3b8',
    lineHeight: 1.5,
  },
  channelBadge: {
    display: 'inline-block',
    padding: '2px 8px',
    borderRadius: '6px',
    fontSize: '11px',
    fontWeight: 600,
    textTransform: 'uppercase',
    letterSpacing: '1px',
  },
  sliderBlock: {
    marginBottom: '16px',
  },
  sliderHeader: {
    display: 'flex',
    justifyContent: 'space-between',
    alignItems: 'center',
    marginBottom: '8px',
  },
  sliderLabel: {
    fontSize: '14px',
    color: '#e2e8f0',
    display: 'flex',
    alignItems: 'center',
    gap: '8px',
  },
  sliderValue: {
    fontSize: '14px',
    color: '#06b6d4',
    fontWeight: 600,
    fontVariantNumeric: 'tabular-nums',
  },
  slider: {
    width: '100%',
    accentColor: '#06b6d4',
  },
  preview: {
    background: 'rgba(6,182,212,0.08)',
    border: '1px solid rgba(6,182,212,0.2)',
    borderRadius: '16px',
    padding: '20px',
    marginTop: '24px',
    marginBottom: '24px',
  },
  previewTitle: {
    fontSize: '12px',
    textTransform: 'uppercase',
    letterSpacing: '2px',
    color: '#06b6d4',
    marginBottom: '12px',
  },
  className: {
    fontFamily: "'Cormorant Garamond', serif",
    fontSize: '24px',
    color: '#e2e8f0',
    marginBottom: '8px',
  },
  classDesc: {
    fontSize: '14px',
    color: '#94a3b8',
    lineHeight: 1.6,
  },
  attunementBars: {
    marginTop: '16px',
  },
  barRow: {
    display: 'flex',
    alignItems: 'center',
    gap: '8px',
    marginBottom: '8px',
  },
  barLabel: {
    width: '50px',
    fontSize: '12px',
    color: '#94a3b8',
    textTransform: 'capitalize',
  },
  barTrack: {
    flex: 1,
    height: '8px',
    background: 'rgba(255,255,255,0.1)',
    borderRadius: '4px',
    overflow: 'hidden',
  },
  barFill: {
    height: '100%',
    borderRadius: '4px',
    transition: 'width 0.3s ease',
  },
  createBtn: {
    width: '100%',
    padding: '18px',
    borderRadius: '16px',
    border: 'none',
    background: 'linear-gradient(135deg, #06b6d4, #0891b2)',
    color: '#fff',
    fontSize: '16px',
    fontWeight: 600,
    cursor: 'pointer',
    marginBottom: '16px',
  },
  backBtn: {
    width: '100%',
    padding: '14px',
    borderRadius: '16px',
    border: '1px solid rgba(255,255,255,0.2)',
    background: 'transparent',
    color: '#94a3b8',
    fontSize: '14px',
    cursor: 'pointer',
  },
};

export default function CharacterCreator({ characterHook }) {
  const navigate = useNavigate();
  const { createCharacter, deriveEmergentClass } = characterHook;

  const [name, setName] = useState('');
  const [selectedArchetype, setSelectedArchetype] = useState('ARCHITECT');
  const [attunement, setAttunement] = useState({
    mind: 55,
    heart: 55,
    body: 55,
    action: 55,
  });

  const archetype = Archetypes[selectedArchetype];

  const selectArchetype = (key) => {
    setSelectedArchetype(key);
    const a = Archetypes[key];
    if (!a) return;
    const dominant = a.dominant || 0.6;
    const others = a.others || 0.55;
    const map = {
      ORACLE:     { mind: 80, heart: 40, body: 40, action: 40 },
      BARD:       { mind: 40, heart: 80, body: 40, action: 40 },
      CULTIVATOR: { mind: 40, heart: 40, body: 80, action: 40 },
      TEMPLAR:    { mind: 40, heart: 40, body: 40, action: 80 },
      ARCHITECT:  { mind: 55, heart: 55, body: 55, action: 55 },
    };
    setAttunement(map[key] || { mind: 55, heart: 55, body: 55, action: 55 });
  };

  const updateAttunement = (key, value) => {
    setAttunement(prev => ({ ...prev, [key]: parseInt(value) }));
  };

  const handleCreate = () => {
    const char = createCharacter({
      name,
      archetypeKey: selectedArchetype,
      attunement,
      deck: [], // User builds deck next
    });
    navigate('/create/deck', { state: { characterId: char.id } });
  };

  const previewChar = {
    name: name.trim() || 'Wanderer',
    archetype: selectedArchetype,
    attunement: {
      [Channel.MIND]: attunement.mind / 100,
      [Channel.HEART]: attunement.heart / 100,
      [Channel.BODY]: attunement.body / 100,
      [Channel.ACTION]: attunement.action / 100,
    },
  };
  const emergentClass = deriveEmergentClass(previewChar);

  const channelOrder = [Channel.MIND, Channel.HEART, Channel.BODY, Channel.ACTION];
  const channelNames = { [Channel.MIND]: 'Mind', [Channel.HEART]: 'Heart', [Channel.BODY]: 'Body', [Channel.ACTION]: 'Action' };

  return (
    <div style={styles.page}>
      <div style={styles.header}>
        <h1 style={styles.title}>Create Your Character</h1>
        <p style={styles.subtitle}>You are the instrument playing the instrument.</p>
      </div>

      {/* ─── NAME ───────────────────────────────────────── */}
      <label style={styles.label}>Your Name</label>
      <input
        style={styles.input}
        value={name}
        onChange={e => setName(e.target.value)}
        placeholder="Enter your name..."
        maxLength={30}
      />

      {/* ─── ARCHETYPE ──────────────────────────────────── */}
      <label style={{ ...styles.label, marginTop: '24px' }}>Choose Your Archetype</label>
      <div style={styles.archetypeGrid}>
        {Object.entries(Archetypes).map(([key, a]) => (
          <button
            key={key}
            style={{ ...styles.archetypeCard, ...(selectedArchetype === key ? styles.archetypeActive : {}) }}
            onClick={() => selectArchetype(key)}
          >
            <div style={styles.archetypeName}>{a.name}</div>
            <div style={styles.archetypeChannel}>
              <span style={{ ...styles.channelBadge, background: a.channel ? CHANNEL_COLORS[a.channel]?.bg : 'rgba(255,255,255,0.1)', color: a.channel ? CHANNEL_COLORS[a.channel]?.primary : '#94a3b8' }}>
                {a.channel || 'Balanced'}
              </span>
            </div>
            <div style={styles.archetypeDesc}>{a.desc}</div>
          </button>
        ))}
      </div>

      {/* ─── ATTUNEMENT SLIDERS ─────────────────────────── */}
      <label style={styles.label}>Channel Attunement</label>
      {channelOrder.map(ch => (
        <div key={ch} style={styles.sliderBlock}>
          <div style={styles.sliderHeader}>
            <span style={{ ...styles.sliderLabel, color: CHANNEL_COLORS[ch]?.primary }}>
              {channelNames[ch]}
            </span>
            <span style={styles.sliderValue}>{attunement[ch]}%</span>
          </div>
          <input
            type="range" min="0" max="100" step="5"
            value={attunement[ch]}
            onChange={e => updateAttunement(ch, e.target.value)}
            style={{ ...styles.slider, accentColor: CHANNEL_COLORS[ch]?.primary }}
          />
        </div>
      ))}

      {/* ─── PREVIEW ────────────────────────────────────── */}
      <div style={styles.preview}>
        <div style={styles.previewTitle}>Emergent Class</div>
        <div style={styles.className}>{emergentClass}</div>
        <div style={styles.classDesc}>
          {archetype?.desc}
        </div>
        <div style={styles.attunementBars}>
          {channelOrder.map(ch => (
            <div key={ch} style={styles.barRow}>
              <span style={styles.barLabel}>{channelNames[ch]}</span>
              <div style={styles.barTrack}>
                <div style={{
                  ...styles.barFill,
                  width: `${attunement[ch]}%`,
                  background: CHANNEL_COLORS[ch]?.primary,
                  opacity: 0.8,
                }} />
              </div>
            </div>
          ))}
        </div>
      </div>

      {/* ─── ACTIONS ────────────────────────────────────── */}
      <button style={styles.createBtn} onClick={handleCreate}>
        Create Character → Build Deck
      </button>
      <button style={styles.backBtn} onClick={() => navigate(-1)}>
        Back
      </button>
    </div>
  );
}
