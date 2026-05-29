import { useState } from 'react';
import { useSceneArtist } from '../hooks/useSceneArtist';

// ════════════════════════════════════════════════════════════
// SceneArtist — The Forge UI
// ════════════════════════════════════════════════════════════
// A LitRPG-themed art generation component for adventure nodes.
// Uses the local ComfyUI + LongCat-Image pipeline.
//
// Shows: prompt preview, generation orb, progress bar, result image.

function GenerationOrb({ progress, status }) {
  const isActive = status === 'generating' || status === 'submitting';
  const channelColors = {
    building: 'var(--gold)',
    submitting: 'var(--mind)',
    generating: 'var(--gold)',
  };
  const color = channelColors[status] || 'var(--ink-muted)';

  return (
    <div
      className="flex items-center justify-center"
      style={{
        width: '80px',
        height: '80px',
        borderRadius: '50%',
        position: 'relative',
        background: `radial-gradient(circle, ${color}20 0%, transparent 70%)`,
        border: `1px solid ${color}30`,
        margin: '0 auto 16px',
      }}
    >
      {isActive && (
        <div
          style={{
            position: 'absolute',
            inset: '-6px',
            borderRadius: '50%',
            border: `1px solid ${color}20`,
            animation: 'orbPulse 2s ease-in-out infinite',
          }}
        />
      )}
      <span
        className="font-heading"
        style={{
          fontSize: '1.8rem',
          color,
          opacity: isActive ? 1 : 0.4,
          transition: 'opacity 0.3s ease',
        }}
      >
        {status === 'done' ? '✦' : status === 'error' ? '✕' : '◈'}
      </span>
    </div>
  );
}

function PromptPreview({ prompt }) {
  const [expanded, setExpanded] = useState(false);
  if (!prompt) return null;

  return (
    <div
      style={{
        marginBottom: '16px',
        padding: '12px 14px',
        borderRadius: 'var(--radius-md)',
        background: 'var(--void-raised)',
        border: '1px solid rgba(255,255,255,0.06)',
      }}
    >
      <div
        className="flex items-center justify-between"
        style={{ cursor: 'pointer' }}
        onClick={() => setExpanded(!expanded)}
      >
        <span
          className="font-sans"
          style={{ fontSize: '0.7rem', color: 'var(--ink-muted)', letterSpacing: '0.1em', textTransform: 'uppercase' }}
        >
          Visual Prompt
        </span>
        <span style={{ fontSize: '0.7rem', color: 'var(--gold-dim)' }}>
          {expanded ? '▲' : '▼'}
        </span>
      </div>
      {expanded && (
        <p
          className="font-serif"
          style={{
            marginTop: '8px',
            fontSize: '0.8rem',
            lineHeight: 1.6,
            color: 'var(--ink-secondary)',
            fontStyle: 'italic',
          }}
        >
          {prompt}
        </p>
      )}
    </div>
  );
}

function ProgressBar({ progress, status }) {
  const colors = {
    building: 'var(--gold-dim)',
    submitting: 'var(--mind)',
    generating: 'var(--gold)',
    done: 'var(--success)',
    error: 'var(--danger)',
  };
  const color = colors[status] || 'var(--ink-muted)';

  return (
    <div style={{ marginBottom: '16px' }}>
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
            width: `${progress}%`,
            height: '100%',
            borderRadius: 'var(--radius-full)',
            background: color,
            transition: 'width 0.8s var(--ease-out-expo)',
          }}
        />
      </div>
      <div
        className="flex items-center justify-between"
        style={{ marginTop: '6px' }}
      >
        <span className="font-sans" style={{ fontSize: '0.65rem', color: 'var(--ink-muted)', letterSpacing: '0.08em', textTransform: 'uppercase' }}>
          {status === 'building' && 'Forging prompt...'}
          {status === 'submitting' && 'Summoning the forge...'}
          {status === 'generating' && 'The artist paints...'}
          {status === 'done' && 'Scene complete'}
          {status === 'error' && 'The spell faltered'}
          {status === 'idle' && 'Ready'}
        </span>
        <span className="font-heading" style={{ fontSize: '0.7rem', color }}>
          {progress}%
        </span>
      </div>
    </div>
  );
}

export default function SceneArtist({ node, onImageGenerated }) {
  const artist = useSceneArtist();

  const handleGenerate = async () => {
    try {
      const blobUrl = await artist.generate(node);
      if (blobUrl && onImageGenerated) {
        onImageGenerated(blobUrl);
      }
    } catch {
      // Error already set in hook
    }
  };

  const handleUseImage = () => {
    if (artist.imageUrl && onImageGenerated) {
      onImageGenerated(artist.imageUrl);
    }
  };

  const isActive = artist.isBuilding || artist.isSubmitting || artist.isGenerating;

  return (
    <div
      className="parchment-panel"
      style={{
        marginTop: '16px',
        marginBottom: '16px',
        animation: artist.status !== 'idle' ? 'fadeInUp 0.4s var(--ease-out-expo)' : 'none',
      }}
    >
      <div className="flex items-center gap-2" style={{ marginBottom: '12px' }}>
        <span style={{ fontSize: '0.9rem', color: 'var(--gold)' }}>◈</span>
        <h4
          className="font-heading"
          style={{ fontSize: '0.85rem', color: 'var(--gold)', letterSpacing: '0.04em' }}
        >
          The Forge
        </h4>
        <span className="font-sans" style={{ fontSize: '0.6rem', color: 'var(--ink-faint)', marginLeft: 'auto' }}>
          LongCat-Image via ComfyUI
        </span>
      </div>

      {/* Prompt preview */}
      <PromptPreview prompt={artist.promptPreview} />

      {/* Orb + Progress */}
      {artist.status !== 'idle' && (
        <>
          <GenerationOrb progress={artist.progress} status={artist.status} />
          <ProgressBar progress={artist.progress} status={artist.status} />
        </>
      )}

      {/* Generated image preview */}
      {artist.imageUrl && (
        <div
          className="animate-fade-in-scale"
          style={{
            marginBottom: '16px',
            borderRadius: 'var(--radius-md)',
            overflow: 'hidden',
            border: '1px solid rgba(201,168,76,0.2)',
          }}
        >
          <img
            src={artist.imageUrl}
            alt="Generated scene"
            style={{
              width: '100%',
              height: 'auto',
              display: 'block',
              filter: 'brightness(0.9)',
            }}
          />
        </div>
      )}

      {/* Error */}
      {artist.error && (
        <div
          style={{
            padding: '10px 14px',
            borderRadius: 'var(--radius-md)',
            background: 'rgba(239,68,68,0.08)',
            border: '1px solid rgba(239,68,68,0.2)',
            marginBottom: '12px',
          }}
        >
          <p className="font-sans" style={{ fontSize: '0.75rem', color: 'var(--danger)' }}>
            {artist.error}
          </p>
        </div>
      )}

      {/* Action buttons */}
      <div className="flex items-center gap-2" style={{ flexWrap: 'wrap' }}>
        {!isActive && !artist.imageUrl && (
          <button className="btn btn-primary" onClick={handleGenerate}>
            <span>◈</span> Forge Scene
          </button>
        )}

        {isActive && (
          <button className="btn btn-ghost" onClick={artist.cancel}>
            <span>✕</span> Cancel
          </button>
        )}

        {artist.imageUrl && (
          <>
            <button className="btn btn-primary" onClick={handleUseImage}>
              <span>✦</span> Use This Image
            </button>
            <button className="btn btn-ghost" onClick={handleGenerate}>
              <span>↻</span> Reforge
            </button>
            <button className="btn btn-ghost" onClick={artist.reset}>
              <span>✕</span> Dismiss
            </button>
          </>
        )}

        {artist.isError && !isActive && (
          <button className="btn btn-primary" onClick={handleGenerate}>
            <span>↻</span> Try Again
          </button>
        )}
      </div>
    </div>
  );
}
