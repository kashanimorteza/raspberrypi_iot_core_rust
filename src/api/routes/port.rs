//--------------------------------------------------------------------------------- Location
// src/api/routes/port.rs

//--------------------------------------------------------------------------------- Description
// This is route for port

//--------------------------------------------------------------------------------- Import
use axum::routing::{get, post, put, delete};
use axum::Router;
use crate::AppState;
use crate::api::handlers::port::{list_ports, get_port, create_port, update_port, delete_port, disable_port, enable_port};

//--------------------------------------------------------------------------------- Router
pub fn router() -> Router<AppState> 
{
    Router::new()
        .route("/items", get(list_ports))
        .route("/item/{id}", get(get_port))
        .route("/enable/{id}", get(enable_port))
        .route("/disable/{id}", get(disable_port))
        .route("/update/{id}", put(update_port))
        .route("/add", post(create_port))
        .route("/delete/{id}", delete(delete_port))       
}