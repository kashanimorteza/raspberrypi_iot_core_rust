//--------------------------------------------------------------------------------- Location
// src/api/routes/timer.rs

//--------------------------------------------------------------------------------- Description
// This is route for timer

//--------------------------------------------------------------------------------- Import
use axum::routing::{get, post, put, delete};
use axum::Router;
use crate::AppState;
use crate::api::handlers::timer::{list_timers, get_timer, create_timer, update_timer, delete_timer};



//--------------------------------------------------------------------------------- Router
pub fn router() -> Router<AppState> 
{
    Router::new()
        .route("/items", get(list_timers))
        .route("/item/{id}", get(get_timer))
        .route("/update/{id}", put(update_timer))
        .route("/add", post(create_timer))
        .route("/delete/{id}", delete(delete_timer))       
}