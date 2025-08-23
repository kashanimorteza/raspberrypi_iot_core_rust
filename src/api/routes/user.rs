//--------------------------------------------------------------------------------- Location
// src/api/routes/user.rs

//--------------------------------------------------------------------------------- Description
// This is route for user

//--------------------------------------------------------------------------------- Import
use axum::routing::{get, post, put, delete};
use axum::Router;
use crate::AppState;
use crate::api::handlers::user::{list_users, get_user, create_user, update_user, delete_user, disable_user, enable_user};

//--------------------------------------------------------------------------------- Router
pub fn router() -> Router<AppState> 
{
    Router::new()
        .route("/items", get(list_users))
        .route("/item/{id}", get(get_user))
        .route("/enable/{id}", get(enable_user))
        .route("/disable/{id}", get(disable_user))
        .route("/update/{id}", put(update_user))
        .route("/add", post(create_user))
        .route("/delete/{id}", delete(delete_user))       
}