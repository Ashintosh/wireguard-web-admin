use axum::{ Router, routing::get };
use std::{ error::Error, net::SocketAddr };

mod handlers;
mod templates;
mod utils;
mod models;
mod db;

use handlers::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let db_path = std::env::var("DATABASE_PATH").unwrap_or_else(|_| "meta.db".to_string());
    let _conn = db::init_db(&db_path)?;
    println!("Database initialized at: {}", db_path);
    
    let app = Router::new()
        .route("/", get(index))
        .route("/login", get(login))
        .route("/dashboard", get(dashboard))
        .route("/client/create", get(create_client))
        .route("/client/edit", get(edit_client))
        .route("/styles.css", get(styles_css));

    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    println!("Listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}