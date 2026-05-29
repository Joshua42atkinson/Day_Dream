import { useRef, useCallback, useEffect } from 'react';
import { AmbientMoods } from '../data/arcana';

// ════════════════════════════════════════════════════════════
// useAudioOrchestrator — TTS + Ambient + Storyteller Engine
// ════════════════════════════════════════════════════════════
// Hands-free audio controller. Manages:
//   - Web Speech API TTS (with voice/style selection)
//   - Ambient drone (sine wave, cross-fading)
//   - Sound effects (chimes, transitions)
//   - Bluetooth headphone detection (best-effort)
//   - Auto-narration queue (story text → speech)

export function useAudioOrchestrator(settings) {
  const ctxRef = useRef(null);
  const droneOscRef = useRef(null);
  const droneGainRef = useRef(null);
  const sfxGainRef = useRef(null);
  const ttsQueueRef = useRef([]);
  const isSpeakingRef = useRef(false);

  // ─── Audio Context Init ──────────────────────────────────
  const initAudio = useCallback(() => {
    if (ctxRef.current) return ctxRef.current;
    try {
      const ctx = new (window.AudioContext || window.webkitAudioContext)();
      ctxRef.current = ctx;

      droneGainRef.current = ctx.createGain();
      droneGainRef.current.gain.value = 0;
      droneGainRef.current.connect(ctx.destination);

      sfxGainRef.current = ctx.createGain();
      sfxGainRef.current.gain.value = settings.soundEffects ? 0.5 : 0;
      sfxGainRef.current.connect(ctx.destination);
    } catch {
      // Audio unavailable
    }
    return ctxRef.current;
  }, [settings.soundEffects]);

  // ─── Ambient Drone ───────────────────────────────────────
  const playMood = useCallback((moodId) => {
    const ctx = initAudio();
    const gain = droneGainRef.current;
    if (!ctx || !gain) return;

    const mood = Object.values(AmbientMoods).find(m => m.id === moodId) || AmbientMoods.CAVE;

    // Stop previous
    if (droneOscRef.current) {
      try { droneOscRef.current.stop(); } catch { /* noop */ }
      droneOscRef.current = null;
    }

    if (!mood.drone) {
      // Silence mode
      gain.gain.setTargetAtTime(0, ctx.currentTime, 0.5);
      return;
    }

    const osc = ctx.createOscillator();
    osc.type = 'sine';
    osc.frequency.value = mood.drone;
    osc.connect(gain);
    osc.start();
    droneOscRef.current = osc;

    const targetVol = (settings.ambientVolume || 0.3) * 0.15;
    gain.gain.cancelScheduledValues(ctx.currentTime);
    gain.gain.setTargetAtTime(targetVol, ctx.currentTime, 1.0);
  }, [initAudio, settings.ambientVolume]);

  const fadeMood = useCallback(() => {
    const ctx = ctxRef.current;
    const gain = droneGainRef.current;
    if (!ctx || !gain) return;
    gain.gain.setTargetAtTime(0, ctx.currentTime, 0.5);
  }, []);

  // ─── Sound Effects ───────────────────────────────────────
  const playChime = useCallback((type = 'soft') => {
    const ctx = initAudio();
    const gain = sfxGainRef.current;
    if (!ctx || !gain || !settings.soundEffects) return;

    const osc = ctx.createOscillator();
    const noteGain = ctx.createGain();
    osc.connect(noteGain);
    noteGain.connect(gain);

    const freqs = {
      soft: [528.0, 528.0 * 1.25],     // C major feel
      enter: [174.6, 220.0],           // Healing to mystery
      exit: [220.0, 174.6],
      success: [528.0, 659.3, 783.9],  // C-E-G arpeggio
      depth: [136.1, 136.1],           // Om
    };

    const notes = freqs[type] || freqs.soft;
    const now = ctx.currentTime;

    notes.forEach((freq, i) => {
      const o = ctx.createOscillator();
      o.type = 'sine';
      o.frequency.value = freq;
      const g = ctx.createGain();
      g.gain.setValueAtTime(0, now + i * 0.15);
      g.gain.linearRampToValueAtTime(0.08, now + i * 0.15 + 0.05);
      g.gain.exponentialRampToValueAtTime(0.001, now + i * 0.15 + 0.8);
      o.connect(g);
      g.connect(gain);
      o.start(now + i * 0.15);
      o.stop(now + i * 0.15 + 0.8);
    });
  }, [initAudio, settings.soundEffects]);

  // ─── TTS (Text-to-Speech) ────────────────────────────────
  const speak = useCallback((text, opts = {}) => {
    if (!window.speechSynthesis) return Promise.resolve();

    return new Promise((resolve) => {
      const utter = new SpeechSynthesisUtterance(text);
      utter.rate = opts.rate ?? settings.ttsRate ?? 0.9;
      utter.pitch = opts.pitch ?? settings.ttsPitch ?? 1.0;
      utter.volume = opts.volume ?? settings.ttsVolume ?? 1.0;

      // Try to match voice style
      const voices = window.speechSynthesis.getVoices();
      const style = opts.voiceStyle ?? settings.voiceStyle ?? 'whisper';
      const preferred = voices.find(v => {
        const name = v.name.toLowerCase();
        if (style === 'whisper') return name.includes('whisper') || name.includes('soft');
        if (style === 'bard') return name.includes('expressive') || name.includes('warm');
        if (style === 'sage') return name.includes('calm') || name.includes('measured');
        return v.default;
      });
      if (preferred) utter.voice = preferred;

      utter.onend = () => { isSpeakingRef.current = false; resolve(); };
      utter.onerror = () => { isSpeakingRef.current = false; resolve(); };

      isSpeakingRef.current = true;
      window.speechSynthesis.cancel(); // Stop any current speech
      window.speechSynthesis.speak(utter);
    });
  }, [settings]);

  const stopSpeaking = useCallback(() => {
    if (window.speechSynthesis) {
      window.speechSynthesis.cancel();
    }
    isSpeakingRef.current = false;
    ttsQueueRef.current = [];
  }, []);

  const queueSpeech = useCallback((text, opts = {}) => {
    ttsQueueRef.current.push({ text, opts });
    if (!isSpeakingRef.current) {
      processQueue();
    }
  }, []);

  const processQueue = useCallback(async () => {
    while (ttsQueueRef.current.length > 0) {
      const { text, opts } = ttsQueueRef.current.shift();
      await speak(text, opts);
    }
  }, [speak]);

  // ─── Bluetooth Detection (best effort) ───────────────────
  const checkHeadphones = useCallback(async () => {
    try {
      if (navigator.mediaDevices && navigator.mediaDevices.enumerateDevices) {
        const devices = await navigator.mediaDevices.enumerateDevices();
        const audioOutputs = devices.filter(d => d.kind === 'audiooutput');
        const hasBluetooth = audioOutputs.some(d =>
          d.label.toLowerCase().includes('bluetooth') ||
          d.label.toLowerCase().includes('bt') ||
          d.label.toLowerCase().includes('headphone')
        );
        return hasBluetooth;
      }
    } catch { /* ignore */ }
    return false;
  }, []);

  // ─── Storyteller Prompt Builder ──────────────────────────
  const buildStoryPrompt = useCallback((context) => {
    const { node, character, deck, history } = context;
    const wordList = deck?.map(w => w.word).join(', ') || '';
    const archetype = character?.archetypeName || 'Wanderer';
    const name = character?.name || 'Traveler';

    return [
      `You are a storyteller narrating an interactive educational adventure.`,
      `The listener is ${name}, ${archetype}.`,
      `Their power words: ${wordList}.`,
      `Current scene: ${node?.title || 'Unknown place'}.`,
      `Scene context: ${node?.story || ''}`,
      `Focus word: ${node?.focusWord || ''}`,
      `Channel: ${node?.channel || ''}`,
      ``,
      `Rules:`,
      `1. Speak in second person ("You see...", "You feel...")`,
      `2. Weave the focus word naturally into the narrative`,
      `3. End with a pause cue: "..."`,
      `4. Keep responses under 4 sentences`,
      `5. Never break character or mention you are an AI`,
      `6. If this is a choice moment, present exactly 2-3 clear options`,
      history ? `\nStory so far: ${history}` : '',
    ].join('\n');
  }, []);

  // ─── Cleanup ─────────────────────────────────────────────
  useEffect(() => {
    return () => {
      stopSpeaking();
      fadeMood();
      if (ctxRef.current && ctxRef.current.state !== 'closed') {
        ctxRef.current.close().catch(() => {});
      }
    };
  }, [stopSpeaking, fadeMood]);

  return {
    playMood,
    fadeMood,
    playChime,
    speak,
    stopSpeaking,
    queueSpeech,
    checkHeadphones,
    buildStoryPrompt,
    isSpeaking: () => isSpeakingRef.current,
  };
}
