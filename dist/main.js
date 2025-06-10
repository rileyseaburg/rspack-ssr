console.log('Hello from main.js - placeholder file');

// This is a placeholder until RSPack builds the actual React bundle
document.addEventListener('DOMContentLoaded', function() {
    const root = document.getElementById('root');
    if (root) {
        root.innerHTML = '<div class="app"><h1>Hello from Placeholder JavaScript!</h1><p>This will be replaced by React when RSPack build is integrated.</p></div>';
    }
});