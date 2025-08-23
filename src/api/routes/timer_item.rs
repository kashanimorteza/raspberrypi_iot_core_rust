//--------------------------------------------------------------------------------- Location
// src/api/routes/timer_item.rs

//--------------------------------------------------------------------------------- Description
// This is route for timer_item

//--------------------------------------------------------------------------------- Import
use axum::routing::{get, post, put, delete};
use axum::Router;
use crate::AppState;
use crate::api::handlers::timer_item::{list_timer_items, get_timer_item, create_timer_item, update_timer_item, delete_timer_item, disable_timer_item, enable_timer_item, status_timer_item};

//--------------------------------------------------------------------------------- Router
pub fn router() -> Router<AppState> 
{
    Router::new()
        .route("/items", get(list_timer_items))
        .route("/item/{id}", get(get_timer_item))
        .route("/enable/{id}", get(enable_timer_item))
        .route("/disable/{id}", get(disable_timer_item))
        .route("/status/{id}", get(status_timer_item))
        .route("/update/{id}", put(update_timer_item))
        .route("/add", post(create_timer_item))
        .route("/delete/{id}", delete(delete_timer_item))       
}