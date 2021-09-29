import React, { useState } from 'react';
import './App.css';

function App() {
  const [url, setUrl] = useState('')

  const submit = (e: any) => {
    e.preventDefault();
    const fullPath = `http://${url}`;
    window.location.href = fullPath
    console.log(fullPath)
  }

  const update = (e: any) => {
    setUrl(e.target.value)
  }

  return (
    <form onSubmit={submit} style={{ width: 510, paddingLeft: 5, paddingRight: 5, backgroundColor: "#333333", position: "absolute", display: 'flex', left: '50%', top: '50%', transform: "translate(-50%, -50%)", fontSize: 30 }}>
      <input style={{ fontSize: 30, width: 400 }} placeholder="192.XXX.XX.X:PORT" value={url} onChange={update} />
      <button type="submit" style={{ fontSize: 30, width: 100 }} > Go </button>
    </form>
  );
}

export default App;
