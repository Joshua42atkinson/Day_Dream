import { useState, useEffect, useCallback } from 'react';
import { useNavigate } from 'react-router-dom';
import { useAudioOrchestrator } from '../hooks/useAudioOrchestrator';
import { useSettings } from '../hooks/useSettings';
import { useStorytellerAI } from '../hooks/useStorytellerAI';
import { useVoiceCommands } from '../hooks/useVoiceCommands';
import { getAdventure } from '../data/curriculum';
import { useCharacter } from '../hooks/useCharacter';
import { AmbientMoods } from '../data/arcana';

// ─── Styles ────────────────────────────────────────────────
const styles = {
  page: {
    minHeight: '100vh',
    background: '#0a0a0f',
    color: '#e2e8f0',
    fontFamily: "'Inter', system-ui, sans-serif",
    display: 'flex',
    flexDirection: 'column',
    alignItems: 'center',
    justifyContent: 'center',
    padding: '24px',
    position: 'relative',
    overflow: 'hidden',
  },
  brand: {
    position: 'absolute',
    top: '16px',
    left: '50%',
    transform: 'translateX(-50%)',
    textAlign: 'center',
  },
  brandTitle: {
    fontFamily: "'Cormorant Garamond', serif",
    fontSize: '14px',
    color: '#06b6d4',
    letterSpacing: '3px',
    textTransform: 'uppercase',
    margin: 0,
  },
  brandSubtitle: {
    fontSize: '10px',
    color: '#64748b',
    letterSpacing: '2px',
    textTransform: 'uppercase',
    margin: '4px 0 0 0',
  },
  orb: {
    width: '180px',
    height: '180px',
    borderRadius: '50%',
    background: 'radial-gradient(circle, rgba(6,182,212,0.25) 0%, rgba(6,182,212,0.05) 60%, transparent 80%)',
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
    position: 'relative',
    animation: 'pulse 4s ease-in-out infinite',
    cursor: 'pointer',
  },
  orbInner: {
    width: '100px',
    height: '100px',
    borderRadius: '50%',
    background: 'radial-gradient(circle, rgba(6,182,212,0.4) 0%, rgba(6,182,212,0.1) 70%, transparent 100%)',
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
    fontSize: '40px',
  },
  orbLabel: {
    position: 'absolute',
    bottom: '-28px',
    fontSize: '11px',
    color: '#64748b',
    letterSpacing: '1px',
  },
  statusText: {
    marginTop: '48px',
    fontSize: '16px',
    color: '#e2e8f0',
    textAlign: 'center',
    maxWidth: '380px',
    lineHeight: 1.7,
    minHeight: '80px',
    fontStyle: 'italic',
  },
  statusDot: {
    display: 'inline-block',
    width: '8px',
    height: '8px',
    borderRadius: '50%',
    marginRight: '8px',
  },
  controls: {
    position: 'absolute',
    bottom: '32px',
    display: 'flex',
    gap: '16px',
    alignItems: 'center',
    flexWrap: 'wrap',
    justifyContent: 'center',
  },
  ctrlBtn: {
    width: '52px',
    height: '52px',
    borderRadius: '50%',
    border: '2px solid rgba(6,182,212,0.3)',
    background: 'rgba(6,182,212,0.08)',
    color: '#06b6d4',
    fontSize: '20px',
    cursor: 'pointer',
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
    transition: 'all 0.2s',
  },
  ctrlBtnActive: {
    borderColor: '#06b6d4',
    background: 'rgba(6,182,212,0.2)',
  },
  backBtn: {
    position: 'absolute',
    top: '16px',
    left: '16px',
    background: 'none',
    border: '1px solid rgba(255,255,255,0.15)',
    color: '#94a3b8',
    borderRadius: '50%',
    width: '36px',
    height: '36px',
    cursor: 'pointer',
    fontSize: '16px',
  },
  wordBadge: {
    position: 'absolute',
    top: '16px',
    right: '16px',
    padding: '6px 12px',
    borderRadius: '20px',
    background: 'rgba(6,182,212,0.12)',
    border: '1px solid rgba(6,182,212,0.25)',
    color: '#06b6d4',
    fontSize: '12px',
    fontWeight: 600,
  },
  backendBadge: {
    position: 'absolute',
    bottom: '100px',
    padding: '4px 10px',
    borderRadius: '12px',
    fontSize: '10px',
    letterSpacing: '1px',
    textTransform: 'uppercase',
  },
  hint: {
    position: 'absolute',
    bottom: '96px',
    fontSize: '11px',
    color: '#475569',
    textAlign: 'center',
    maxWidth: '340px',
    lineHeight: 1.5,
  },
  transcript: {
    position: 'absolute',
    top: '50%',
    left: '50%',
    transform: 'translate(-50%, -50%)',
    background: 'rgba(10,10,15,0.95)',
    border: '1px solid rgba(6,182,212,0.3)',
    borderRadius: '16px',
    padding: '20px 24px',
    color: '#06b6d4',
    fontSize: '16px',
    textAlign: 'center',
    maxWidth: '300px',
    zIndex: 10,
  },
};

const BACKEND_COLORS = {
  stepaudio: '#22d3ee',
  lmstudio: '#a855f7',
  offline: '#64748b',
  fallback: '#ef4444',
};

const BACKEND_NAMES = {
  stepaudio: 'StepAudio R1.1',
  lmstudio: 'LM Studio',
  offline: 'Offline',
  fallback: 'Fallback',
};

// ─── Key Commands ──────────────────────────────────────────
const KEY_MAP = {
  ' ': 'pause',
  'ArrowUp': 'choice_0',
  'ArrowRight': 'choice_1',
  'ArrowDown': 'choice_2',
  'ArrowLeft': 'choice_3',
  'd': 'depth',
  's': 'skip',
  'r': 'repeat',
};

export default function AudioPlay({ adventureId, adventure: propAdventure, onBack }) {
  const navigate = useNavigate();
  const settingsHook = useSettings();
  const characterHook = useCharacter();
  const audio = useAudioOrchestrator(settingsHook.settings);
  const storyteller = useStorytellerAI();

  const [adventure, setAdventure] = useState(() => {
    if (propAdventure) return propAdventure;
    if (adventureId) return getAdventure(adventureId);
    return null;
  });

  const [currentNodeId, setCurrentNodeId] = useState(adventure?.start || '');
  const [status, setStatus] = useState('Press space to begin your oral tradition...');
  const [isPlaying, setIsPlaying] = useState(false);
  const [history, setHistory] = useState('');
  const [showChoices, setShowChoices] = useState(false);
  const [currentNarration, setCurrentNarration] = useState('');
  const [narrationSource, setNarrationSource] = useState('');

  const currentNode = adventure?.nodes?.[currentNodeId];
  const choices = currentNode?.exits ? Object.entries(currentNode.exits).filter(([_, v]) => v) : [];

  // Load active character from settings
  const activeChar = settingsHook.settings.activeCharacterId
    ? characterHook.getCharacter(settingsHook.settings.activeCharacterId)
    : null;
  const activeDeck = activeChar?.deck || [];

  // Probe storyteller AI on mount
  useEffect(() => {
    storyteller.probe();
  }, []);

  // Initialize audio
  useEffect(() => {
    if (!adventure) return;
    const mood = AmbientMoods[settingsHook.settings.ambientMood];
    audio.playMood(mood?.id || 'cave');
    return () => {
      audio.fadeMood();
      audio.stopSpeaking();
    };
  }, [adventure]);

  // ─── Narrate current scene ─────────────────────────────────
  const narrateScene = useCallback(async () => {
    if (!currentNode) return;
    setIsPlaying(true);
    setShowChoices(false);

    // Get AI-narrated or static story
    const result = await storyteller.narrate({
      node: currentNode,
      character: activeChar,
      deck: activeDeck,
      mood: AmbientMoods[settingsHook.settings.ambientMood]?.name,
      history,
    });

    setCurrentNarration(result.text);
    setNarrationSource(result.source);
    setStatus(result.text);

    // Speak it
    await audio.speak(result.text);

    // Show choices if any
    const exits = Object.values(currentNode.exits || {}).filter(v => v);
    if (exits.length > 0) {
      setShowChoices(true);
      const choiceText = exits.map((c, i) => `${['One', 'Two', 'Three', 'Four'][i] || i + 1}: ${c.label}`).join('. ');
      await audio.speak(`Choose. ${choiceText}`);
    } else {
      setStatus('The oral tradition rests. You carry the story now.');
      audio.playChime('success');
    }

    setIsPlaying(false);
  }, [currentNode, activeChar, activeDeck, settingsHook.settings.ambientMood, history, storyteller, audio]);

  // ─── Handle choice ─────────────────────────────────────────
  const makeChoice = useCallback(async (index) => {
    if (index >= choices.length) return;
    const [dir, choice] = choices[index];
    if (!choice?.to) return;

    audio.playChime('enter');

    // Get AI transition narration
    const transition = await storyteller.narrateChoice({ node: currentNode, character: activeChar, deck: activeDeck }, choice.label);
    if (transition.text) {
      await audio.speak(transition.text);
    }

    // Update history
    setHistory(prev => prev + `\nAt ${currentNode.title}, chose: ${choice.label}. `);
    setCurrentNodeId(choice.to);
  }, [choices, currentNode, activeChar, activeDeck, storyteller, audio]);

  // ─── Handle depth ────────────────────────────────────────
  const askDepth = useCallback(async () => {
    if (!currentNode) return;
    audio.playChime('depth');
    const result = await storyteller.depthDive({
      node: currentNode,
      character: activeChar,
      deck: activeDeck,
    });
    setStatus(result.text);
    await audio.speak(result.text);
  }, [currentNode, activeChar, activeDeck, storyteller, audio]);

  // ─── Voice command handler ─────────────────────────────────
  const handleVoiceCommand = useCallback((cmd, text) => {
    if (cmd === 'pause') {
      if (isPlaying) {
        audio.stopSpeaking();
        setIsPlaying(false);
      } else {
        narrateScene();
      }
    } else if (cmd === 'play') {
      if (!isPlaying) narrateScene();
    } else if (cmd.startsWith('choice_')) {
      const idx = parseInt(cmd.split('_')[1]);
      makeChoice(idx);
    } else if (cmd === 'depth') {
      askDepth();
    } else if (cmd === 'skip') {
      audio.stopSpeaking();
      setIsPlaying(false);
    } else if (cmd === 'repeat') {
      if (currentNarration) audio.speak(currentNarration);
    } else if (cmd === 'text' && text) {
      // Player said something unrecognised — could send to AI in future
      setStatus(`I heard: "${text}"`);
    }
  }, [isPlaying, narrateScene, makeChoice, askDepth, audio, currentNarration]);

  const voice = useVoiceCommands({ onCommand: handleVoiceCommand, enabled: true });

  // ─── Keyboard controls ───────────────────────────────────
  useEffect(() => {
    const handleKey = async (e) => {
      if (e.target.tagName === 'INPUT' || e.target.tagName === 'TEXTAREA') return;
      const cmd = KEY_MAP[e.key];
      if (!cmd) return;
      e.preventDefault();
      handleVoiceCommand(cmd);
    };
    window.addEventListener('keydown', handleKey);
    return () => window.removeEventListener('keydown', handleKey);
  }, [handleVoiceCommand]);

  // Auto-narrate on node change
  useEffect(() => {
    if (settingsHook.settings.autoNarrate && currentNode && !isPlaying) {
      narrateScene();
    }
  }, [currentNodeId]);

  // ─── Orb tap handler ─────────────────────────────────────
  const handleOrbTap = () => {
    if (isPlaying) {
      audio.stopSpeaking();
      setIsPlaying(false);
    } else {
      narrateScene();
    }
  };

  if (!adventure) {
    return (
      <div style={styles.page}>
        <div style={styles.statusText}>No adventure loaded.</div>
        <button style={styles.backBtn} onClick={onBack || (() => navigate('/'))}>←</button>
      </div>
    );
  }

  const b = storyteller.backend;
  const bColor = BACKEND_COLORS[b?.name] || '#64748b';
  const bName = BACKEND_NAMES[b?.name] || 'Checking...';

  return (
    <div style={styles.page}>
      <style>{`
        @keyframes pulse {
          0%, 100% { transform: scale(1); opacity: 0.7; }
          50% { transform: scale(1.08); opacity: 1; }
        }
      `}</style>

      {/* Brand */}
      <div style={styles.brand}>
        <p style={styles.brandTitle}>Gamified Oral Tradition</p>
        <p style={styles.brandSubtitle}>{adventure.title}</p>
      </div>

      <button style={styles.backBtn} onClick={onBack || (() => navigate('/'))}>←</button>

      {currentNode?.focusWord && (
        <div style={styles.wordBadge}>{currentNode.focusWord}</div>
      )}

      {/* Backend status */}
      <div style={{ ...styles.backendBadge, color: bColor, border: `1px solid ${bColor}40` }}>
        <span style={{ ...styles.statusDot, background: bColor }} />
        {storyteller.isProbing ? 'Detecting AI...' : bName}
      </div>

      {/* Listening transcript overlay */}
      {voice.isListening && voice.transcript && (
        <div style={styles.transcript}>"{voice.transcript}"</div>
      )}

      {/* The Orb */}
      <div style={styles.orb} onClick={handleOrbTap}>
        <div style={styles.orbInner}>
          {isPlaying ? '◉' : voice.isListening ? '🎙' : '○'}
        </div>
        <span style={styles.orbLabel}>
          {isPlaying ? 'LISTENING' : voice.isListening ? 'HEARING' : 'TAP TO BEGIN'}
        </span>
      </div>

      {/* Status / Story text */}
      <div style={styles.statusText}>{status}</div>

      {/* Choice buttons (visible but also voice-controlled) */}
      {showChoices && (
        <div style={{ marginTop: '16px', display: 'flex', flexDirection: 'column', gap: '6px', alignItems: 'center' }}>
          {choices.map(([dir, c], i) => {
            const dirLabel = { up: '↑', right: '→', down: '↓', left: '←' }[dir];
            return (
              <button
                key={dir}
                onClick={() => makeChoice(i)}
                style={{
                  padding: '8px 16px',
                  borderRadius: '10px',
                  border: '1px solid rgba(6,182,212,0.25)',
                  background: 'rgba(6,182,212,0.08)',
                  color: '#e2e8f0',
                  fontSize: '13px',
                  cursor: 'pointer',
                  minWidth: '260px',
                  textAlign: 'left',
                }}
              >
                <span style={{ color: '#06b6d4', marginRight: '8px' }}>{dirLabel}</span>
                {c.label}
              </button>
            );
          })}
        </div>
      )}

      {/* Hint */}
      <div style={styles.hint}>
        {voice.supported
          ? 'Say "play" to start. Say directions: "up", "right", "down", "left". Say "depth" to go deeper. Say "pause" or "repeat".'
          : 'Space: Play/Pause \u00B7 Arrows: Choose \u00B7 D: Depth \u00B7 S: Skip \u00B7 R: Repeat'}
      </div>

      {/* Controls */}
      <div style={styles.controls}>
        <button
          style={{ ...styles.ctrlBtn, ...(isPlaying ? styles.ctrlBtnActive : {}) }}
          onClick={handleOrbTap}
          title="Play / Pause"
        >
          {isPlaying ? '⏸' : '▶'}
        </button>
        <button
          style={{ ...styles.ctrlBtn, ...(voice.isListening ? styles.ctrlBtnActive : {}) }}
          onClick={voice.isListening ? voice.stopListening : voice.startListening}
          title="Voice Command"
        >
          {voice.isListening ? '🎙' : '🎤'}
        </button>
        <button style={styles.ctrlBtn} onClick={askDepth} title="Depth">🜂</button>
        <button
          style={styles.ctrlBtn}
          onClick={() => { audio.stopSpeaking(); setIsPlaying(false); }}
          title="Stop"
        >
          ⏹
        </button>
      </div>
    </div>
  );
}
