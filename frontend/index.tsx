import React from 'react';
import { createRoot } from 'react-dom/client';
import './styles/style.css';

const App: React.FC = () => {
  return (
    <div className="app">
      <h1>Hello from React + RSPack!</h1>
      <p>This is a server-side rendered application with Actix Web + Tera.</p>
    </div>
  );
};

const container = document.getElementById('root');
if (container) {
  const root = createRoot(container);
  root.render(<App />);
}