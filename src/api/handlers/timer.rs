//--------------------------------------------------------------------------------- Location
// src/api/handlers/timer.rs

//--------------------------------------------------------------------------------- Description
// Axum handlers for Timer CRUD operations

//--------------------------------------------------------------------------------- Import
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use std::collections::HashMap;
use utoipa::ToSchema;
use crate::{orm::models::timer::Model as TimerModel, logics::general::ModelOutput, AppState};
use crate::api::services::timer::TimerService;

//--------------------------------------------------------------------------------- Request DTOs
#[derive(Deserialize, ToSchema)]
#[schema(description = "Request payload for creating a new timer")]
pub struct CreateTimerRequest {
    pub user_id: i32,
    pub name: String,
    pub description: String,
    pub enable: bool,
}

#[derive(Deserialize, ToSchema)]
#[schema(description = "Request payload for updating an existing timer")]
pub struct UpdateTimerRequest {
    pub user_id: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub enable: Option<bool>,
}

//--------------------------------------------------------------------------------- Handlers
#[utoipa::path(
    get,
    path = "/timers/items",
    tag = "⏰ Timers",
    summary = "List all timers",
    description = "Retrieve a list of all timers with optional query parameters for filtering",
    params(
        ("limit" = Option<i32>, Query, description = "Maximum number of timers to return"),
        ("offset" = Option<i32>, Query, description = "Number of timers to skip"),
    ),
    responses(
        (status = 200, description = "List of timers retrieved successfully", body = Vec<TimerModel>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn list_timers(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<ModelOutput<Vec<TimerModel>>>, StatusCode> {
    let service = TimerService::new();
    let result = service.items(&state.db, params).await;
    Ok(Json(result))
}

#[utoipa::path(
    get,
    path = "/timers/{id}",
    tag = "⏰ Timers",
    summary = "Get timer by ID",
    description = "Retrieve a specific timer by its unique identifier",
    params(
        ("id" = i32, Path, description = "Timer ID")
    ),
    responses(
        (status = 200, description = "Timer retrieved successfully", body = TimerModel),
        (status = 404, description = "Timer not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_timer(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<TimerModel>>, StatusCode> {
    let service = TimerService::new();
    let result = service.item(&state.db, id).await;
    Ok(Json(result))
}

#[utoipa::path(
    post,
    path = "/timers/add",
    tag = "⏰ Timers",
    summary = "Create new timer",
    description = "Create a new timer with the provided information",
    request_body = CreateTimerRequest,
    responses(
        (status = 201, description = "Timer created successfully", body = TimerModel),
        (status = 400, description = "Invalid request data"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_timer(
    State(state): State<AppState>,
    Json(payload): Json<CreateTimerRequest>,
) -> Result<Json<ModelOutput<TimerModel>>, StatusCode> {
    let service = TimerService::new();
    let timer_model = TimerModel {
        id: 0, // Will be auto-generated
        user_id: payload.user_id,
        name: payload.name,
        description: payload.description,
        enable: payload.enable,
    };
    
    let result = service.add(&state.db, timer_model).await;
    Ok(Json(result))
}

#[utoipa::path(
    put,
    path = "/timers/update/{id}",
    tag = "⏰ Timers",
    summary = "Update timer",
    description = "Update an existing timer with new information",
    params(
        ("id" = i32, Path, description = "Timer ID to update")
    ),
    request_body = UpdateTimerRequest,
    responses(
        (status = 200, description = "Timer updated successfully", body = TimerModel),
        (status = 404, description = "Timer not found"),
        (status = 400, description = "Invalid request data"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_timer(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateTimerRequest>,
) -> Result<Json<ModelOutput<TimerModel>>, StatusCode> {
    let service = TimerService::new();
    
    let timer_model = TimerModel {
        id,
        user_id: payload.user_id.unwrap_or_default(),
        name: payload.name.unwrap_or_default(),
        description: payload.description.unwrap_or_default(),
        enable: payload.enable.unwrap_or(true),
    };
    
    let result = service.update(&state.db, timer_model).await;
    Ok(Json(result))
}

#[utoipa::path(
    delete,
    path = "/timers/delete/{id}",
    tag = "⏰ Timers",
    summary = "Delete timer",
    description = "Delete a timer by its unique identifier",
    params(
        ("id" = i32, Path, description = "Timer ID to delete")
    ),
    responses(
        (status = 200, description = "Timer deleted successfully"),
        (status = 404, description = "Timer not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_timer(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<String>>, StatusCode> {
    let service = TimerService::new();
    let result = service.delete(&state.db, id).await;
    Ok(Json(result))
}
