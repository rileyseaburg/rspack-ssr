use actix_web::{web, App, HttpServer, HttpResponse, Result};
use actix_files::Files;
use tera::{Tera, Context};
use std::sync::Arc;
use std::process::Command;

struct AppState {
    tera: Arc<Tera>,
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

fn run_build() -> anyhow::Result<()> {
    log::info!("Starting RSPack build process...");
    
    // Check if node_modules exists, if not install dependencies
    if !std::path::Path::new("node_modules").exists() {
        log::info!("Installing Node.js dependencies...");
        let output = Command::new("npm")
            .arg("install")
            .output()?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            log::error!("npm install failed: {}", stderr);
            return Err(anyhow::anyhow!("npm install failed: {}", stderr));
        }
        log::info!("Dependencies installed successfully");
    }
    
    // Run rspack build
    log::info!("Running RSPack build...");
    let output = Command::new("npm")
        .arg("run")
        .arg("build")
        .output()?;
    
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        log::info!("RSPack build completed successfully: {}", stdout);
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        log::error!("RSPack build failed: {}", stderr);
        Err(anyhow::anyhow!("RSPack build failed: {}", stderr))
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    // Run the build process
    if let Err(e) = run_build() {
        log::error!("Build process failed: {}", e);
        // Continue anyway with placeholder files
        log::warn!("Continuing with placeholder files...");
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
