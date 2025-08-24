//--------------------------------------------------------------------------------- Location
// src/api/routes/timer_device.rs

//--------------------------------------------------------------------------------- Description
// This is route for timer_device

//--------------------------------------------------------------------------------- Import
use axum::routing::{get, post, put, delete};
use axum::Router;
use crate::AppState;
use crate::api::handlers::timer_device::{list_timer_devices, get_timer_device, create_timer_device, update_timer_device, delete_timer_device, disable_timer_device, enable_timer_device, status_timer_device, import_timer_device};

//--------------------------------------------------------------------------------- Router
pub fn router() -> Router<AppState> 
{
    Router::new()
        .route("/items", get(list_timer_devices))
        .route("/import", get(import_timer_device))
        .route("/item/{id}", get(get_timer_device))
        .route("/enable/{id}", get(enable_timer_device))
        .route("/disable/{id}", get(disable_timer_device))
        .route("/status/{id}", get(status_timer_device))
        .route("/update/{id}", put(update_timer_device))
        .route("/add", post(create_timer_device))
        .route("/delete/{id}", delete(delete_timer_device))       
}