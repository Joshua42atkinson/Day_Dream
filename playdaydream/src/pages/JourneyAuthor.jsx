import { useState } from 'react';
import { useNavigate, useLocation } from 'react-router-dom';
import { ARCANA, Symbols } from '../data/arcana';
import { Channel } from '../data/constants';
import SceneArtist from '../components/SceneArtist';
import BatchForge from '../components/BatchForge';

// ─── Styles ────────────────────────────────────────────────
const styles = {
  page: {
    minHeight: '100vh',
    background: 'linear-gradient(180deg, #0a0a0f 0%, #12121f 100%)',
    color: '#e2e8f0',
    fontFamily: "'Inter', system-ui, sans-serif",
    padding: '24px 16px',
    maxWidth: '700px',
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
  label: {
    fontSize: '12px',
    textTransform: 'uppercase',
    letterSpacing: '2px',
    color: '#06b6d4',
    marginBottom: '8px',
    fontWeight: 600,
    display: 'block',
  },
  input: {
    width: '100%',
    padding: '12px 14px',
    borderRadius: '10px',
    border: '1px solid rgba(255,255,255,0.1)',
    background: 'rgba(255,255,255,0.05)',
    color: '#e2e8f0',
    fontSize: '15px',
    fontFamily: "'Inter', sans-serif",
    outline: 'none',
    marginBottom: '16px',
    boxSizing: 'border-box',
  },
  textarea: {
    width: '100%',
    padding: '12px 14px',
    borderRadius: '10px',
    border: '1px solid rgba(255,255,255,0.1)',
    background: 'rgba(255,255,255,0.05)',
    color: '#e2e8f0',
    fontSize: '15px',
    fontFamily: "'Inter', sans-serif",
    outline: 'none',
    resize: 'vertical',
    minHeight: '80px',
    marginBottom: '16px',
    boxSizing: 'border-box',
  },
  select: {
    width: '100%',
    padding: '12px 14px',
    borderRadius: '10px',
    border: '1px solid rgba(255,255,255,0.1)',
    background: 'rgba(255,255,255,0.05)',
    color: '#e2e8f0',
    fontSize: '15px',
    fontFamily: "'Inter', sans-serif",
    outline: 'none',
    marginBottom: '16px',
    boxSizing: 'border-box',
  },
  nodeCard: {
    background: 'rgba(26,26,46,0.8)',
    border: '1px solid rgba(6,182,212,0.15)',
    borderRadius: '16px',
    padding: '20px',
    marginBottom: '16px',
  },
  nodeHeader: {
    display: 'flex',
    justifyContent: 'space-between',
    alignItems: 'center',
    marginBottom: '16px',
  },
  nodeNum: {
    fontSize: '12px',
    color: '#06b6d4',
    fontWeight: 600,
  },
  removeNodeBtn: {
    background: 'rgba(239,68,68,0.2)',
    border: 'none',
    color: '#ef4444',
    borderRadius: '8px',
    padding: '4px 10px',
    fontSize: '12px',
    cursor: 'pointer',
  },
  madlibRow: {
    display: 'grid',
    gridTemplateColumns: '1fr 1fr',
    gap: '10px',
    marginBottom: '12px',
  },
  madlibLabel: {
    fontSize: '11px',
    color: '#94a3b8',
    marginBottom: '4px',
    display: 'flex',
    alignItems: 'center',
    gap: '4px',
  },
  madlibSymbol: {
    fontSize: '14px',
    color: '#06b6d4',
  },
  addNodeBtn: {
    width: '100%',
    padding: '16px',
    borderRadius: '16px',
    border: '2px dashed rgba(6,182,212,0.3)',
    background: 'rgba(6,182,212,0.05)',
    color: '#06b6d4',
    fontSize: '15px',
    cursor: 'pointer',
    marginBottom: '24px',
  },
  actionRow: {
    display: 'grid',
    gridTemplateColumns: '1fr 1fr',
    gap: '10px',
    marginBottom: '16px',
  },
  exportBtn: {
    padding: '16px',
    borderRadius: '16px',
    border: 'none',
    background: 'linear-gradient(135deg, #06b6d4, #0891b2)',
    color: '#fff',
    fontSize: '15px',
    fontWeight: 600,
    cursor: 'pointer',
  },
  testBtn: {
    padding: '16px',
    borderRadius: '16px',
    border: '1px solid rgba(6,182,212,0.3)',
    background: 'rgba(6,182,212,0.1)',
    color: '#06b6d4',
    fontSize: '15px',
    fontWeight: 600,
    cursor: 'pointer',
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
  },
  wordBadge: {
    display: 'inline-flex',
    alignItems: 'center',
    gap: '4px',
    padding: '4px 8px',
    borderRadius: '6px',
    fontSize: '12px',
    fontWeight: 500,
  },
};

// ─── Mad Libs Slot Types ───────────────────────────────────
const MADLIB_SLOTS = [
  { key: 'setting',   symbol: Symbols.STONE, label: 'Setting',   placeholder: 'an ancient library', desc: 'Where does this scene take place?' },
  { key: 'subject',   symbol: Symbols.STAR,  label: 'Subject',   placeholder: 'the old keeper', desc: 'Who or what is the focus?' },
  { key: 'action',    symbol: Symbols.SPARK, label: 'Action',    placeholder: 'whispers a secret', desc: 'What happens?' },
  { key: 'modifier',  symbol: Symbols.PRISM, label: 'Modifier',  placeholder: 'with trembling hands', desc: 'How does it happen?' },
];

const EMPTY_NODE = () => ({
  id: `node_${Date.now()}_${Math.random().toString(36).slice(2, 6)}`,
  word: '',
  story: '',
  madlibs: { setting: '', subject: '', action: '', modifier: '' },
  depthQuestion: '',
  connectTo: '',
  image: '',
});

export default function JourneyAuthor() {
  const navigate = useNavigate();
  const location = useLocation();
  const { characterId, deck } = location.state || {};

  const [journeyTitle, setJourneyTitle] = useState('');
  const [journeyDesc, setJourneyDesc] = useState('');
  const [nodes, setNodes] = useState([EMPTY_NODE()]);

  const updateNode = (index, patch) => {
    setNodes(prev => prev.map((n, i) => i === index ? { ...n, ...patch } : n));
  };

  const addNode = () => {
    setNodes(prev => [...prev, EMPTY_NODE()]);
  };

  const removeNode = (index) => {
    setNodes(prev => prev.filter((_, i) => i !== index));
  };

  const buildStoryGraph = () => {
    const graphNodes = {};
    const connections = [];

    nodes.forEach((node, i) => {
      const wordData = ARCANA.find(a => a.word === node.word);
      // Build story with Mad Libs filled in
      let story = node.story;
      if (node.madlibs.setting) story = story.replace(/\[setting\]/gi, node.madlibs.setting);
      if (node.madlibs.subject) story = story.replace(/\[subject\]/gi, node.madlibs.subject);
      if (node.madlibs.action) story = story.replace(/\[action\]/gi, node.madlibs.action);
      if (node.madlibs.modifier) story = story.replace(/\[modifier\]/gi, node.madlibs.modifier);

      graphNodes[node.id] = {
        id: node.id,
        title: `${wordData?.word || 'Untitled'} — ${node.madlibs.setting || 'Somewhere'}`,
        focusWord: wordData?.word || '',
        channel: wordData?.channel || Channel.MIND,
        story,
        image: node.image || '',
        depth: node.depthQuestion,
        exits: {},
        droneHz: 174.6, // default
      };

      if (node.connectTo && graphNodes[node.connectTo]) {
        connections.push({ from: node.connectTo, to: node.id });
        // Add exit to previous node
        const prevNode = graphNodes[node.connectTo];
        const exitKey = Object.keys(prevNode.exits).length === 0 ? 'up' :
                        Object.keys(prevNode.exits).length === 1 ? 'right' : 'down';
        prevNode.exits[exitKey] = {
          label: `Continue to ${wordData?.word || 'next'}`,
          to: node.id,
          virtue: 'curiosity',
        };
      }
    });

    // First node has no prerequisites, make it the start
    const startNode = nodes[0]?.id || '';

    return {
      id: `journey_${Date.now()}`,
      title: journeyTitle.trim() || 'Untitled Journey',
      description: journeyDesc,
      start: startNode,
      nodes: graphNodes,
      connections,
      characterId,
      deck: deck || [],
    };
  };

  const handleExport = () => {
    const graph = buildStoryGraph();
    const blob = new Blob([JSON.stringify(graph, null, 2)], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `${graph.title.replace(/\s+/g, '_')}.json`;
    a.click();
    URL.revokeObjectURL(url);
  };

  const handleTestRun = () => {
    const graph = buildStoryGraph();
    sessionStorage.setItem('daydream_custom_adventure', JSON.stringify(graph));
    navigate('/custom');
  };

  const availableWords = deck?.length ? deck : ARCANA.map(a => a.word);

  return (
    <div style={styles.page}>
      <div style={styles.header}>
        <h1 style={styles.title}>Author Your Journey</h1>
        <p style={styles.subtitle}>
          Build a word-DAG. Each node is a scene. Mad Libs make it your own.
        </p>
      </div>

      <label style={styles.label}>Journey Title</label>
      <input
        style={styles.input}
        value={journeyTitle}
        onChange={e => setJourneyTitle(e.target.value)}
        placeholder="e.g., The Mirror's Edge"
      />

      <label style={styles.label}>Description</label>
      <textarea
        style={styles.textarea}
        value={journeyDesc}
        onChange={e => setJourneyDesc(e.target.value)}
        placeholder="What is this journey about?"
      />

      {/* ─── BATCH FORGE ────────────────────────────────── */}
      <BatchForge
        nodes={nodes}
        onImageGenerated={(nodeIndex, blobUrl) => updateNode(nodeIndex, { image: blobUrl })}
      />

      {/* ─── NODES ──────────────────────────────────────── */}
      {nodes.map((node, i) => (
        <div key={node.id} style={styles.nodeCard}>
          <div style={styles.nodeHeader}>
            <span style={styles.nodeNum}>Node {i + 1}</span>
            {nodes.length > 1 && (
              <button style={styles.removeNodeBtn} onClick={() => removeNode(i)}>Remove</button>
            )}
          </div>

          <label style={styles.label}>Power Word</label>
          <select
            style={styles.select}
            value={node.word}
            onChange={e => updateNode(i, { word: e.target.value })}
          >
            <option value="">Select a word...</option>
            {availableWords.map(w => {
              const a = ARCANA.find(x => x.word === w);
              return (
                <option key={w} value={w}>
                  {a?.symbol || ''} {w} ({a?.channel})
                </option>
              );
            })}
          </select>

          <label style={styles.label}>Story Text</label>
          <textarea
            style={styles.textarea}
            value={node.story}
            onChange={e => updateNode(i, { story: e.target.value })}
            placeholder="You enter [setting]. [subject] [action] [modifier]. The word [word] hangs in the air..."
          />

          <label style={styles.label}>Mad Libs Slots</label>
          <div style={styles.madlibRow}>
            {MADLIB_SLOTS.map(slot => (
              <div key={slot.key}>
                <div style={styles.madlibLabel}>
                  <span style={styles.madlibSymbol}>{slot.symbol}</span>
                  {slot.label}
                </div>
                <input
                  style={{ ...styles.input, marginBottom: 0 }}
                  value={node.madlibs[slot.key]}
                  onChange={e => updateNode(i, { madlibs: { ...node.madlibs, [slot.key]: e.target.value } })}
                  placeholder={slot.placeholder}
                />
              </div>
            ))}
          </div>

          <label style={styles.label}>Depth Question (Socratic)</label>
          <input
            style={styles.input}
            value={node.depthQuestion}
            onChange={e => updateNode(i, { depthQuestion: e.target.value })}
            placeholder="What does this word reveal about who you are becoming?"
          />

          {/* Scene Art Generation */}
          {(() => {
            const wordData = ARCANA.find(a => a.word === node.word);
            const artistNode = {
              focusWord: node.word,
              channel: wordData?.channel?.toLowerCase() || 'body',
              story: node.story,
              themes: wordData ? [wordData.desc] : [],
            };
            return (
              <SceneArtist
                node={artistNode}
                onImageGenerated={(url) => updateNode(i, { image: url })}
              />
            );
          })()}

          {/* Image preview if generated */}
          {node.image && (
            <div style={{ marginTop: '12px', borderRadius: '10px', overflow: 'hidden', border: '1px solid rgba(201,168,76,0.2)' }}>
              <img src={node.image} alt="Scene" style={{ width: '100%', height: 'auto', display: 'block', filter: 'brightness(0.9)' }} />
            </div>
          )}

          {i > 0 && (
            <>
              <label style={styles.label}>Connect To Previous</label>
              <select
                style={styles.select}
                value={node.connectTo}
                onChange={e => updateNode(i, { connectTo: e.target.value })}
              >
                <option value="">Select a previous node...</option>
                {nodes.slice(0, i).map((n, idx) => (
                  <option key={n.id} value={n.id}>
                    Node {idx + 1}: {n.word || 'Untitled'}
                  </option>
                ))}
              </select>
            </>
          )}
        </div>
      ))}

      <button style={styles.addNodeBtn} onClick={addNode}>+ Add Node</button>

      <div style={styles.actionRow}>
        <button style={styles.exportBtn} onClick={handleExport}>Export JSON</button>
        <button style={styles.testBtn} onClick={handleTestRun}>Test Run →</button>
      </div>

      <button style={styles.backBtn} onClick={() => navigate(-1)}>Back</button>
    </div>
  );
}
