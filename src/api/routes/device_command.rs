//--------------------------------------------------------------------------------- Location
// src/api/routes/device_command.rs

//--------------------------------------------------------------------------------- Description
// This is route for device_command

//--------------------------------------------------------------------------------- Import
use axum::routing::{get, post, put, delete};
use axum::Router;
use crate::AppState;
use crate::api::handlers::device_command::{list_device_commands, get_device_command, create_device_command, update_device_command, delete_device_command, disable_device_command, enable_device_command, status_device_command};

//--------------------------------------------------------------------------------- Router
pub fn router() -> Router<AppState> 
{
    Router::new()
        .route("/items", get(list_device_commands))

        .route("/item/{id}", get(get_device_command))
        .route("/enable/{id}", get(enable_device_command))
        .route("/disable/{id}", get(disable_device_command))
        .route("/status/{id}", get(status_device_command))
        .route("/update/{id}", put(update_device_command))
        .route("/add", post(create_device_command))
        .route("/delete/{id}", delete(delete_device_command))       
}