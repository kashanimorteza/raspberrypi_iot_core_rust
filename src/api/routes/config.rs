//--------------------------------------------------------------------------------- Location
// src/api/routes/config.rs

//--------------------------------------------------------------------------------- Description
// This is route for config

//--------------------------------------------------------------------------------- Import
use axum::routing::{get, post, put, delete};
use axum::Router;
use crate::AppState;
use crate::api::handlers::config::{list_configs, get_config, create_config, update_config, delete_config, disable_config, enable_config};



//--------------------------------------------------------------------------------- Router
pub fn router() -> Router<AppState> 
{
    Router::new()
        .route("/items", get(list_configs))
        .route("/item/{id}", get(get_config))
        .route("/update/{id}", put(update_config))
        .route("/add", post(create_config))
        .route("/delete/{id}", delete(delete_config))
        .route("/disable/{id}", get(disable_config))
        .route("/enable/{id}", get(enable_config))       
}
