import { useState, useCallback, useRef } from 'react';

// ════════════════════════════════════════════════════════════
// useStorytellerAI — StepAudio R1.1 / Nemotron / Offline Bridge
// ════════════════════════════════════════════════════════════
// Probes local vLLM backends and generates dynamic story narration.
// Priority: StepAudio :9998 → LM Studio :1234 → Offline (static).
//
// "Gamified Oral Tradition" — the AI is the bard. The player speaks,
// the AI listens, the world responds.

const STEP_AUDIO_URL = 'http://localhost:9998/v1';
const LM_STUDIO_URL = 'http://localhost:1234/v1';
const FETCH_TIMEOUT = 8000;

async function probeBackend(url) {
  try {
    const ctrl = new AbortController();
    const t = setTimeout(() => ctrl.abort(), 3000);
    const resp = await fetch(`${url}/models`, { signal: ctrl.signal });
    clearTimeout(t);
    if (resp.ok) {
      const data = await resp.json();
      return data.data?.[0]?.id || 'unknown';
    }
  } catch {
    // ignore
  }
  return null;
}

function buildSystemPrompt(character, deck, mood) {
  const name = character?.name || 'Traveler';
  const archetype = character?.archetypeName || 'Wanderer';
  const words = deck?.length ? deck.join(', ') : 'the ancient words';
  const moodName = mood || 'The Cave';

  return [
    `You are the Storyteller of the Oral Tradition. You narrate an interactive audio adventure.`,
    ``,
    `The listener is ${name}, ${archetype}.`,
    `Their power words: ${words}.`,
    `The ambient mood is ${moodName}.`,
    ``,
    `RULES OF THE BARD:`,
    `1. Speak in second person ("You stand...", "You feel...")`,
    `2. Keep responses under 60 words — this is spoken, not read.`,
    `3. End with a pause (...). Never break character.`,
    `4. When presenting choices, number them: "One: [option]. Two: [option]."`,
    `5. Weave the focus word naturally into the narrative.`,
    `6. Match the ambient mood in tone — cave is grounding, forest is gentle, void is mysterious.`,
    `7. Never mention you are an AI. You are the voice of the story itself.`,
  ].join('\n');
}

export function useStorytellerAI() {
  const [backend, setBackend] = useState(null); // { name, url, model }
  const [isProbing, setIsProbing] = useState(false);
  const backendRef = useRef(null);

  // ─── Probe on first use ──────────────────────────────────
  const probe = useCallback(async () => {
    if (isProbing) return backendRef.current;
    setIsProbing(true);

    // 1. Try StepAudio R1.1 (production)
    const stepModel = await probeBackend(STEP_AUDIO_URL);
    if (stepModel) {
      const b = { name: 'stepaudio', url: STEP_AUDIO_URL, model: stepModel };
      setBackend(b);
      backendRef.current = b;
      setIsProbing(false);
      return b;
    }

    // 2. Try LM Studio / Nemotron (dev fallback)
    const lmModel = await probeBackend(LM_STUDIO_URL);
    if (lmModel) {
      const b = { name: 'lmstudio', url: LM_STUDIO_URL, model: lmModel };
      setBackend(b);
      backendRef.current = b;
      setIsProbing(false);
      return b;
    }

    // 3. Offline
    const b = { name: 'offline', url: null, model: null };
    setBackend(b);
    backendRef.current = b;
    setIsProbing(false);
    return b;
  }, [isProbing]);

  // ─── Chat completion ─────────────────────────────────────
  const narrate = useCallback(async (context) => {
    const { node, character, deck, mood, history } = context;
    const b = backendRef.current || (await probe());

    const userPrompt = [
      `Current scene: ${node?.title || 'Unknown'}`,
      `Focus word: ${node?.focusWord || 'Silence'}`,
      `Channel: ${node?.channel || 'mind'}`,
      `Scene context: ${node?.story || 'You stand in silence.'}`,
      history ? `\nWhat has happened so far: ${history.slice(-200)}` : '',
    ].join('\n');

    // Offline fallback: return the node's static story
    if (b.name === 'offline') {
      return {
        text: node?.story || 'The story waits for you to begin.',
        source: 'offline',
        choices: node?.exits ? Object.values(node.exits).filter(v => v).map(c => c.label) : [],
      };
    }

    try {
      const ctrl = new AbortController();
      const t = setTimeout(() => ctrl.abort(), FETCH_TIMEOUT);

      const resp = await fetch(`${b.url}/chat/completions`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          model: b.model,
          messages: [
            { role: 'system', content: buildSystemPrompt(character, deck, mood) },
            { role: 'user', content: userPrompt },
          ],
          temperature: 0.8,
          max_tokens: 256,
          stream: false,
        }),
        signal: ctrl.signal,
      });
      clearTimeout(t);

      if (!resp.ok) throw new Error(`HTTP ${resp.status}`);

      const data = await resp.json();
      const text = data.choices?.[0]?.message?.content || node?.story || '...';

      return {
        text,
        source: b.name,
        raw: data,
      };
    } catch (err) {
      console.error('[StorytellerAI] Narration failed:', err);
      return {
        text: node?.story || 'The story continues in silence.',
        source: 'fallback',
        error: err.message,
      };
    }
  }, [probe]);

  // ─── Generate dynamic depth response ─────────────────────
  const depthDive = useCallback(async (context) => {
    const { node, character, deck } = context;
    const b = backendRef.current || (await probe());

    if (b.name === 'offline') {
      return { text: node?.depth || 'Look deeper...', source: 'offline' };
    }

    const prompt = [
      `The student asks a deeper question about: ${node?.focusWord || 'the moment'}`,
      `Current depth prompt: ${node?.depth || 'What do you truly feel?'}`,
      `Channel: ${node?.channel || 'mind'}`,
      `Respond with a single Socratic question that invites introspection.`,
      `Max 30 words. Speak as the Storyteller.`,
    ].join('\n');

    try {
      const ctrl = new AbortController();
      const t = setTimeout(() => ctrl.abort(), FETCH_TIMEOUT);
      const resp = await fetch(`${b.url}/chat/completions`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          model: b.model,
          messages: [
            { role: 'system', content: buildSystemPrompt(character, deck) },
            { role: 'user', content: prompt },
          ],
          temperature: 0.9,
          max_tokens: 128,
        }),
        signal: ctrl.signal,
      });
      clearTimeout(t);

      if (!resp.ok) throw new Error(`HTTP ${resp.status}`);
      const data = await resp.json();
      return {
        text: data.choices?.[0]?.message?.content || node?.depth || 'Look deeper...',
        source: b.name,
      };
    } catch (err) {
      return { text: node?.depth || 'Look deeper...', source: 'fallback', error: err.message };
    }
  }, [probe]);

  // ─── Generate choice narration ───────────────────────────
  const narrateChoice = useCallback(async (context, choiceLabel) => {
    const { character, deck } = context;
    const b = backendRef.current || (await probe());

    if (b.name === 'offline') {
      return { text: `You choose: ${choiceLabel}`, source: 'offline' };
    }

    const prompt = [
      `The player has chosen: "${choiceLabel}"`,
      `Narrate the moment of decision in 2 sentences.`,
      `Make it feel significant. End with "..."`,
    ].join('\n');

    try {
      const ctrl = new AbortController();
      const t = setTimeout(() => ctrl.abort(), FETCH_TIMEOUT);
      const resp = await fetch(`${b.url}/chat/completions`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          model: b.model,
          messages: [
            { role: 'system', content: buildSystemPrompt(character, deck) },
            { role: 'user', content: prompt },
          ],
          temperature: 0.85,
          max_tokens: 128,
        }),
        signal: ctrl.signal,
      });
      clearTimeout(t);

      if (!resp.ok) throw new Error(`HTTP ${resp.status}`);
      const data = await resp.json();
      return {
        text: data.choices?.[0]?.message?.content || `You choose: ${choiceLabel}...`,
        source: b.name,
      };
    } catch (err) {
      return { text: `You choose: ${choiceLabel}...`, source: 'fallback', error: err.message };
    }
  }, [probe]);

  return {
    backend,
    isProbing,
    probe,
    narrate,
    depthDive,
    narrateChoice,
  };
}
