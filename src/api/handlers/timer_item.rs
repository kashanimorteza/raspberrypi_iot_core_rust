//--------------------------------------------------------------------------------- Location
// src/api/handlers/timer_item.rs

//--------------------------------------------------------------------------------- Description
// Axum handlers for TimerItem CRUD operations

//--------------------------------------------------------------------------------- Import
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use std::collections::HashMap;
use utoipa::ToSchema;
use crate::{orm::models::timer_item::Model as TimerItemModel, logics::general::ModelOutput, AppState};
use crate::api::services::timer_item::TimerItemService;

//--------------------------------------------------------------------------------- Request DTOs
#[derive(Deserialize, ToSchema)]
#[schema(description = "Request payload for creating a new timer item")]
pub struct CreateTimerItemRequest {
    #[schema(example = 1)]
    pub timer_id: i32,
    #[schema(example = "Morning Schedule")]
    pub name: String,
    #[schema(example = "08:00")]
    pub value_from: String,
    #[schema(example = "18:00")]
    pub value_to: String,
    #[schema(example = "Daily morning to evening schedule")]
    pub description: String,
    #[schema(example = true)]
    pub enable: bool,
}

#[derive(Deserialize, ToSchema)]
#[schema(description = "Request payload for updating an existing timer item")]
pub struct UpdateTimerItemRequest {
    #[schema(example = 1)]
    pub timer_id: Option<i32>,
    #[schema(example = "Morning Schedule")]
    pub name: Option<String>,
    #[schema(example = "08:00")]
    pub value_from: Option<String>,
    #[schema(example = "18:00")]
    pub value_to: Option<String>,
    #[schema(example = "Daily morning to evening schedule")]
    pub description: Option<String>,
    #[schema(example = true)]
    pub enable: Option<bool>,
}

//--------------------------------------------------------------------------------- Handlers
#[utoipa::path(
    get,
    path = "/timer_item/items",
    tag = "📝 Timer Item",

    params(
        ("limit" = Option<i32>, Query, description = "Maximum number of timer items to return"),
        ("offset" = Option<i32>, Query, description = "Number of timer items to skip"),
    ),
    responses(
        (status = 200, description = "List of timer items retrieved successfully", body = Vec<TimerItemModel>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn list_timer_items(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<ModelOutput<Vec<TimerItemModel>>>, StatusCode> {
    let service = TimerItemService::new();
    let result = service.items(&state.db, params).await;
    Ok(Json(result))
}

#[utoipa::path(
    get,
    path = "/timer_item/item/{id}",
    tag = "📝 Timer Item",

    params(
        ("id" = i32, Path, description = "Timer item ID")
    ),
    responses(
        (status = 200, description = "Timer item retrieved successfully", body = TimerItemModel),
        (status = 404, description = "Timer item not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_timer_item(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<TimerItemModel>>, StatusCode> {
    let service = TimerItemService::new();
    let result = service.item(&state.db, id).await;
    Ok(Json(result))
}

#[utoipa::path(
    get,
    path = "/timer_item/enable/{id}",
    tag = "📝 Timer Item",

    params(
        ("id" = i32, Path, description = "Timer item ID to enable")
    ),
    responses(
        (status = 200, description = "Timer item enabled successfully", body = TimerItemModel),
        (status = 404, description = "Timer item not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn enable_timer_item(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<TimerItemModel>>, StatusCode> {
    let service = TimerItemService::new();
    let result = service.enable(&state.db, id).await;
    Ok(Json(result))
}

#[utoipa::path(
    get,
    path = "/timer_item/disable/{id}",
    tag = "📝 Timer Item",

    params(
        ("id" = i32, Path, description = "Timer item ID to disable")
    ),
    responses(
        (status = 200, description = "Timer item disabled successfully", body = TimerItemModel),
        (status = 404, description = "Timer item not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn disable_timer_item(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<TimerItemModel>>, StatusCode> {
    let service = TimerItemService::new();
    let result = service.disable(&state.db, id).await;
    Ok(Json(result))
}

#[utoipa::path(
    put,
    path = "/timer_item/update/{id}",
    tag = "📝 Timer Item",

    params(
        ("id" = i32, Path, description = "Timer item ID to update")
    ),
    request_body = UpdateTimerItemRequest,
    responses(
        (status = 200, description = "Timer item updated successfully", body = TimerItemModel),
        (status = 404, description = "Timer item not found"),
        (status = 400, description = "Invalid request data"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_timer_item(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateTimerItemRequest>,
) -> Result<Json<ModelOutput<TimerItemModel>>, StatusCode> {
    let service = TimerItemService::new();
    
    let timer_item_model = TimerItemModel {
        id,
        timer_id: payload.timer_id.unwrap_or_default(),
        name: payload.name.unwrap_or_default(),
        value_from: payload.value_from.unwrap_or_default(),
        value_to: payload.value_to.unwrap_or_default(),
        description: payload.description.unwrap_or_default(),
        enable: payload.enable.unwrap_or(true),
    };
    
    let result = service.update(&state.db, timer_item_model).await;
    Ok(Json(result))
}

#[utoipa::path(
    post,
    path = "/timer_item/add",
    tag = "📝 Timer Item",

    request_body = CreateTimerItemRequest,
    responses(
        (status = 201, description = "Timer item created successfully", body = TimerItemModel),
        (status = 400, description = "Invalid request data"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_timer_item(
    State(state): State<AppState>,
    Json(payload): Json<CreateTimerItemRequest>,
) -> Result<Json<ModelOutput<TimerItemModel>>, StatusCode> {
    let service = TimerItemService::new();
    let timer_item_model = TimerItemModel {
        id: 0, // Will be auto-generated
        timer_id: payload.timer_id,
        name: payload.name,
        value_from: payload.value_from,
        value_to: payload.value_to,
        description: payload.description,
        enable: payload.enable,
    };
    
    let result = service.add(&state.db, timer_item_model).await;
    Ok(Json(result))
}

#[utoipa::path(
    delete,
    path = "/timer_item/delete/{id}",
    tag = "📝 Timer Item",

    params(
        ("id" = i32, Path, description = "Timer item ID to delete")
    ),
    responses(
        (status = 200, description = "Timer item deleted successfully"),
        (status = 404, description = "Timer item not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_timer_item(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<String>>, StatusCode> {
    let service = TimerItemService::new();
    let result = service.delete(&state.db, id).await;
    Ok(Json(result))
}
