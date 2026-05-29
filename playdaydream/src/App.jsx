import { BrowserRouter, Routes, Route, Navigate, useParams, useNavigate } from 'react-router-dom'
import Home from './pages/Home'
import Play from './pages/Play'

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

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/play/:adventureId" element={<PlayRoute />} />
        <Route path="/custom" element={<CustomPlayRoute />} />
        <Route path="*" element={<Navigate to="/" replace />} />
      </Routes>
    </BrowserRouter>
  )
}

export default App
