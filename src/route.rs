//--------------------------------------------------------------------------------- Location
// src/route.rs

//--------------------------------------------------------------------------------- Description
// Application router configuration with all routes and middleware setup

//--------------------------------------------------------------------------------- Import
use axum::{middleware::from_fn, Router};
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::{api, doc, AppState};

//--------------------------------------------------------------------------------- Router Configuration
pub fn create_app(state: AppState) -> Router {
    // Middleware stack
    let middleware_stack = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .layer(from_fn(api::middleware::logging_middleware));

    // Routes configuration
    Router::new()
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
        .merge(SwaggerUi::new("/doc").url("/api-doc/openapi.json", doc::ApiDoc::openapi()))
        .layer(middleware_stack)
        .with_state(state)
}
