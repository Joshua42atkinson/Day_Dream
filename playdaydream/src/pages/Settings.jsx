import { useNavigate } from 'react-router-dom';
import { VoiceStyles, AmbientMoods } from '../data/arcana';

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
    display: 'flex',
    alignItems: 'center',
    gap: '12px',
    marginBottom: '32px',
  },
  backBtn: {
    background: 'none',
    border: '1px solid rgba(255,255,255,0.2)',
    color: '#e2e8f0',
    borderRadius: '50%',
    width: '40px',
    height: '40px',
    cursor: 'pointer',
    fontSize: '18px',
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
  },
  title: {
    fontFamily: "'Cormorant Garamond', serif",
    fontSize: '28px',
    fontWeight: 600,
    color: '#06b6d4',
    margin: 0,
  },
  section: {
    marginBottom: '32px',
  },
  sectionTitle: {
    fontSize: '12px',
    textTransform: 'uppercase',
    letterSpacing: '2px',
    color: '#06b6d4',
    marginBottom: '16px',
    fontWeight: 600,
  },
  card: {
    background: 'rgba(26,26,46,0.8)',
    border: '1px solid rgba(6,182,212,0.15)',
    borderRadius: '16px',
    padding: '20px',
    marginBottom: '12px',
  },
  label: {
    fontSize: '14px',
    color: '#94a3b8',
    marginBottom: '8px',
    display: 'block',
  },
  value: {
    fontSize: '16px',
    color: '#e2e8f0',
    fontWeight: 500,
  },
  grid: {
    display: 'grid',
    gridTemplateColumns: 'repeat(2, 1fr)',
    gap: '8px',
  },
  option: {
    background: 'rgba(255,255,255,0.05)',
    border: '1px solid rgba(255,255,255,0.1)',
    borderRadius: '12px',
    padding: '12px',
    cursor: 'pointer',
    transition: 'all 0.2s',
    textAlign: 'left',
  },
  optionActive: {
    background: 'rgba(6,182,212,0.15)',
    borderColor: '#06b6d4',
  },
  optionName: {
    fontSize: '14px',
    fontWeight: 600,
    color: '#e2e8f0',
    marginBottom: '4px',
  },
  optionDesc: {
    fontSize: '12px',
    color: '#94a3b8',
  },
  slider: {
    width: '100%',
    accentColor: '#06b6d4',
    marginTop: '8px',
  },
  toggle: {
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'space-between',
    padding: '12px 0',
  },
  toggleLabel: {
    fontSize: '14px',
    color: '#e2e8f0',
  },
  toggleSwitch: {
    width: '48px',
    height: '24px',
    borderRadius: '12px',
    background: '#334155',
    position: 'relative',
    cursor: 'pointer',
    transition: 'background 0.2s',
  },
  toggleKnob: {
    width: '20px',
    height: '20px',
    borderRadius: '50%',
    background: '#fff',
    position: 'absolute',
    top: '2px',
    left: '2px',
    transition: 'left 0.2s',
  },
  knobActive: {
    left: '26px',
    background: '#06b6d4',
  },
  dangerBtn: {
    width: '100%',
    padding: '14px',
    borderRadius: '12px',
    border: '1px solid rgba(239,68,68,0.3)',
    background: 'rgba(239,68,68,0.1)',
    color: '#ef4444',
    fontSize: '14px',
    cursor: 'pointer',
    marginTop: '16px',
  },
  testBtn: {
    width: '100%',
    padding: '14px',
    borderRadius: '12px',
    border: '1px solid rgba(6,182,212,0.3)',
    background: 'rgba(6,182,212,0.1)',
    color: '#06b6d4',
    fontSize: '14px',
    cursor: 'pointer',
    marginTop: '8px',
  },
};

export default function Settings({ settingsHook }) {
  const navigate = useNavigate();
  const { settings, update, setVoiceStyle, setAmbientMood, setTtsRate, setTtsPitch,
          setAmbientVolume, toggleBluetoothOnly, toggleAutoNarrate, toggleSoundEffects,
          resetToDefaults } = settingsHook;

  const testVoice = () => {
    if (!window.speechSynthesis) return;
    const text = "You stand at the threshold of your own story. Every choice shapes who you become.";
    const utter = new SpeechSynthesisUtterance(text);
    utter.rate = settings.ttsRate;
    utter.pitch = settings.ttsPitch;
    utter.volume = settings.ttsVolume;
    window.speechSynthesis.cancel();
    window.speechSynthesis.speak(utter);
  };

  const Toggle = ({ label, value, onChange }) => (
    <div style={styles.toggle}>
      <span style={styles.toggleLabel}>{label}</span>
      <div
        style={{ ...styles.toggleSwitch, ...(value ? { background: 'rgba(6,182,212,0.3)' } : {}) }}
        onClick={onChange}
      >
        <div style={{ ...styles.toggleKnob, ...(value ? styles.knobActive : {}) }} />
      </div>
    </div>
  );

  return (
    <div style={styles.page}>
      <div style={styles.header}>
        <button style={styles.backBtn} onClick={() => navigate(-1)}>←</button>
        <h1 style={styles.title}>Vibe Portal</h1>
      </div>

      {/* ─── VOICE ────────────────────────────────────────── */}
      <div style={styles.section}>
        <div style={styles.sectionTitle}>Narration Voice</div>
        <div style={styles.card}>
          <label style={styles.label}>Storyteller Persona</label>
          <div style={styles.grid}>
            {Object.values(VoiceStyles).map(v => (
              <button
                key={v.id}
                style={{ ...styles.option, ...(settings.voiceStyle === v.id ? styles.optionActive : {}) }}
                onClick={() => setVoiceStyle(v.id)}
              >
                <div style={styles.optionName}>{v.name}</div>
                <div style={styles.optionDesc}>{v.desc}</div>
              </button>
            ))}
          </div>
        </div>

        <div style={styles.card}>
          <label style={styles.label}>Speech Rate: {settings.ttsRate.toFixed(1)}x</label>
          <input
            type="range" min="0.5" max="1.5" step="0.1"
            value={settings.ttsRate}
            onChange={e => setTtsRate(parseFloat(e.target.value))}
            style={styles.slider}
          />
        </div>

        <div style={styles.card}>
          <label style={styles.label}>Voice Pitch: {settings.ttsPitch.toFixed(1)}</label>
          <input
            type="range" min="0.5" max="2.0" step="0.1"
            value={settings.ttsPitch}
            onChange={e => setTtsPitch(parseFloat(e.target.value))}
            style={styles.slider}
          />
        </div>

        <button style={styles.testBtn} onClick={testVoice}>Test Voice</button>
      </div>

      {/* ─── AMBIENT ────────────────────────────────────── */}
      <div style={styles.section}>
        <div style={styles.sectionTitle}>Ambient Mood</div>
        <div style={styles.card}>
          <div style={styles.grid}>
            {Object.values(AmbientMoods).map(m => (
              <button
                key={m.id}
                style={{ ...styles.option, ...(settings.ambientMood === m.id ? styles.optionActive : {}) }}
                onClick={() => setAmbientMood(m.id)}
              >
                <div style={styles.optionName}>{m.name}</div>
                <div style={styles.optionDesc}>{m.desc}</div>
              </button>
            ))}
          </div>
        </div>

        <div style={styles.card}>
          <label style={styles.label}>Ambient Volume: {Math.round((settings.ambientVolume || 0) * 100)}%</label>
          <input
            type="range" min="0" max="1" step="0.05"
            value={settings.ambientVolume || 0}
            onChange={e => setAmbientVolume(parseFloat(e.target.value))}
            style={styles.slider}
          />
        </div>
      </div>

      {/* ─── AUDIO ROUTING ──────────────────────────────── */}
      <div style={styles.section}>
        <div style={styles.sectionTitle}>Audio Routing</div>
        <div style={styles.card}>
          <Toggle label="Headphones Only" value={settings.bluetoothOnly} onChange={toggleBluetoothOnly} />
          <Toggle label="Auto-Narrate Story" value={settings.autoNarrate} onChange={toggleAutoNarrate} />
          <Toggle label="Sound Effects" value={settings.soundEffects} onChange={toggleSoundEffects} />
        </div>
      </div>

      {/* ─── RESET ──────────────────────────────────────── */}
      <button style={styles.dangerBtn} onClick={resetToDefaults}>
        Reset All Settings to Default
      </button>
    </div>
  );
}
