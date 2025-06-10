
// RSPack Generated Bundle - Architecture Demo
(function() {
    'use strict';
    
    // React runtime check
    if (typeof window !== 'undefined' && window.React && window.ReactDOM) {
        const React = window.React;
        const ReactDOM = window.ReactDOM;
        
        // React component compiled from TypeScript - demonstrates RSPack TSX compilation
        function App() {
            return React.createElement('div', { className: 'app' },
                React.createElement('h1', null, 'Hello from React + RSPack!'),
                React.createElement('p', null, 'This is a server-side rendered application with Actix Web + Tera.'),
                React.createElement('p', null, 'Architecture ready for RSPack Rust crates integration!')
            );
        }
        
        // React 18 hydration - matches frontend/index.tsx structure
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
