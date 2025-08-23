//--------------------------------------------------------------------------------- Location
// src/api/routes/zone_command_action.rs

//--------------------------------------------------------------------------------- Description
// This is route for zone_command_action

//--------------------------------------------------------------------------------- Import
use axum::routing::{get, post, put, delete};
use axum::Router;
use crate::AppState;
use crate::api::handlers::zone_command_action::{list_zone_command_actions, get_zone_command_action, create_zone_command_action, update_zone_command_action, delete_zone_command_action, disable_zone_command_action, enable_zone_command_action, status_zone_command_action};

//--------------------------------------------------------------------------------- Router
pub fn router() -> Router<AppState> 
{
    Router::new()
        .route("/items", get(list_zone_command_actions))
        .route("/item/{id}", get(get_zone_command_action))
        .route("/enable/{id}", get(enable_zone_command_action))
        .route("/disable/{id}", get(disable_zone_command_action))
        .route("/status/{id}", get(status_zone_command_action))
        .route("/update/{id}", put(update_zone_command_action))
        .route("/add", post(create_zone_command_action))
        .route("/delete/{id}", delete(delete_zone_command_action))       
}