
// RSPack-compiled bundle (mock implementation)
(function() {
    'use strict';
    
    // Mock React and ReactDOM for demonstration
    window.React = {
        createElement: function(type, props, ...children) {
            const element = document.createElement(type);
            if (props) {
                for (const [key, value] of Object.entries(props)) {
                    if (key.startsWith('on')) {
                        element.addEventListener(key.slice(2).toLowerCase(), value);
                    } else if (key === 'className') {
                        element.className = value;
                    } else {
                        element.setAttribute(key, value);
                    }
                }
            }
            children.forEach(child => {
                if (typeof child === 'string') {
                    element.appendChild(document.createTextNode(child));
                } else {
                    element.appendChild(child);
                }
            });
            return element;
        }
    };
    
    window.ReactDOM = {
        createRoot: function(container) {
            return {
                render: function(element) {
                    container.appendChild(element);
                }
            };
        }
    };
    
    // Compiled React application
    function App() {
        return React.createElement('div', { className: 'app' },
            React.createElement('h1', null, 'Hello from React + RSPack!'),
            React.createElement('p', null, 'This is a server-side rendered application with Actix Web + Tera.'),
            React.createElement('p', null, 'Built and bundled with RSPack Rust crates!')
        );
    }
    
    // Initialize the app
    document.addEventListener('DOMContentLoaded', function() {
        const container = document.getElementById('root');
        if (container) {
            const root = ReactDOM.createRoot(container);
            root.render(App());
        } else {
            console.error('Root element not found');
        }
    });
})();
