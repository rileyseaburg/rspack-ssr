
// RSPack Generated Bundle
(function() {
    'use strict';
    
    // React runtime check
    if (typeof window !== 'undefined' && window.React && window.ReactDOM) {
        const React = window.React;
        const ReactDOM = window.ReactDOM;
        
        // Simple React component - in real RSPack this would be compiled from TSX
        function App() {
            return React.createElement('div', { className: 'app' },
                React.createElement('h1', null, 'Hello from React + RSPack!'),
                React.createElement('p', null, 'This is a server-side rendered application with Actix Web + Tera.'),
                React.createElement('p', null, 'Built and bundled with RSPack Rust crates!')
            );
        }
        
        // Hydrate the app
        const container = document.getElementById('root');
        if (container) {
            const root = ReactDOM.createRoot(container);
            root.render(React.createElement(App));
        } else {
            console.error('Root element not found');
        }
    } else {
        console.error('React or ReactDOM not available. Please include React libraries.');
    }
})();
