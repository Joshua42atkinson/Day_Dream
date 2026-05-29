import { useBatchArtist } from '../hooks/useBatchArtist';
import { ARCANA } from '../data/arcana';

// ════════════════════════════════════════════════════════════
// BatchForge — Automated Multi-Scene Art Generation Panel
// ════════════════════════════════════════════════════════════
// One button to generate art for every node that has a word and story.
// Shows per-node status, overall progress, and results.

const STATUS_ICONS = {
  pending: '◌',
  building: '◈',
  queued: '◇',
  generating: '◈',
  done: '✦',
  error: '✕',
};

const STATUS_COLORS = {
  pending: 'var(--ink-faint)',
  building: 'var(--gold-dim)',
  queued: 'var(--mind)',
  generating: 'var(--gold)',
  done: 'var(--success)',
  error: 'var(--danger)',
};

function JobRow({ job }) {
  return (
    <div
      className="flex items-center gap-2"
      style={{
        padding: '6px 10px',
        borderRadius: 'var(--radius-sm)',
        background: 'var(--void-raised)',
        marginBottom: '4px',
      }}
    >
      <span style={{ fontSize: '0.8rem', color: STATUS_COLORS[job.status] || 'var(--ink-muted)' }}>
        {STATUS_ICONS[job.status] || '?'}
      </span>
      <span
        className="font-sans"
        style={{ fontSize: '0.75rem', color: 'var(--ink-primary)', flex: 1, minWidth: 0 }}
      >
        {job.word}
      </span>
      <span
        className="font-sans"
        style={{ fontSize: '0.65rem', color: STATUS_COLORS[job.status], minWidth: '50px', textAlign: 'right' }}
      >
        {job.status === 'done' ? 'Done' : job.status === 'error' ? 'Failed' : `${job.progress}%`}
      </span>
    </div>
  );
}

export default function BatchForge({ nodes, onImageGenerated }) {
  const batch = useBatchArtist();

  const eligibleNodes = nodes
    .map((n, i) => ({ ...n, index: i }))
    .filter((n) => n.word && n.story && !n.image);

  const handleForgeAll = () => {
    const artistNodes = eligibleNodes.map((n) => {
      const wordData = ARCANA.find((a) => a.word === n.word);
      return {
        focusWord: n.word,
        channel: wordData?.channel?.toLowerCase() || 'body',
        story: n.story,
        themes: wordData ? [wordData.desc] : [],
      };
    });

    batch.startBatch(artistNodes, (index, blobUrl) => {
      const nodeIndex = eligibleNodes[index]?.index;
      if (nodeIndex !== undefined && onImageGenerated) {
        onImageGenerated(nodeIndex, blobUrl);
      }
    });
  };

  const doneCount = batch.jobs.filter((j) => j.status === 'done').length;
  const errorCount = batch.jobs.filter((j) => j.status === 'error').length;
  const totalJobs = batch.jobs.length;

  return (
    <div
      className="parchment-panel"
      style={{
        marginBottom: '20px',
        animation: batch.isRunning ? 'glowPulse 2s ease-in-out infinite' : 'none',
      }}
    >
      <div className="flex items-center gap-2" style={{ marginBottom: '12px' }}>
        <span style={{ fontSize: '0.9rem', color: 'var(--gold)' }}>◈</span>
        <h4
          className="font-heading"
          style={{ fontSize: '0.85rem', color: 'var(--gold)', letterSpacing: '0.04em' }}
        >
          The Batch Forge
        </h4>
        <span className="font-sans" style={{ fontSize: '0.6rem', color: 'var(--ink-faint)', marginLeft: 'auto' }}>
          {eligibleNodes.length} scenes ready
        </span>
      </div>

      {/* Overall progress */}
      {batch.isRunning && (
        <div style={{ marginBottom: '12px' }}>
          <div
            style={{
              width: '100%',
              height: '4px',
              background: 'var(--void-raised)',
              borderRadius: 'var(--radius-full)',
              overflow: 'hidden',
            }}
          >
            <div
              style={{
                width: `${batch.overallProgress}%`,
                height: '100%',
                borderRadius: 'var(--radius-full)',
                background: 'var(--gold)',
                transition: 'width 0.8s var(--ease-out-expo)',
              }}
            />
          </div>
          <div className="flex items-center justify-between" style={{ marginTop: '4px' }}>
            <span className="font-sans" style={{ fontSize: '0.65rem', color: 'var(--ink-muted)' }}>
              {doneCount} done · {errorCount} failed · {totalJobs - doneCount - errorCount} remaining
            </span>
            <span className="font-heading" style={{ fontSize: '0.7rem', color: 'var(--gold)' }}>
              {batch.overallProgress}%
            </span>
          </div>
        </div>
      )}

      {/* Job list */}
      {batch.jobs.length > 0 && (
        <div style={{ marginBottom: '12px', maxHeight: '200px', overflowY: 'auto' }}>
          {batch.jobs.map((job) => (
            <JobRow key={job.index} job={job} />
          ))}
        </div>
      )}

      {/* Summary when done */}
      {batch.isDone && (
        <div
          style={{
            padding: '10px 14px',
            borderRadius: 'var(--radius-md)',
            background: 'rgba(16,185,129,0.06)',
            border: '1px solid rgba(16,185,129,0.15)',
            marginBottom: '12px',
          }}
        >
          <p className="font-sans" style={{ fontSize: '0.75rem', color: 'var(--success)' }}>
            ✦ {doneCount} of {totalJobs} scenes forged successfully.
          </p>
        </div>
      )}

      {batch.isError && (
        <div
          style={{
            padding: '10px 14px',
            borderRadius: 'var(--radius-md)',
            background: 'rgba(239,68,68,0.06)',
            border: '1px solid rgba(239,68,68,0.15)',
            marginBottom: '12px',
          }}
        >
          <p className="font-sans" style={{ fontSize: '0.75rem', color: 'var(--danger)' }}>
            ✕ Some scenes failed. Check individual status below.
          </p>
        </div>
      )}

      {/* Action buttons */}
      <div className="flex items-center gap-2" style={{ flexWrap: 'wrap' }}>
        {!batch.isRunning && !batch.isDone && !batch.isError && eligibleNodes.length > 0 && (
          <button className="btn btn-primary" onClick={handleForgeAll}>
            <span>◈</span> Forge All {eligibleNodes.length} Scenes
          </button>
        )}

        {batch.isRunning && (
          <button className="btn btn-ghost" onClick={batch.cancel}>
            <span>✕</span> Stop Forge
          </button>
        )}

        {(batch.isDone || batch.isError || batch.isCancelled) && (
          <button className="btn btn-ghost" onClick={batch.reset}>
            <span>↻</span> Reset
          </button>
        )}
      </div>
    </div>
  );
}
