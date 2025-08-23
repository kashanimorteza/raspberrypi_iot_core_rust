//--------------------------------------------------------------------------------- Location
// src/api/routes/zone_command_if.rs

//--------------------------------------------------------------------------------- Description
// This is route for zone_command_if

//--------------------------------------------------------------------------------- Import
use axum::routing::{get, post, put, delete};
use axum::Router;
use crate::AppState;
use crate::api::handlers::zone_command_if::{list_zone_command_ifs, get_zone_command_if, create_zone_command_if, update_zone_command_if, delete_zone_command_if, disable_zone_command_if, enable_zone_command_if, status_zone_command_if};

//--------------------------------------------------------------------------------- Router
pub fn router() -> Router<AppState> 
{
    Router::new()
        .route("/items", get(list_zone_command_ifs))
        .route("/item/{id}", get(get_zone_command_if))
        .route("/enable/{id}", get(enable_zone_command_if))
        .route("/disable/{id}", get(disable_zone_command_if))
        .route("/status/{id}", get(status_zone_command_if))
        .route("/update/{id}", put(update_zone_command_if))
        .route("/add", post(create_zone_command_if))
        .route("/delete/{id}", delete(delete_zone_command_if))       
}