//--------------------------------------------------------------------------------- Location
// src/api/routes/log.rs

//--------------------------------------------------------------------------------- Description
// This is route for log

//--------------------------------------------------------------------------------- Import
use axum::routing::{get, post, put, delete};
use axum::Router;
use crate::AppState;
use crate::api::handlers::log::{list_logs, get_log, create_log, update_log, delete_log, disable_log, enable_log, status_log};

//--------------------------------------------------------------------------------- Router
pub fn router() -> Router<AppState> 
{
    Router::new()
        .route("/items", get(list_logs))
        .route("/item/{id}", get(get_log))
        .route("/enable/{id}", get(enable_log))
        .route("/disable/{id}", get(disable_log))
        .route("/update/{id}", put(update_log))
        .route("/add", post(create_log))
        .route("/delete/{id}", delete(delete_log))
        .route("/status/{id}", get(status_log))       
}