//--------------------------------------------------------------------------------- Location
// src/api/routes/zone.rs

//--------------------------------------------------------------------------------- Description
// This is route for zone

//--------------------------------------------------------------------------------- Import
use axum::routing::{get, post, put, delete};
use axum::Router;
use crate::AppState;
use crate::api::handlers::zone::{list_zones, get_zone, create_zone, update_zone, delete_zone, disable_zone, enable_zone};

//--------------------------------------------------------------------------------- Router
pub fn router() -> Router<AppState> 
{
    Router::new()
        .route("/items", get(list_zones))
        .route("/item/{id}", get(get_zone))
        .route("/update/{id}", put(update_zone))
        .route("/add", post(create_zone))
        .route("/delete/{id}", delete(delete_zone))
        .route("/disable/{id}", get(disable_zone))
        .route("/enable/{id}", get(enable_zone))       
}