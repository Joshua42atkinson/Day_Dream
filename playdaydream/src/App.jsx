import { BrowserRouter, Routes, Route, Navigate, useParams, useNavigate } from 'react-router-dom'
import Home from './pages/Home'
import Play from './pages/Play'
import AudioPlay from './pages/AudioPlay'
import CharacterCreator from './pages/CharacterCreator'
import DeckBuilder from './pages/DeckBuilder'
import JourneyAuthor from './pages/JourneyAuthor'
import Settings from './pages/Settings'
import PlayerCodex from './pages/PlayerCodex'
import { useSettings } from './hooks/useSettings'
import { useCharacter } from './hooks/useCharacter'

function PlayRoute() {
  const { adventureId } = useParams()
  const navigate = useNavigate()
  return <Play adventureId={adventureId} onBack={() => navigate('/')} />
}

function CustomPlayRoute() {
  const navigate = useNavigate()
  const adventure = (() => {
    try {
      const raw = sessionStorage.getItem('daydream_custom_adventure')
      return raw ? JSON.parse(raw) : null
    } catch {
      return null
    }
  })()

  if (!adventure) {
    return <Navigate to="/" replace />
  }

  return <Play adventure={adventure} onBack={() => navigate('/')} />
}

function AudioPlayRoute() {
  const { adventureId } = useParams()
  const navigate = useNavigate()
  return <AudioPlay adventureId={adventureId} onBack={() => navigate('/')} />
}

function SettingsRoute() {
  const navigate = useNavigate()
  const settingsHook = useSettings()
  return <Settings settingsHook={settingsHook} />
}

function CharacterCreatorRoute() {
  const characterHook = useCharacter()
  return <CharacterCreator characterHook={characterHook} />
}

function DeckBuilderRoute() {
  const characterHook = useCharacter()
  return <DeckBuilder characterHook={characterHook} />
}

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/play/:adventureId" element={<PlayRoute />} />
        <Route path="/custom" element={<CustomPlayRoute />} />
        <Route path="/audio/:adventureId" element={<AudioPlayRoute />} />
        <Route path="/create/character" element={<CharacterCreatorRoute />} />
        <Route path="/create/deck" element={<DeckBuilderRoute />} />
        <Route path="/create/journey" element={<JourneyAuthor />} />
        <Route path="/settings" element={<SettingsRoute />} />
        <Route path="/codex" element={<PlayerCodex />} />
        <Route path="*" element={<Navigate to="/" replace />} />
      </Routes>
    </BrowserRouter>
  )
}

export default App
