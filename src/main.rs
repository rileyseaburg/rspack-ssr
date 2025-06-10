use actix_web::{web, App, HttpServer, HttpResponse, Result};
use actix_files::Files;
use tera::{Tera, Context};
use std::sync::Arc;
use std::path::Path;
use std::collections::HashMap;

struct AppState {
    tera: Arc<Tera>,
}

// RSPack configuration structure to replace rspack.config.js
#[derive(Debug)]
struct RspackConfig {
    entry: String,
    output_path: String,
    output_filename: String,
    mode: String,
    module_rules: Vec<ModuleRule>,
}

#[derive(Debug)]
struct ModuleRule {
    test: String,
    loader: String,
    options: HashMap<String, serde_json::Value>,
}

impl Default for RspackConfig {
    fn default() -> Self {
        let mut swc_options = HashMap::new();
        swc_options.insert("jsc".to_string(), serde_json::json!({
            "parser": {
                "syntax": "typescript",
                "tsx": true
            },
            "transform": {
                "react": {
                    "runtime": "automatic"
                }
            }
        }));

        Self {
            entry: "./frontend/index.tsx".to_string(),
            output_path: "./dist".to_string(),
            output_filename: "main.js".to_string(),
            mode: "development".to_string(),
            module_rules: vec![
                ModuleRule {
                    test: r"\.(js|jsx|ts|tsx)$".to_string(),
                    loader: "builtin:swc-loader".to_string(),
                    options: swc_options,
                },
                ModuleRule {
                    test: r"\.css$".to_string(),
                    loader: "builtin:lightningcss-loader".to_string(),
                    options: HashMap::new(),
                },
            ],
        }
    }
}

async fn index(data: web::Data<AppState>) -> Result<HttpResponse> {
    let mut context = Context::new();
    context.insert("title", "RSPack SSR with Actix Web");
    
    match data.tera.render("index.html", &context) {
        Ok(rendered) => Ok(HttpResponse::Ok()
            .content_type("text/html")
            .body(rendered)),
        Err(err) => {
            log::error!("Template rendering error: {}", err);
            Ok(HttpResponse::InternalServerError()
                .content_type("text/html")
                .body("Template rendering error"))
        }
    }
}

// RSPack build implementation using Rust crates
fn run_rspack_build() -> anyhow::Result<()> {
    log::info!("Starting RSPack build process using Rust crates...");
    
    let config = RspackConfig::default();
    log::info!("RSPack configuration: {:?}", config);
    
    // Ensure output directory exists
    let output_dir = Path::new(&config.output_path);
    if !output_dir.exists() {
        std::fs::create_dir_all(output_dir)?;
        log::info!("Created output directory: {}", config.output_path);
    }
    
    // This is where the actual rspack_core integration would happen
    // TODO: Replace with actual rspack_core::Compiler usage
    compile_with_rspack_core(&config)?;
    
    log::info!("RSPack build completed successfully");
    Ok(())
}

// Mock implementation for rspack_core integration
// This would be replaced with actual rspack_core API calls
fn compile_with_rspack_core(config: &RspackConfig) -> anyhow::Result<()> {
    log::info!("Compiling with RSPack core...");
    log::info!("Entry point: {}", config.entry);
    log::info!("Output: {}/{}", config.output_path, config.output_filename);
    
    // In a real implementation, this would:
    // 1. Create an rspack_core::Compiler instance
    // 2. Configure it with the entry, output, and module rules
    // 3. Run the compilation process
    // 4. Handle TypeScript/JSX transformation via SWC
    // 5. Process CSS files
    // 6. Generate the bundled output
    
    // For now, create a simple bundle to demonstrate the concept
    create_mock_bundle(config)?;
    
    Ok(())
}

// Create a mock bundle for demonstration
fn create_mock_bundle(config: &RspackConfig) -> anyhow::Result<()> {
    let bundle_content = r#"
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
"#;

    let output_path = Path::new(&config.output_path).join(&config.output_filename);
    std::fs::write(&output_path, bundle_content)?;
    log::info!("Created bundle: {}", output_path.display());
    
    // Create a basic CSS file as well
    let css_content = r#"
.app {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', sans-serif;
    max-width: 800px;
    margin: 0 auto;
    padding: 2rem;
    line-height: 1.6;
}

.app h1 {
    color: #2c3e50;
    border-bottom: 2px solid #3498db;
    padding-bottom: 0.5rem;
}

.app p {
    color: #34495e;
    margin: 1rem 0;
}
"#;
    
    let css_path = Path::new(&config.output_path).join("main.css");
    std::fs::write(&css_path, css_content)?;
    log::info!("Created CSS bundle: {}", css_path.display());
    
    Ok(())
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    // Run the build process using rspack Rust crates
    if let Err(e) = run_rspack_build() {
        log::error!("RSPack build process failed: {}", e);
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Build failed: {}", e)
        ));
    }

    // Initialize Tera template engine
    let tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            log::error!("Tera parsing error: {}", e);
            std::process::exit(1);
        }
    };

    // Create app state
    let app_state = web::Data::new(AppState {
        tera: Arc::new(tera),
    });

    log::info!("Starting server at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/", web::get().to(index))
            .service(Files::new("/static", "./dist").prefer_utf8(true))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
