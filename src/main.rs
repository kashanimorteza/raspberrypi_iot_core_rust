//--------------------------------------------------------------------------------- Location
// src/main.rs

//--------------------------------------------------------------------------------- Description
// Main Axum server with proper structure: Routes | Handlers | Extractors | Middlewares | State Management

//--------------------------------------------------------------------------------- Import
pub use axum::{middleware::from_fn, Router};
pub use dotenvy::dotenv;
pub use sea_orm::{Database, DatabaseConnection};
pub use std::net::SocketAddr;
pub use tower::ServiceBuilder;
pub use tower_http::{cors::CorsLayer, trace::TraceLayer};
pub use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
pub mod api;
pub mod orm;
pub mod logics;
pub mod args;
mod doc;


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

    // Middleware stack
    let middleware_stack = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .layer(from_fn(api::middleware::logging_middleware));

    // Routes configuration
    let app = Router::new()
        .nest("/config", api::routes::config::router())
        .nest("/user", api::routes::user::router())
        .nest("/device", api::routes::device::router())
        .nest("/zone", api::routes::zone::router())
        .nest("/device_command", api::routes::device_command::router())
        .nest("/log", api::routes::log::router())
        .nest("/port", api::routes::port::router())
        .nest("/timer", api::routes::timer::router())
        .nest("/timer_device", api::routes::timer_device::router())
        .nest("/timer_item", api::routes::timer_item::router())
        .nest("/timer_limit", api::routes::timer_limit::router())
        .nest("/zone_command", api::routes::zone_command::router())
        .nest("/zone_command_action", api::routes::zone_command_action::router())
        .nest("/zone_command_if", api::routes::zone_command_if::router())
        .merge(SwaggerUi::new("/doc").url("/api-doc/openapi.json", doc::ApiDoc::openapi()),)
        .layer(middleware_stack)
        .with_state(state);

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

