import { useState, useMemo } from 'react';
import { useNavigate, useLocation } from 'react-router-dom';
import { ARCANA, SYNERGIES, getChannelColor, getArcanaStageStars } from '../data/arcana';
import { Channel } from '../data/constants';

// ─── Styles ────────────────────────────────────────────────
const styles = {
  page: {
    minHeight: '100vh',
    background: 'linear-gradient(180deg, #0a0a0f 0%, #12121f 100%)',
    color: '#e2e8f0',
    fontFamily: "'Inter', system-ui, sans-serif",
    padding: '24px 16px',
    maxWidth: '900px',
    margin: '0 auto',
  },
  header: {
    marginBottom: '24px',
  },
  title: {
    fontFamily: "'Cormorant Garamond', serif",
    fontSize: '28px',
    fontWeight: 600,
    color: '#06b6d4',
    margin: '0 0 8px 0',
  },
  subtitle: {
    fontSize: '14px',
    color: '#94a3b8',
  },
  twoCol: {
    display: 'grid',
    gridTemplateColumns: '1fr 1fr',
    gap: '16px',
  },
  col: {
    minWidth: 0,
  },
  colTitle: {
    fontSize: '12px',
    textTransform: 'uppercase',
    letterSpacing: '2px',
    color: '#06b6d4',
    marginBottom: '12px',
    fontWeight: 600,
  },
  deckCount: {
    fontSize: '12px',
    color: '#94a3b8',
    marginLeft: '8px',
  },
  card: {
    background: 'rgba(26,26,46,0.8)',
    border: '1px solid rgba(255,255,255,0.08)',
    borderRadius: '12px',
    padding: '12px 14px',
    marginBottom: '8px',
    display: 'flex',
    alignItems: 'center',
    gap: '10px',
    transition: 'all 0.2s',
    position: 'relative',
  },
  cardGlow: {
    borderColor: 'rgba(6,182,212,0.4)',
    boxShadow: '0 0 12px rgba(6,182,212,0.15)',
  },
  symbol: {
    fontSize: '20px',
    lineHeight: 1,
    flexShrink: 0,
  },
  cardBody: {
    flex: 1,
    minWidth: 0,
  },
  wordName: {
    fontSize: '14px',
    fontWeight: 600,
    color: '#e2e8f0',
    marginBottom: '2px',
  },
  wordMeta: {
    fontSize: '11px',
    color: '#94a3b8',
    display: 'flex',
    alignItems: 'center',
    gap: '6px',
  },
  channelDot: {
    width: '8px',
    height: '8px',
    borderRadius: '50%',
    display: 'inline-block',
  },
  stars: {
    color: '#fbbf24',
    fontSize: '11px',
  },
  desc: {
    fontSize: '12px',
    color: '#64748b',
    marginTop: '4px',
    lineHeight: 1.4,
  },
  actionBtn: {
    width: '28px',
    height: '28px',
    borderRadius: '50%',
    border: 'none',
    cursor: 'pointer',
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
    fontSize: '18px',
    fontWeight: 700,
    flexShrink: 0,
  },
  addBtn: {
    background: 'rgba(6,182,212,0.2)',
    color: '#06b6d4',
  },
  removeBtn: {
    background: 'rgba(239,68,68,0.2)',
    color: '#ef4444',
  },
  disabledBtn: {
    opacity: 0.3,
    cursor: 'not-allowed',
  },
  synergyPanel: {
    background: 'rgba(6,182,212,0.08)',
    border: '1px solid rgba(6,182,212,0.2)',
    borderRadius: '16px',
    padding: '16px',
    marginBottom: '20px',
  },
  synergyTitle: {
    fontSize: '12px',
    textTransform: 'uppercase',
    letterSpacing: '2px',
    color: '#06b6d4',
    marginBottom: '10px',
  },
  synergyItem: {
    fontSize: '13px',
    color: '#e2e8f0',
    padding: '6px 0',
    borderBottom: '1px solid rgba(255,255,255,0.05)',
  },
  synergyName: {
    fontWeight: 600,
    color: '#22d3ee',
  },
  saveBtn: {
    width: '100%',
    padding: '18px',
    borderRadius: '16px',
    border: 'none',
    background: 'linear-gradient(135deg, #06b6d4, #0891b2)',
    color: '#fff',
    fontSize: '16px',
    fontWeight: 600,
    cursor: 'pointer',
    marginTop: '16px',
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
    marginTop: '12px',
  },
  emptyDeck: {
    textAlign: 'center',
    padding: '40px 20px',
    color: '#64748b',
    fontSize: '14px',
  },
  filterRow: {
    display: 'flex',
    gap: '6px',
    marginBottom: '12px',
    flexWrap: 'wrap',
  },
  filterBtn: {
    padding: '6px 12px',
    borderRadius: '20px',
    border: '1px solid rgba(255,255,255,0.1)',
    background: 'rgba(255,255,255,0.05)',
    color: '#94a3b8',
    fontSize: '12px',
    cursor: 'pointer',
    textTransform: 'capitalize',
  },
  filterActive: {
    borderColor: '#06b6d4',
    color: '#06b6d4',
    background: 'rgba(6,182,212,0.1)',
  },
};

export default function DeckBuilder({ characterHook }) {
  const navigate = useNavigate();
  const location = useLocation();
  const { getCharacter, setCharacterDeck } = characterHook;

  const characterId = location.state?.characterId;
  const character = characterId ? getCharacter(characterId) : null;

  const [deck, setDeck] = useState(character?.deck || []);
  const [filter, setFilter] = useState('all');

  const maxDeck = 20;

  const filteredArcana = useMemo(() => {
    if (filter === 'all') return ARCANA;
    return ARCANA.filter(a => a.channel === filter);
  }, [filter]);

  const activeSynergies = useMemo(() => {
    const set = new Set(deck);
    return SYNERGIES.filter(s => s.words.every(w => set.has(w)));
  }, [deck]);

  const glowingWords = useMemo(() => {
    const words = new Set();
    activeSynergies.forEach(s => s.words.forEach(w => words.add(w)));
    return words;
  }, [activeSynergies]);

  const addToDeck = (word) => {
    if (deck.length >= maxDeck || deck.includes(word)) return;
    setDeck(prev => [...prev, word]);
  };

  const removeFromDeck = (word) => {
    setDeck(prev => prev.filter(w => w !== word));
  };

  const handleSave = () => {
    if (characterId) {
      setCharacterDeck(characterId, deck);
    }
    navigate('/create/journey', { state: { characterId, deck } });
  };

  const filters = ['all', Channel.MIND, Channel.HEART, Channel.BODY, Channel.ACTION];
  const filterNames = { all: 'All', [Channel.MIND]: 'Mind', [Channel.HEART]: 'Heart', [Channel.BODY]: 'Body', [Channel.ACTION]: 'Action' };

  const ArcanaCard = ({ item, inDeck, onAction }) => (
    <div style={{ ...styles.card, ...(glowingWords.has(item.word) ? styles.cardGlow : {}) }}>
      <span style={styles.symbol}>{item.symbol}</span>
      <div style={styles.cardBody}>
        <div style={styles.wordName}>{item.word}</div>
        <div style={styles.wordMeta}>
          <span style={{ ...styles.channelDot, background: getChannelColor(item.channel) }} />
          <span>{item.channel}</span>
          <span style={styles.stars}>{getArcanaStageStars(item.stage)}</span>
        </div>
        <div style={styles.desc}>{item.desc}</div>
      </div>
      <button
        style={{ ...styles.actionBtn, ...(inDeck ? styles.removeBtn : styles.addBtn), ...(deck.length >= maxDeck && !inDeck ? styles.disabledBtn : {}) }}
        onClick={onAction}
        disabled={!inDeck && deck.length >= maxDeck}
      >
        {inDeck ? '−' : '+'}
      </button>
    </div>
  );

  return (
    <div style={styles.page}>
      <div style={styles.header}>
        <h1 style={styles.title}>Build Your Spell Deck</h1>
        <p style={styles.subtitle}>
          Choose {maxDeck} words that define your journey.
          {character ? ` — ${character.name}` : ''}
        </p>
      </div>

      {/* ─── SYNERGIES ──────────────────────────────────── */}
      {activeSynergies.length > 0 && (
        <div style={styles.synergyPanel}>
          <div style={styles.synergyTitle}>Active Resonance</div>
          {activeSynergies.map(s => (
            <div key={s.name} style={styles.synergyItem}>
              <span style={styles.synergyName}>{s.name}</span> (+{s.bonus}):
              {' '}{s.words.join(' + ')} — {s.desc}
            </div>
          ))}
        </div>
      )}

      <div style={styles.twoCol}>
        {/* ─── LIBRARY ──────────────────────────────────── */}
        <div style={styles.col}>
          <div style={styles.colTitle}>
            ARCANA Library
            <span style={styles.deckCount}>({ARCANA.length} words)</span>
          </div>
          <div style={styles.filterRow}>
            {filters.map(f => (
              <button
                key={f}
                style={{ ...styles.filterBtn, ...(filter === f ? styles.filterActive : {}) }}
                onClick={() => setFilter(f)}
              >
                {filterNames[f]}
              </button>
            ))}
          </div>
          {filteredArcana.map(item => (
            <ArcanaCard
              key={item.word}
              item={item}
              inDeck={deck.includes(item.word)}
              onAction={() => deck.includes(item.word) ? removeFromDeck(item.word) : addToDeck(item.word)}
            />
          ))}
        </div>

        {/* ─── DECK ─────────────────────────────────────── */}
        <div style={styles.col}>
          <div style={styles.colTitle}>
            Your Deck
            <span style={styles.deckCount}>({deck.length}/{maxDeck})</span>
          </div>
          {deck.length === 0 ? (
            <div style={styles.emptyDeck}>
              No spells yet.<br />
              Tap + to add words from the library.
            </div>
          ) : (
            deck.map(word => {
              const item = ARCANA.find(a => a.word === word);
              return item ? (
                <ArcanaCard
                  key={word}
                  item={item}
                  inDeck={true}
                  onAction={() => removeFromDeck(word)}
                />
              ) : null;
            })
          )}
        </div>
      </div>

      <button style={styles.saveBtn} onClick={handleSave}>
        Save Deck → Author Journey
      </button>
      <button style={styles.backBtn} onClick={() => navigate(-1)}>
        Back
      </button>
    </div>
  );
}
