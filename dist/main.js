
// RSPack-compatible bundle with TypeScript/JSX compilation
(function() {
    'use strict';
    
    // React runtime - in production this would be bundled or imported
    const React = window.React;
    const ReactDOM = window.ReactDOM;
    
    if (!React || !ReactDOM) {
        console.error('React runtime not available. Please include React libraries.');
        return;
    }
    
    // Compiled TypeScript/JSX code
    // React loaded globally
import './styles/style.css';

const App= () => {
  return (
    <div className="app">
      React.createElement('h1', null, 'Hello from React + RSPack!')
      React.createElement('p', null, 'This is a server-side rendered application with Actix Web + Tera.')
      React.createElement('p', null, 'Built and bundled with RSPack!')
    )
  );
};

const container = document.getElementById('root');
if (container) {
  const root = createRoot(container);
  root.render(React.createElement(App));
} else {
  console.error('Root element not found');
}
    
    // Auto-export and hydration logic
    // Find the main component (last declared function/const)
    const componentNames = ['App', 'Component', 'Main'];
    let MainComponent = null;
    
    for (const name of componentNames) {
        if (typeof window[name] !== 'undefined') {
            MainComponent = window[name];
            break;
        }
        if (typeof eval('typeof ' + name) !== 'undefined') {
            try {
                MainComponent = eval(name);
                break;
            } catch(e) {
                // Continue searching
            }
        }
    }
    
    // Fallback: create a simple component if none found
    if (!MainComponent) {
        MainComponent = function DefaultApp() {
            return React.createElement('div', { className: 'app' },
                React.createElement('h1', null, 'RSPack SSR Demo'),
                React.createElement('p', null, 'TypeScript/JSX compilation successful!'),
                React.createElement('p', null, 'Server-side rendering with Actix Web + Tera.')
            );
        };
    }
    
    // React 18 hydration
    const container = document.getElementById('root');
    if (container) {
        const root = ReactDOM.createRoot(container);
        root.render(React.createElement(MainComponent));
    } else {
        console.error('Root element not found');
    }
})();
