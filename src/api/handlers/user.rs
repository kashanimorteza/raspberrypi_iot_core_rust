//--------------------------------------------------------------------------------- Location
// src/handlers/user.rs

//--------------------------------------------------------------------------------- Description
// Axum handlers for User CRUD operations

//--------------------------------------------------------------------------------- Import
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use std::collections::HashMap;
use utoipa::ToSchema;
use crate::{orm::models::user::Model as UserModel, logics::general::ModelOutput, AppState};
use crate::api::services::user::UserService;

//--------------------------------------------------------------------------------- Request DTOs
#[derive(Deserialize, ToSchema)]
#[schema(description = "Request payload for creating a new user")]
pub struct CreateUserRequest {
    #[schema(example = "John Doe")]
    pub name: String,
    #[schema(example = "johndoe")]
    pub username: String,
    #[schema(example = "secure_password")]
    pub password: String,
    #[schema(example = "api_key_123")]
    pub key: String,
    #[schema(example = "john@example.com")]
    pub email: String,
    #[schema(example = "+1234567890")]
    pub phone: String,
    #[schema(example = "telegram_123")]
    pub tg_id: String,
    #[schema(example = true)]
    pub enable: bool,
}

#[derive(Deserialize, ToSchema)]
#[schema(description = "Request payload for updating an existing user")]
pub struct UpdateUserRequest {
    #[schema(example = "John Doe")]
    pub name: Option<String>,
    #[schema(example = "johndoe")]
    pub username: Option<String>,
    #[schema(example = "secure_password")]
    pub password: Option<String>,
    #[schema(example = "api_key_123")]
    pub key: Option<String>,
    #[schema(example = "john@example.com")]
    pub email: Option<String>,
    #[schema(example = "+1234567890")]
    pub phone: Option<String>,
    #[schema(example = "telegram_123")]
    pub tg_id: Option<String>,
    #[schema(example = true)]
    pub enable: Option<bool>,
}

//--------------------------------------------------------------------------------- Handlers
//------------------------- Items
#[utoipa::path(
    get,
    path = "/user/items",
    tag = "👥 User",
    params(
        ("limit" = Option<i32>, Query, description = "Maximum number of users to return"),
        ("offset" = Option<i32>, Query, description = "Number of users to skip"),
    ),
    responses(
        (status = 200, description = "List of users retrieved successfully", body = Vec<UserModel>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn list_users(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<ModelOutput<Vec<UserModel>>>, StatusCode> {
    let service = UserService::new();
    let result = service.items(&state.db, params).await;
    Ok(Json(result))
}

//------------------------- Item
#[utoipa::path(
    get,
    path = "/user/item/{id}",
    tag = "👥 User",
    params(
        ("id" = i32, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User retrieved successfully", body = UserModel),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<UserModel>>, StatusCode> {
    let service = UserService::new();
    let result = service.item(&state.db, id).await;
    Ok(Json(result))
}

//------------------------- Update
#[utoipa::path(
    put,
    path = "/user/update/{id}",
    tag = "👥 User",
    params(
        ("id" = i32, Path, description = "User ID to update")
    ),
    request_body = UpdateUserRequest,
    responses(
        (status = 200, description = "User updated successfully", body = UserModel),
        (status = 404, description = "User not found"),
        (status = 400, description = "Invalid request data"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<ModelOutput<UserModel>>, StatusCode> {
    let service = UserService::new();
    
    let user_model = UserModel {
        id,
        name: payload.name.unwrap_or_default(),
        username: payload.username.unwrap_or_default(),
        password: payload.password.unwrap_or_default(),
        key: payload.key.unwrap_or_default(),
        email: payload.email.unwrap_or_default(),
        phone: payload.phone.unwrap_or_default(),
        tg_id: payload.tg_id.unwrap_or_default(),
        enable: payload.enable.unwrap_or(true),
    };
    
    let result = service.update(&state.db, user_model).await;
    Ok(Json(result))
}

//------------------------- Add
#[utoipa::path(
    post,
    path = "/user/add",
    tag = "👥 User",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created successfully", body = UserModel),
        (status = 400, description = "Invalid request data"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<ModelOutput<UserModel>>, StatusCode> {
    let service = UserService::new();
    let user_model = UserModel {
        id: 0, // Will be auto-generated
        name: payload.name,
        username: payload.username,
        password: payload.password,
        key: payload.key,
        email: payload.email,
        phone: payload.phone,
        tg_id: payload.tg_id,
        enable: payload.enable,
    };
    
    let result = service.add(&state.db, user_model).await;
    Ok(Json(result))
}

//------------------------- Delete
#[utoipa::path(
    delete,
    path = "/user/delete/{id}",
    tag = "👥 User",
    params(
        ("id" = i32, Path, description = "User ID to delete")
    ),
    responses(
        (status = 200, description = "User deleted successfully"),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<String>>, StatusCode> {
    let service = UserService::new();
    let result = service.delete(&state.db, id).await;
    Ok(Json(result))
}

//------------------------- Disable
#[utoipa::path(
    get,
    path = "/user/disable/{id}",
    tag = "👤 User",

    params(
        ("id" = i32, Path, description = "User ID to disable")
    ),
    responses(
        (status = 200, description = "User disabled successfully", body = UserModel),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn disable_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<UserModel>>, StatusCode> {
    let service = UserService::new();
    let result = service.disable(&state.db, id).await;
    Ok(Json(result))
}

//------------------------- Enable
#[utoipa::path(
    get,
    path = "/user/enable/{id}",
    tag = "👤 User",

    params(
        ("id" = i32, Path, description = "User ID to enable")
    ),
    responses(
        (status = 200, description = "User enabled successfully", body = UserModel),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn enable_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<UserModel>>, StatusCode> {
    let service = UserService::new();
    let result = service.enable(&state.db, id).await;
    Ok(Json(result))
}
