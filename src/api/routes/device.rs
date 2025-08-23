//--------------------------------------------------------------------------------- Location
// src/api/routes/device.rs

//--------------------------------------------------------------------------------- Description
// This is route for device

//--------------------------------------------------------------------------------- Import
use axum::routing::{get, post, put, delete};
use axum::Router;
use crate::AppState;
use crate::api::handlers::device::{list_devices, get_device, create_device, update_device, delete_device, disable_device, enable_device};

//--------------------------------------------------------------------------------- Router
pub fn router() -> Router<AppState> 
{
    Router::new()
        .route("/items", get(list_devices))
        .route("/item/{id}", get(get_device))
        .route("/enable/{id}", get(enable_device))
        .route("/disable/{id}", get(disable_device))
        .route("/update/{id}", put(update_device))
        .route("/add", post(create_device))
        .route("/delete/{id}", delete(delete_device))       
}