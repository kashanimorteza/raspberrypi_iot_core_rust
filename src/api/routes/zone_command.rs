//--------------------------------------------------------------------------------- Location
// src/api/routes/zone_command.rs

//--------------------------------------------------------------------------------- Description
// This is route for zone_command

//--------------------------------------------------------------------------------- Import
use axum::routing::{get, post, put, delete};
use axum::Router;
use crate::AppState;
use crate::api::handlers::zone_command::{list_zone_commands, get_zone_command, create_zone_command, update_zone_command, delete_zone_command, disable_zone_command, enable_zone_command};



//--------------------------------------------------------------------------------- Router
pub fn router() -> Router<AppState> 
{
    Router::new()
        .route("/items", get(list_zone_commands))
        .route("/item/{id}", get(get_zone_command))
        .route("/update/{id}", put(update_zone_command))
        .route("/add", post(create_zone_command))
        .route("/delete/{id}", delete(delete_zone_command))
        .route("/disable/{id}", get(disable_zone_command))
        .route("/enable/{id}", get(enable_zone_command))       
}