// Compiled from TypeScript/JSX using Rust-based compiler
(function() {
    'use strict';
    
    // React runtime (in a real implementation, this would be bundled)
    if (typeof window !== 'undefined' && !window.React) {
        console.error('React not found. Please include React before this script.');
        return;
    }
    
    const React = window.React || {};
    const ReactDOM = window.ReactDOM || {};
    
    // React imported via global
// ReactDOM imported via global
// CSS loaded separately

const App = () => {
  return 
    React.createElement('div', {className: 'app'}, 
      React.createElement('h1', null, 'Hello from React + RSPack!')
      React.createElement('p', null, 'This is a server-side rendered application with Actix Web + Tera.')
      React.createElement('p', null, 'Built and bundled with RSPack!')
    )
  ;
};

const container = document.getElementById('root');
if (container) {
  const root = ReactDOM.createRoot(container);
  root.render(React.createElement(App));
} else {
  console.error('Root element not found');
}
})();
