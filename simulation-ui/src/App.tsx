import { useEffect, useState } from 'react'
import { whose_a_good_boy } from '../pkg'

function App() {
  const [response, setResponse] = useState<string>("Waiting for Rust...");

  useEffect(() => {
    const answer = whose_a_good_boy();
    setResponse(answer);
  }, [])

  return (
    <div style={{ display: 'flex', justifyContent: 'center', alignItems: 'center', height: '100vh' }}>
      <h1>
        Who's a good boy? <span style={{ color: 'red' }}>{response}</span>
      </h1>
    </div>
  )
}

export default App