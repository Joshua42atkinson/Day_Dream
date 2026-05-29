import { useState, useCallback, useRef } from 'react';
import { buildSceneFromNode } from '../lib/artPromptBuilder';

// ════════════════════════════════════════════════════════════
// useSceneArtist — ComfyUI Art Generation Hook
// ════════════════════════════════════════════════════════════
// Bridges the React frontend to the local ComfyUI server at :8188
// via the Vite dev proxy (/api/comfyui → localhost:8188).
//
// Pipeline: build workflow → submit → poll → download → blob URL
//
// Usage:
//   const { generate, status, progress, imageUrl, error } = useSceneArtist();
//   generate(node); // node = { story, focusWord, channel, themes }

const COMFYUI_PROXY = '/api/comfyui';
const POLL_INTERVAL_MS = 2000;
const MAX_POLL_SECONDS = 600;

async function comfyFetch(path, opts = {}) {
  const url = `${COMFYUI_PROXY}${path}`;
  const resp = await fetch(url, {
    ...opts,
    headers: {
      'Content-Type': 'application/json',
      ...opts.headers,
    },
  });
  if (!resp.ok) {
    const text = await resp.text().catch(() => '');
    throw new Error(`ComfyUI ${resp.status}: ${text}`);
  }
  return resp.json();
}

export function useSceneArtist() {
  const [status, setStatus] = useState('idle'); // idle | building | submitting | generating | done | error
  const [progress, setProgress] = useState(0); // 0–100
  const [imageUrl, setImageUrl] = useState(null);
  const [error, setError] = useState(null);
  const [promptPreview, setPromptPreview] = useState('');
  const abortRef = useRef(false);

  const cancel = useCallback(() => {
    abortRef.current = true;
    setStatus('idle');
    setProgress(0);
  }, []);

  const generate = useCallback(async (node) => {
    abortRef.current = false;
    setStatus('building');
    setProgress(5);
    setError(null);
    setImageUrl(null);

    try {
      // 1. Build workflow from node
      const { prompt, negativePrompt, workflow } = buildSceneFromNode(node);
      setPromptPreview(prompt);
      if (abortRef.current) return;

      // 2. Pre-flight: check ComfyUI health
      setStatus('submitting');
      setProgress(10);
      const stats = await comfyFetch('/system_stats');
      if (!stats || !stats.devices) {
        throw new Error('ComfyUI is not running on :8188. Start it first.');
      }
      if (abortRef.current) return;

      // 3. Submit workflow
      const payload = JSON.stringify(workflow);
      const submitResp = await fetch(`${COMFYUI_PROXY}/prompt`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: payload,
      });
      if (!submitResp.ok) {
        const text = await submitResp.text();
        throw new Error(`ComfyUI rejected workflow: ${text}`);
      }
      const { prompt_id: promptId } = await submitResp.json();
      if (!promptId) throw new Error('No prompt_id returned from ComfyUI');
      if (abortRef.current) return;

      // 4. Poll for result
      setStatus('generating');
      setProgress(15);
      const startTime = Date.now();
      let result = null;

      while (Date.now() - startTime < MAX_POLL_SECONDS * 1000) {
        if (abortRef.current) return;

        await new Promise((r) => setTimeout(r, POLL_INTERVAL_MS));

        const history = await comfyFetch(`/history/${promptId}`);
        if (!history[promptId]) {
          // Still in queue
          const queue = await comfyFetch('/queue');
          const pending = queue.queue_running?.length || 0;
          setProgress(15 + Math.min(pending ? 30 : 60, 70));
          continue;
        }

        const job = history[promptId];
        const jobStatus = job.status?.status_str;

        if (jobStatus === 'error') {
          const msgs = job.status?.messages || [];
          throw new Error(`Generation failed: ${JSON.stringify(msgs)}`);
        }

        const outputs = job.outputs || {};
        const images = [];
        for (const nodeId of Object.keys(outputs)) {
          const nodeOut = outputs[nodeId];
          for (const img of nodeOut.images || []) {
            images.push(img);
          }
        }

        if (images.length > 0) {
          result = images[0];
          break;
        }

        setProgress(85);
      }

      if (!result) {
        throw new Error('Generation timed out after 10 minutes');
      }
      if (abortRef.current) return;

      // 5. Download image
      setProgress(90);
      const { filename, subfolder = '', type = 'output' } = result;
      const params = new URLSearchParams({ filename, subfolder, type });
      const imgResp = await fetch(`${COMFYUI_PROXY}/view?${params.toString()}`);
      if (!imgResp.ok) throw new Error('Failed to download generated image');

      const blob = await imgResp.blob();
      const blobUrl = URL.createObjectURL(blob);
      setImageUrl(blobUrl);
      setStatus('done');
      setProgress(100);

      return blobUrl;
    } catch (err) {
      if (abortRef.current) return;
      setError(err.message || 'Unknown generation error');
      setStatus('error');
      setProgress(0);
      throw err;
    }
  }, []);

  const reset = useCallback(() => {
    setStatus('idle');
    setProgress(0);
    setImageUrl(null);
    setError(null);
    setPromptPreview('');
    abortRef.current = false;
  }, []);

  return {
    generate,
    cancel,
    reset,
    status,
    progress,
    imageUrl,
    error,
    promptPreview,
    isIdle: status === 'idle',
    isBuilding: status === 'building',
    isSubmitting: status === 'submitting',
    isGenerating: status === 'generating',
    isDone: status === 'done',
    isError: status === 'error',
  };
}
