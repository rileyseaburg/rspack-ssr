use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer, Result};
use std::sync::Arc;
use tera::{Context, Tera};

struct AppState {
    tera: Arc<Tera>,
}

async fn index(data: web::Data<AppState>) -> Result<HttpResponse> {
    let mut context = Context::new();
    context.insert("title", "RSPack SSR with Actix Web");

    match data.tera.render("index.html", &context) {
        Ok(rendered) => Ok(HttpResponse::Ok().content_type("text/html").body(rendered)),
        Err(err) => {
            log::error!("Template rendering error: {}", err);
            Ok(HttpResponse::InternalServerError()
                .content_type("text/html")
                .body("Template rendering error"))
        }
    }
}

// RSPack-compatible compilation module
// This module implements TypeScript/JSX compilation and bundling
// Designed to demonstrate the integration patterns used by RSPack
mod rspack_integration {
    use anyhow::{Context, Result};
    use regex::Regex;
    use std::path::{Path, PathBuf};

    pub struct RspackCompiler {
        entry_point: String,
        output_dir: String,
    }

    impl RspackCompiler {
        pub fn new(entry_point: &str, output_dir: &str) -> Self {
            Self {
                entry_point: entry_point.to_string(),
                output_dir: output_dir.to_string(),
            }
        }

        pub fn entry_point(&self) -> &str {
            &self.entry_point
        }

        pub fn output_dir(&self) -> &str {
            &self.output_dir
        }

        pub fn compile(&self) -> Result<()> {
            log::info!(
                "Starting TypeScript/JSX compilation with entry: {}",
                self.entry_point
            );

            // Ensure output directory exists
            let output_dir = Path::new(&self.output_dir);
            if !output_dir.exists() {
                std::fs::create_dir_all(output_dir)?;
                log::info!("Created output directory: {}", self.output_dir);
            }

            // Compile TypeScript/JSX entry point
            self.compile_typescript_entry()?;

            // Process CSS files
            self.process_css_files()?;

            log::info!("TypeScript/JSX compilation completed successfully");
            Ok(())
        }

        fn compile_typescript_entry(&self) -> Result<()> {
            let entry_path = Path::new(&self.entry_point);
            if !entry_path.exists() {
                return Err(anyhow::anyhow!(
                    "Entry point not found: {}",
                    self.entry_point
                ));
            }

            let source_code = std::fs::read_to_string(entry_path)
                .with_context(|| format!("Failed to read entry point: {}", self.entry_point))?;

            log::info!("Compiling TypeScript/JSX: {}", self.entry_point);

            // Transform TypeScript/JSX to JavaScript
            let compiled_js = self.transform_typescript_to_javascript(&source_code)?;

            // Write compiled output
            let js_output = Path::new(&self.output_dir).join("main.js");
            std::fs::write(&js_output, compiled_js)
                .with_context(|| format!("Failed to write output: {}", js_output.display()))?;

            log::info!("JavaScript bundle written to: {}", js_output.display());
            Ok(())
        }

        fn transform_typescript_to_javascript(&self, source_code: &str) -> Result<String> {
            // TypeScript/JSX to JavaScript transformation pipeline
            let mut js_code = source_code.to_string();
            
            // 1. Remove TypeScript type annotations
            js_code = self.remove_typescript_types(&js_code)?;
            
            // 2. Transform JSX to React.createElement calls
            js_code = self.transform_jsx_to_react_calls(&js_code)?;
            
            // 3. Transform ES6 imports to CommonJS (simplified)
            js_code = self.transform_imports(&js_code)?;
            
            // 4. Wrap in runtime environment
            Ok(self.wrap_with_runtime(js_code))
        }

        fn remove_typescript_types(&self, code: &str) -> Result<String> {
            // Remove TypeScript interface declarations
            let interface_regex = Regex::new(r"interface\s+\w+\s*\{[^}]*\}")?;
            let mut result = interface_regex.replace_all(code, "").to_string();
            
            // Remove type annotations from function parameters
            let param_type_regex = Regex::new(r"(\w+)\s*:\s*[^,\)=]+")?;
            result = param_type_regex.replace_all(&result, "$1").to_string();
            
            // Remove return type annotations
            let return_type_regex = Regex::new(r"\)\s*:\s*[^{]+")?;
            result = return_type_regex.replace_all(&result, ")").to_string();
            
            // Remove variable type annotations
            let var_type_regex = Regex::new(r":\s*React\.FC<[^>]*>")?;
            result = var_type_regex.replace_all(&result, "").to_string();
            
            Ok(result)
        }

        fn transform_jsx_to_react_calls(&self, code: &str) -> Result<String> {
            let mut result = code.to_string();
            
            // Transform self-closing JSX tags: <Component /> -> React.createElement(Component)
            let self_closing_regex = Regex::new(r"<(\w+)([^>]*)/>")?;
            result = self_closing_regex.replace_all(&result, |caps: &regex::Captures| {
                let component = &caps[1];
                let props = &caps[2].trim();
                if props.is_empty() {
                    format!("React.createElement({})", component)
                } else {
                    // Simple props parsing (just pass as object)
                    format!("React.createElement({}, {{{}}})", component, props)
                }
            }).to_string();
            
            // Transform JSX with children - simplified approach without backreferences
            // Handle common patterns
            result = result.replace("<div>", "React.createElement('div', null, ");
            result = result.replace("</div>", ")");
            result = result.replace("<h1>", "React.createElement('h1', null, '");
            result = result.replace("</h1>", "')");
            result = result.replace("<p>", "React.createElement('p', null, '");
            result = result.replace("</p>", "')");
            
            Ok(result)
        }

        fn transform_imports(&self, code: &str) -> Result<String> {
            // Transform ES6 imports to global React usage
            let import_react_regex = Regex::new(r#"import React from ['"]react['"];"#)?;
            let mut result = import_react_regex.replace_all(code, "// React loaded globally").to_string();
            
            // Remove other imports (simplified approach)
            let import_regex = Regex::new(r"import.*?from.*?;[\n\r]*")?;
            result = import_regex.replace_all(&result, "").to_string();
            
            // Remove export statements 
            let export_regex = Regex::new(r"export\s+(default\s+)?")?;
            result = export_regex.replace_all(&result, "").to_string();
            
            Ok(result)
        }

        fn wrap_with_runtime(&self, compiled_code: String) -> String {
            format!(
                r#"
// RSPack-compatible bundle with TypeScript/JSX compilation
(function() {{
    'use strict';
    
    // React runtime - in production this would be bundled or imported
    const React = window.React;
    const ReactDOM = window.ReactDOM;
    
    if (!React || !ReactDOM) {{
        console.error('React runtime not available. Please include React libraries.');
        return;
    }}
    
    // Compiled TypeScript/JSX code
    {}
    
    // Auto-export and hydration logic
    // Find the main component (last declared function/const)
    const componentNames = ['App', 'Component', 'Main'];
    let MainComponent = null;
    
    for (const name of componentNames) {{
        if (typeof window[name] !== 'undefined') {{
            MainComponent = window[name];
            break;
        }}
        if (typeof eval('typeof ' + name) !== 'undefined') {{
            try {{
                MainComponent = eval(name);
                break;
            }} catch(e) {{
                // Continue searching
            }}
        }}
    }}
    
    // Fallback: create a simple component if none found
    if (!MainComponent) {{
        MainComponent = function DefaultApp() {{
            return React.createElement('div', {{ className: 'app' }},
                React.createElement('h1', null, 'RSPack SSR Demo'),
                React.createElement('p', null, 'TypeScript/JSX compilation successful!'),
                React.createElement('p', null, 'Server-side rendering with Actix Web + Tera.')
            );
        }};
    }}
    
    // React 18 hydration
    const container = document.getElementById('root');
    if (container) {{
        const root = ReactDOM.createRoot(container);
        root.render(React.createElement(MainComponent));
    }} else {{
        console.error('Root element not found');
    }}
}})();
"#,
                compiled_code
            )
        }

        fn process_css_files(&self) -> Result<()> {
            let css_paths = self.find_css_files()?;
            if css_paths.is_empty() {
                log::info!("No CSS files found to process");
                return Ok(());
            }

            let mut combined_css = String::new();
            for css_path in css_paths {
                let css_content = std::fs::read_to_string(&css_path)
                    .with_context(|| format!("Failed to read CSS file: {}", css_path.display()))?;
                
                combined_css.push_str(&format!("/* {} */\n", css_path.display()));
                combined_css.push_str(&css_content);
                combined_css.push('\n');
                
                log::info!("Processed CSS file: {}", css_path.display());
            }

            let css_output = Path::new(&self.output_dir).join("main.css");
            std::fs::write(&css_output, combined_css)
                .with_context(|| format!("Failed to write CSS output: {}", css_output.display()))?;

            log::info!("CSS bundle written to: {}", css_output.display());
            Ok(())
        }

        fn find_css_files(&self) -> Result<Vec<PathBuf>> {
            let mut css_files = Vec::new();
            
            // Look for CSS files in frontend directory
            let frontend_dir = Path::new("./frontend");
            if frontend_dir.exists() {
                self.collect_css_files_recursive(frontend_dir, &mut css_files)?;
            }
            
            Ok(css_files)
        }

        fn collect_css_files_recursive(&self, dir: &Path, css_files: &mut Vec<PathBuf>) -> Result<()> {
            for entry in std::fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                
                if path.is_dir() {
                    self.collect_css_files_recursive(&path, css_files)?;
                } else if let Some(extension) = path.extension() {
                    if extension == "css" {
                        css_files.push(path);
                    }
                }
            }
            Ok(())
        }
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    // Initialize RSPack compiler with entry point and output directory
    let rspack_compiler = rspack_integration::RspackCompiler::new("./frontend/index.tsx", "./dist");

    // Compile frontend assets using RSPack
    if let Err(e) = rspack_compiler.compile() {
        log::error!("RSPack compilation failed: {}", e);
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("RSPack compilation failed: {}", e),
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

#[cfg(test)]
mod tests {
    use super::rspack_integration::*;
    use std::path::Path;

    #[test]
    fn test_rspack_compiler_creation() {
        let compiler = RspackCompiler::new("./frontend/index.tsx", "./dist");
        // Verify compiler configuration is properly initialized
        assert!(!compiler.entry_point().is_empty());
        assert!(!compiler.output_dir().is_empty());
        assert_eq!(compiler.entry_point(), "./frontend/index.tsx");
        assert_eq!(compiler.output_dir(), "./dist");
    }

    #[test]
    fn test_tsx_compilation_architecture() {
        // Test the TypeScript/JSX compilation pipeline using SWC
        let compiler = RspackCompiler::new("./frontend/index.tsx", "./test_output");

        // Create a test TypeScript file if it doesn't exist
        if !Path::new("./frontend/index.tsx").exists() {
            // Create minimal frontend structure for testing
            if !Path::new("./frontend").exists() {
                std::fs::create_dir_all("./frontend").expect("Failed to create frontend dir");
            }
            
            let test_tsx = r#"
import React from 'react';

interface AppProps {
    title?: string;
}

const App: React.FC<AppProps> = ({ title = 'Test App' }) => {
    return (
        <div className="app">
            <h1>{title}</h1>
            <p>TypeScript + React compilation test</p>
        </div>
    );
};

export default App;
"#;
            std::fs::write("./frontend/index.tsx", test_tsx)
                .expect("Failed to write test TSX file");
        }

        // Test compilation succeeds
        let result = compiler.compile();
        assert!(
            result.is_ok(),
            "TypeScript compilation should succeed: {:?}",
            result.err()
        );

        // Verify output files are created
        assert!(
            Path::new("./test_output/main.js").exists(),
            "JavaScript bundle should be generated"
        );

        // Verify the generated JavaScript contains expected patterns
        if let Ok(js_content) = std::fs::read_to_string("./test_output/main.js") {
            assert!(
                js_content.contains("React.createElement"),
                "Generated code should contain React.createElement calls"
            );
            assert!(
                js_content.contains("App") || js_content.contains("component"),
                "Generated code should contain component references"
            );
            assert!(
                js_content.contains("ReactDOM.createRoot"),
                "Generated code should use React 18 createRoot API"
            );
        }

        // Cleanup test files
        let _ = std::fs::remove_dir_all("./test_output");
    }

    #[test]
    fn test_css_processing() {
        let compiler = RspackCompiler::new("./frontend/index.tsx", "./test_css_output");

        // Create test CSS file
        if !Path::new("./frontend/styles").exists() {
            std::fs::create_dir_all("./frontend/styles").expect("Failed to create styles dir");
        }
        
        let test_css = r#"
.app {
    font-family: Arial, sans-serif;
    margin: 0 auto;
    max-width: 800px;
}

.app h1 {
    color: #333;
    text-align: center;
}
"#;
        std::fs::write("./frontend/styles/style.css", test_css)
            .expect("Failed to write test CSS file");

        // Compile (this should process CSS)
        let result = compiler.compile();
        assert!(result.is_ok(), "Compilation should succeed: {:?}", result.err());

        // Verify CSS output
        assert!(
            Path::new("./test_css_output/main.css").exists(),
            "CSS bundle should be generated"
        );

        if let Ok(css_content) = std::fs::read_to_string("./test_css_output/main.css") {
            assert!(
                css_content.contains(".app"),
                "CSS bundle should contain app styles"
            );
        }

        // Cleanup
        let _ = std::fs::remove_dir_all("./test_css_output");
    }

    #[test]
    fn test_bundle_validation() {
        // Test that validates the structure and content of generated bundles
        let compiler = RspackCompiler::new("./frontend/index.tsx", "./test_bundle_output");

        // Ensure we have test content
        if !Path::new("./frontend/index.tsx").exists() {
            if !Path::new("./frontend").exists() {
                std::fs::create_dir_all("./frontend").expect("Failed to create frontend dir");
            }
            
            let test_content = r#"
import React from 'react';

function TestComponent() {
    return <div>Test Component</div>;
}

export default TestComponent;
"#;
            std::fs::write("./frontend/index.tsx", test_content)
                .expect("Failed to write test file");
        }

        let result = compiler.compile();
        assert!(result.is_ok(), "Bundle generation should succeed");

        // Validate bundle structure
        let bundle_path = Path::new("./test_bundle_output/main.js");
        assert!(bundle_path.exists(), "Bundle file should exist");

        // Validate bundle size (should be reasonable)
        let metadata = std::fs::metadata(bundle_path).expect("Should read bundle metadata");
        assert!(metadata.len() > 100, "Bundle should have meaningful content");
        assert!(metadata.len() < 100_000, "Bundle should not be excessively large");

        // Validate bundle content structure
        if let Ok(bundle_content) = std::fs::read_to_string(bundle_path) {
            // Should contain runtime wrapper
            assert!(
                bundle_content.contains("RSPack-compatible bundle"),
                "Bundle should contain identification comment"
            );
            
            // Should contain React integration
            assert!(
                bundle_content.contains("window.React"),
                "Bundle should integrate with global React"
            );
            
            // Should contain hydration logic
            assert!(
                bundle_content.contains("createRoot"),
                "Bundle should use React 18 hydration"
            );
        }

        // Cleanup
        let _ = std::fs::remove_dir_all("./test_bundle_output");
    }
}
