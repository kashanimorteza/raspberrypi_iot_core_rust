//--------------------------------------------------------------------------------- Location
// src/main.rs

//--------------------------------------------------------------------------------- Description
// Main Axum server with proper structure: Routes | Handlers | Extractors | Middlewares | State Management

//--------------------------------------------------------------------------------- Import
pub use dotenvy::dotenv;
pub use sea_orm::{Database, DatabaseConnection};
pub use std::net::SocketAddr;
pub use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
pub mod api;
pub mod orm;
pub mod logics;
pub mod args;
mod doc;
mod route;

//--------------------------------------------------------------------------------- State Management
#[derive(Clone)]
pub struct AppState 
{
    pub db: DatabaseConnection,
}

//--------------------------------------------------------------------------------- Main
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>
{
    // Load environment variables
    dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "api_1=debug,tower_http=debug".into()),)
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Database connection
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite::memory:".to_string());
    let db: DatabaseConnection = Database::connect(&database_url).await.expect("Failed to connect to database");

    // Handle command-line arguments
    if args::handle_arguments(&db).await? 
    {
        return Ok(()); // Exit if arguments were handled (like --add-users)
    }

    // State management
    let state = AppState { db };

    // Create application with routes and middleware
    let app = route::create_app(state);

    // Server configuration
    let api_host = std::env::var("API_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let api_port = std::env::var("API_PORT").unwrap_or_else(|_| "3000".to_string());
    let addr: SocketAddr = format!("{}:{}", api_host, api_port).parse().expect("invalid host:port");
    tracing::info!("🚀 Server listening on {}", addr);
    
    // Server Start
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

