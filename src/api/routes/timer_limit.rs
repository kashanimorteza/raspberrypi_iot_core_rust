//--------------------------------------------------------------------------------- Location
// src/api/routes/timer_limit.rs

//--------------------------------------------------------------------------------- Description
// This is route for timer_limit

//--------------------------------------------------------------------------------- Import
use axum::routing::{get, post, put, delete};
use axum::Router;
use crate::AppState;
use crate::api::handlers::timer_limit::{list_timer_limits, get_timer_limit, create_timer_limit, update_timer_limit, delete_timer_limit, disable_timer_limit, enable_timer_limit};



//--------------------------------------------------------------------------------- Router
pub fn router() -> Router<AppState> 
{
    Router::new()
        .route("/items", get(list_timer_limits))
        .route("/item/{id}", get(get_timer_limit))
        .route("/update/{id}", put(update_timer_limit))
        .route("/add", post(create_timer_limit))
        .route("/delete/{id}", delete(delete_timer_limit))
        .route("/disable/{id}", get(disable_timer_limit))
        .route("/enable/{id}", get(enable_timer_limit))       
}