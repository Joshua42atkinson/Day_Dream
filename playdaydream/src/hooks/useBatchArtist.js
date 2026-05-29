import { useState, useCallback, useRef } from 'react';
import { buildSceneFromNode } from '../lib/artPromptBuilder';

// ════════════════════════════════════════════════════════════
// useBatchArtist — Automated Multi-Scene Art Generation
// ════════════════════════════════════════════════════════════
// Queue up nodes, auto-generate images for all of them via ComfyUI.
// Tracks per-node status, overall progress, auto-saves blob URLs.
//
// Usage:
//   const { startBatch, status, jobs, overallProgress, cancel } = useBatchArtist();
//   startBatch([
//     { focusWord: 'Resilience', channel: 'action', story: '...', themes: [] },
//     { focusWord: 'Patience', channel: 'body', story: '...', themes: [] },
//   ]);

const COMFYUI_PROXY = '/api/comfyui';
const POLL_INTERVAL_MS = 2500;
const MAX_POLL_SECONDS = 600;

async function comfyFetch(path, opts = {}) {
  const url = `${COMFYUI_PROXY}${path}`;
  const resp = await fetch(url, {
    ...opts,
    headers: { 'Content-Type': 'application/json', ...opts.headers },
  });
  if (!resp.ok) {
    const text = await resp.text().catch(() => '');
    throw new Error(`ComfyUI ${resp.status}: ${text}`);
  }
  return resp.json();
}

export function useBatchArtist() {
  const [status, setStatus] = useState('idle'); // idle | running | done | error | cancelled
  const [jobs, setJobs] = useState([]); // Array of job objects
  const [overallProgress, setOverallProgress] = useState(0);
  const abortRef = useRef(false);

  const reset = useCallback(() => {
    setStatus('idle');
    setJobs([]);
    setOverallProgress(0);
    abortRef.current = false;
  }, []);

  const cancel = useCallback(() => {
    abortRef.current = true;
    setStatus('cancelled');
  }, []);

  const startBatch = useCallback(async (nodes, onJobComplete) => {
    // nodes: array of { focusWord, channel, story, themes, meta? }
    // onJobComplete: (index, blobUrl) => void

    abortRef.current = false;
    setStatus('running');

    const initialJobs = nodes.map((node, i) => ({
      index: i,
      word: node.focusWord || 'scene',
      status: 'pending', // pending | building | queued | generating | done | error
      progress: 0,
      imageUrl: null,
      error: null,
      prompt: '',
      promptId: null,
    }));

    setJobs(initialJobs);
    setOverallProgress(0);

    // Pre-flight: health check
    try {
      const stats = await comfyFetch('/system_stats');
      if (!stats || !stats.devices) {
        throw new Error('ComfyUI is not running on :8188. Start it first.');
      }
    } catch (err) {
      setStatus('error');
      setJobs((prev) =>
        prev.map((j) => ({ ...j, status: 'error', error: err.message }))
      );
      return;
    }

    // Submit ALL workflows to ComfyUI queue (it handles sequential execution)
    const promptIds = [];
    for (let i = 0; i < nodes.length; i++) {
      if (abortRef.current) break;

      const node = nodes[i];
      try {
        setJobs((prev) =>
          prev.map((j) => (j.index === i ? { ...j, status: 'building', progress: 10 } : j))
        );

        const { prompt, negativePrompt, workflow } = buildSceneFromNode(node);

        setJobs((prev) =>
          prev.map((j) => (j.index === i ? { ...j, prompt, progress: 20 } : j))
        );

        const submitResp = await fetch(`${COMFYUI_PROXY}/prompt`, {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(workflow),
        });

        if (!submitResp.ok) {
          const text = await submitResp.text();
          throw new Error(`Rejected: ${text}`);
        }

        const { prompt_id: promptId } = await submitResp.json();
        if (!promptId) throw new Error('No prompt_id returned');

        promptIds.push({ index: i, promptId, word: node.focusWord });

        setJobs((prev) =>
          prev.map((j) =>
            j.index === i ? { ...j, status: 'queued', promptId, progress: 30 } : j
          )
        );
      } catch (err) {
        setJobs((prev) =>
          prev.map((j) =>
            j.index === i ? { ...j, status: 'error', error: err.message, progress: 0 } : j
          )
        );
      }
    }

    // Poll all submitted jobs until they complete
    const pendingJobs = new Map(promptIds.map((p) => [p.promptId, p]));
    const completed = new Set();
    const startTime = Date.now();

    while (pendingJobs.size > 0 && !abortRef.current) {
      if (Date.now() - startTime > MAX_POLL_SECONDS * 1000) {
        // Global timeout
        for (const [promptId, job] of pendingJobs) {
          setJobs((prev) =>
            prev.map((j) =>
              j.index === job.index
                ? { ...j, status: 'error', error: 'Global timeout', progress: 0 }
                : j
            )
          );
        }
        break;
      }

      await new Promise((r) => setTimeout(r, POLL_INTERVAL_MS));

      // Check queue status for global progress estimation
      try {
        const queue = await comfyFetch('/queue');
        const running = queue.queue_running?.length || 0;
        const queued = queue.queue_pending?.length || 0;
        // If ComfyUI queue has few items, some of our jobs are already done
      } catch {
        // Ignore queue poll errors
      }

      // Poll each pending job
      for (const [promptId, jobInfo] of Array.from(pendingJobs)) {
        if (abortRef.current) break;

        try {
          const history = await comfyFetch(`/history/${promptId}`);

          if (!history[promptId]) {
            // Still processing — update to generating if we haven't yet
            setJobs((prev) =>
              prev.map((j) =>
                j.index === jobInfo.index && j.status !== 'generating'
                  ? { ...j, status: 'generating', progress: 50 }
                  : j
              )
            );
            continue;
          }

          const job = history[promptId];
          const jobStatus = job.status?.status_str;

          if (jobStatus === 'error') {
            const msgs = job.status?.messages || [];
            setJobs((prev) =>
              prev.map((j) =>
                j.index === jobInfo.index
                  ? { ...j, status: 'error', error: JSON.stringify(msgs), progress: 0 }
                  : j
              )
            );
            pendingJobs.delete(promptId);
            completed.add(jobInfo.index);
            continue;
          }

          const outputs = job.outputs || {};
          const images = [];
          for (const nodeId of Object.keys(outputs)) {
            for (const img of outputs[nodeId].images || []) {
              images.push(img);
            }
          }

          if (images.length > 0) {
            // Download the image
            const { filename, subfolder = '', type = 'output' } = images[0];
            const params = new URLSearchParams({ filename, subfolder, type });
            const imgResp = await fetch(`${COMFYUI_PROXY}/view?${params.toString()}`);

            if (imgResp.ok) {
              const blob = await imgResp.blob();
              const blobUrl = URL.createObjectURL(blob);

              setJobs((prev) =>
                prev.map((j) =>
                  j.index === jobInfo.index
                    ? { ...j, status: 'done', imageUrl: blobUrl, progress: 100 }
                    : j
                )
              );

              if (onJobComplete) {
                onJobComplete(jobInfo.index, blobUrl);
              }
            } else {
              setJobs((prev) =>
                prev.map((j) =>
                  j.index === jobInfo.index
                    ? { ...j, status: 'error', error: 'Download failed', progress: 0 }
                    : j
                )
              );
            }

            pendingJobs.delete(promptId);
            completed.add(jobInfo.index);
          }
        } catch (err) {
          // Poll error — keep trying
        }
      }

      // Update overall progress
      setJobs((current) => {
        const doneCount = current.filter((j) => j.status === 'done').length;
        const errorCount = current.filter((j) => j.status === 'error').length;
        const total = current.length;
        const pct = total > 0 ? Math.round(((doneCount + errorCount) / total) * 100) : 0;
        setOverallProgress(pct);
        return current;
      });
    }

    if (abortRef.current) {
      setStatus('cancelled');
    } else {
      const hasErrors = jobs.some((j) => j.status === 'error');
      setStatus(hasErrors ? 'error' : 'done');
    }
  }, []);

  return {
    startBatch,
    cancel,
    reset,
    status,
    jobs,
    overallProgress,
    isIdle: status === 'idle',
    isRunning: status === 'running',
    isDone: status === 'done',
    isError: status === 'error',
    isCancelled: status === 'cancelled',
  };
}
