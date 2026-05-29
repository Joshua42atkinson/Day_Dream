import { useState, useEffect, useCallback, useRef } from 'react';
import { useSwipeGesture } from '../hooks/useSwipeGesture';
import { useAmbientDrone } from '../hooks/useAmbientDrone';
import { useStudentTrail } from '../hooks/useStudentTrail';
import { getAdventure, getDefaultAdventure } from '../data/curriculum';
import './Play.css';

// ════════════════════════════════════════════════════════════
// PLAY — The Daydream Game
// ════════════════════════════════════════════════════════════
// Like RuneScape: loads in browser, runs locally, no server.
// A swipe-card narrative MUD. Each slide is a room.
// The student's path through the word-graph IS the assessment.

const BREATHING_PAUSE_MS = 2600;

export default function Play({ adventureId, adventure: adventureProp, onBack }) {
  const adventure = adventureProp || (adventureId && getAdventure(adventureId)) || getDefaultAdventure();
  const [currentNodeId, setCurrentNodeId] = useState(adventure.start);
  const [exitsRevealed, setExitsRevealed] = useState(false);
  const [depthShowing, setDepthShowing] = useState(false);
  const [slideKey, setSlideKey] = useState(0); // force re-mount on slide change
  const [entryDirection, setEntryDirection] = useState(null);
  const [gameOver, setGameOver] = useState(false);

  const { trail, recordStep, recordDepth, resetTrail, getEmergentClass } = useStudentTrail();
  const { playDrone, fadeDrone, initAudio } = useAmbientDrone();

  const node = adventure.nodes[currentNodeId];
  const hasExits = node && Object.values(node.exits).some((e) => e !== null);

  // ─── Swipe commit handler ──────────────────────────────
  const handleSwipe = useCallback(
    (direction, exit) => {
      initAudio(); // requires user gesture
      fadeDrone();
      recordStep(currentNodeId, direction, exit.virtue, node.focusWord, node.channel);

      // Animate out, then transition
      setTimeout(() => {
        setEntryDirection(direction);
        setCurrentNodeId(exit.to);
        setSlideKey((k) => k + 1);
        setExitsRevealed(false);
        setDepthShowing(false);
      }, 400);
    },
    [currentNodeId, node, fadeDrone, initAudio, recordStep]
  );

  const handleDoubleTap = useCallback(() => {
    if (!depthShowing && node?.depth) {
      setDepthShowing(true);
      recordDepth(node.focusWord);
    }
  }, [depthShowing, node, recordDepth]);

  // ─── Swipe gesture ────────────────────────────────────
  const exits = exitsRevealed ? node.exits : { up: null, down: null, left: null, right: null };
  const { cardTransform, activeDir, magnitude, onMouseDown, onTouchStart } =
    useSwipeGesture({
      exits,
      onSwipe: handleSwipe,
      onDoubleTap: handleDoubleTap,
    });

  // ─── Breathing pause → reveal exits ───────────────────
  useEffect(() => {
    if (!node || !hasExits) return;
    setExitsRevealed(false);

    const timer = setTimeout(() => {
      setExitsRevealed(true);
    }, BREATHING_PAUSE_MS);

    return () => clearTimeout(timer);
  }, [slideKey, node, hasExits]);

  // ─── Play drone for this slide ────────────────────────
  useEffect(() => {
    if (node?.droneHz) playDrone(node.droneHz);
  }, [slideKey, node, playDrone]);

  // ─── End detection ────────────────────────────────────
  useEffect(() => {
    if (node && !hasExits) {
      fadeDrone();
      const timer = setTimeout(() => setGameOver(true), 1200);
      return () => clearTimeout(timer);
    }
  }, [node, hasExits, fadeDrone]);

  // ─── Restart handler ─────────────────────────────────
  const handleRestart = useCallback(() => {
    resetTrail();
    setCurrentNodeId(adventure.start);
    setSlideKey((k) => k + 1);
    setEntryDirection(null);
    setGameOver(false);
    setDepthShowing(false);
  }, [adventure, resetTrail]);

  if (!node) return null;

  // ─── Entry animation class ────────────────────────────
  const entryOffset = {
    up: 'translateY(100%)',
    down: 'translateY(-100%)',
    left: 'translateX(100%)',
    right: 'translateX(-100%)',
  };

  return (
    <div id="daydream-game" className="play-container">
      {/* ── THE SLIDE CARD ─────────────────────────── */}
      <div
        key={slideKey}
        className={`slide-card ${exitsRevealed ? 'interactive' : 'entering'}`}
        style={exitsRevealed ? cardTransform : undefined}
        onMouseDown={exitsRevealed ? onMouseDown : undefined}
        onTouchStart={exitsRevealed ? onTouchStart : undefined}
      >
        {/* Background image */}
        <div
          className="slide-bg"
          style={{ backgroundImage: `url('${node.image}')` }}
        />
        <div className="slide-overlay" />

        {/* Breathing ring */}
        <div className={`breathing-ring ${!exitsRevealed && hasExits ? 'active' : ''}`} />

        {/* Exit hints (edge labels) */}
        {exitsRevealed &&
          ['up', 'down', 'left', 'right'].map(
            (dir) =>
              node.exits[dir] && (
                <div
                  key={dir}
                  className={`exit-hint ${dir} ${exitsRevealed ? 'visible' : ''}`}
                  style={{
                    opacity:
                      activeDir && magnitude > 20
                        ? activeDir === dir
                          ? Math.min(1, magnitude / 120)
                          : 0.1
                        : 0.4,
                  }}
                >
                  <span className="arrow">
                    {{ up: '↑', down: '↓', left: '←', right: '→' }[dir]}
                  </span>
                  <span>{node.exits[dir].label}</span>
                </div>
              )
          )}

        {/* Direction label (active during drag) */}
        {activeDir && node.exits[activeDir] && (
          <div
            className={`direction-label ${activeDir}`}
            style={{ color: `rgba(255,255,255,${Math.min(1, magnitude / 150)})` }}
          >
            <span className="dir-arrow">
              {{ up: '↑', down: '↓', left: '←', right: '→' }[activeDir]}
            </span>
            <span>{node.exits[activeDir].label}</span>
          </div>
        )}

        {/* Story content */}
        <div className="slide-content">
          <div className="focus-word">{node.focusWord}</div>
          <h1 className="slide-title">{node.title}</h1>
          <p className="slide-story">{node.story}</p>
        </div>
      </div>

      {/* ── DEPTH OVERLAY (double-tap) ────────────── */}
      {depthShowing && (
        <div
          className={`depth-overlay ${depthShowing ? 'visible' : ''}`}
          onClick={() => setDepthShowing(false)}
        >
          <div className="depth-word">{node.focusWord}</div>
          <div className="depth-question">{node.depth}</div>
          <div className="depth-dismiss">tap anywhere to return</div>
        </div>
      )}

      {/* ── PROGRESS DOTS ─────────────────────────── */}
      <div className="progress-dots">
        {Object.keys(adventure.nodes).map((id) => (
          <div
            key={id}
            className={`progress-dot ${
              trail.steps.some((s) => s.nodeId === id) ? 'visited' : ''
            } ${id === currentNodeId ? 'active' : ''}`}
          />
        ))}
      </div>

      {/* ── END SCREEN (SpellBook Preview) ────────── */}
      {gameOver && (
        <div className="end-overlay visible">
          <div className="end-title">Your journey, traced.</div>
          <div className="end-subtitle">{getEmergentClass()}</div>
          <div className="trail-container">
            {trail.steps.map((step, i) => (
              <div key={i} className="trail-step">
                <span className="trail-word">{step.focusWord}</span>
                <span className="trail-arrow">
                  {{ up: '↑', down: '↓', left: '←', right: '→' }[step.direction]}
                </span>
              </div>
            ))}
          </div>
          <div className="end-pattern">
            {trail.steps.length > 0 && (
              <>
                You chose{' '}
                <strong>
                  {
                    Object.entries(
                      trail.steps.reduce((acc, s) => {
                        acc[s.virtue] = (acc[s.virtue] || 0) + 1;
                        return acc;
                      }, {})
                    )
                      .sort((a, b) => b[1] - a[1])
                      .map(([v, c]) => `${v} ${c}×`)
                      .join(', ')
                  }
                </strong>
              </>
            )}
          </div>
          <button className="restart-btn" onClick={handleRestart}>
            Begin Again
          </button>
        </div>
      )}

      {/* ── BACK BUTTON ─────────────────────────── */}
      {onBack && (
        <button className="back-btn" onClick={onBack} aria-label="Back to adventures">
          ←
        </button>
      )}

      {/* ── KEY HINT ──────────────────────────────── */}
      <div className="key-hint">swipe to navigate · double-tap to dig deeper</div>
    </div>
  );
}
