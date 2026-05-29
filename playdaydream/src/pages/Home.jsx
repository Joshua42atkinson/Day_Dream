import { useNavigate } from 'react-router-dom';
import { useRef, useState } from 'react';
import { getAllAdventures } from '../data/curriculum';
import { adaptStoryGraph } from '../adapters/storygraph';
import './Home.css';

// ════════════════════════════════════════════════════════════
// HOME — Adventure Picker
// ════════════════════════════════════════════════════════════
// The landing page for playdaydream.com.
// Shows available adventures as cards. Tap one to play.

export default function Home() {
  const navigate = useNavigate();
  const adventures = getAllAdventures();
  const fileInputRef = useRef(null);
  const [uploadError, setUploadError] = useState(null);

  const handleFileSelect = (e) => {
    const file = e.target.files?.[0];
    if (!file) return;
    setUploadError(null);

    const reader = new FileReader();
    reader.onload = (ev) => {
      try {
        const json = JSON.parse(ev.target.result);
        const adventure = adaptStoryGraph(json);
        sessionStorage.setItem('daydream_custom_adventure', JSON.stringify(adventure));
        navigate('/custom');
      } catch (err) {
        setUploadError(err.message || 'Invalid StoryGraph JSON');
      }
    };
    reader.readAsText(file);
  };

  return (
    <div className="home-container">
      {/* Ambient background */}
      <div className="home-bg" />

      <div className="home-content">
        {/* Title */}
        <header className="home-header">
          <h1 className="home-title">Daydream</h1>
          <p className="home-subtitle">
            Where words are spells and the path you choose is the lesson.
          </p>
        </header>

        {/* Adventure Cards */}
        <div className="adventure-grid">
          {adventures.map((adv) => {
            // Get first node's image as preview
            const firstNode = adv.nodes[adv.start];
            const previewImage = firstNode?.image || '/images/threshold.png';
            const wordList = Object.values(adv.nodes)
              .map((n) => n.focusWord)
              .filter(Boolean);

            return (
              <button
                key={adv.id}
                className="adventure-card"
                onClick={() => navigate(`/play/${adv.id}`)}
              >
                <div
                  className="adventure-card-bg"
                  style={{ backgroundImage: `url('${previewImage}')` }}
                />
                <div className="adventure-card-overlay" />
                <div className="adventure-card-content">
                  {adv.ageRange && (
                    <span className="adventure-age">{adv.ageRange}</span>
                  )}
                  <h2 className="adventure-card-title">{adv.title}</h2>
                  <p className="adventure-card-desc">{adv.description}</p>
                  <div className="adventure-words">
                    {wordList.map((w) => (
                      <span key={w} className="adventure-word-tag">
                        {w}
                      </span>
                    ))}
                  </div>
                </div>
              </button>
            );
          })}
        </div>

        {/* Custom Adventure Upload */}
        <div className="upload-section">
          <p className="upload-label">Made your own adventure?</p>
          <button
            className="upload-btn"
            onClick={() => fileInputRef.current?.click()}
          >
            Load StoryGraph JSON
          </button>
          <input
            ref={fileInputRef}
            type="file"
            accept=".json,application/json"
            style={{ display: 'none' }}
            onChange={handleFileSelect}
          />
          {uploadError && <p className="upload-error">{uploadError}</p>}
        </div>

        {/* Footer */}
        <footer className="home-footer">
          <p>No accounts. No tracking. Your adventure stays on your device.</p>
        </footer>
      </div>
    </div>
  );
}
