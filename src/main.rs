use actix_web::{web, App, HttpServer, HttpResponse, Result};
use actix_files::Files;
use tera::{Tera, Context};
use std::sync::Arc;

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

// TODO: Implement rspack compilation using rspack Rust crates
// This should replace the npm-based build process
fn run_rspack_build() -> anyhow::Result<()> {
    log::info!("Starting RSPack build process using Rust crates...");
    
    // Placeholder for rspack Rust API integration
    // This needs to be implemented using rspack_core and related crates
    log::warn!("RSPack Rust API integration not yet implemented");
    log::warn!("Falling back to existing static files...");
    
    Ok(())
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    // Run the build process using rspack Rust crates
    if let Err(e) = run_rspack_build() {
        log::error!("RSPack build process failed: {}", e);
        log::warn!("Continuing with existing static files...");
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
