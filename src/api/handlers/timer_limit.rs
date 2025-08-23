//--------------------------------------------------------------------------------- Location
// src/api/handlers/timer_limit.rs

//--------------------------------------------------------------------------------- Description
// Axum handlers for TimerLimit CRUD operations

//--------------------------------------------------------------------------------- Import
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use std::collections::HashMap;
use utoipa::ToSchema;
use crate::{orm::models::timer_limit::Model as TimerLimitModel, logics::general::ModelOutput, AppState};
use crate::api::services::timer_limit::TimerLimitService;

//--------------------------------------------------------------------------------- Request DTOs
#[derive(Deserialize, ToSchema)]
#[schema(description = "Request payload for creating a new timer limit")]
pub struct CreateTimerLimitRequest {
    #[schema(example = 1)]
    pub device_id: i32,
    #[schema(example = 1)]
    pub command_from_id: i32,
    #[schema(example = 2)]
    pub command_to_id: i32,
    #[schema(example = 100)]
    pub value: i32,
    #[schema(example = "Timer limit for device control")]
    pub description: String,
    #[schema(example = true)]
    pub enable: bool,
}

#[derive(Deserialize, ToSchema)]
#[schema(description = "Request payload for updating an existing timer limit")]
pub struct UpdateTimerLimitRequest {
    #[schema(example = 1)]
    pub device_id: Option<i32>,
    #[schema(example = 1)]
    pub command_from_id: Option<i32>,
    #[schema(example = 2)]
    pub command_to_id: Option<i32>,
    #[schema(example = 100)]
    pub value: Option<i32>,
    #[schema(example = "Timer limit for device control")]
    pub description: Option<String>,
    #[schema(example = true)]
    pub enable: Option<bool>,
}

//--------------------------------------------------------------------------------- Handlers
//------------------------- ListTimerLimits
#[utoipa::path(
    get,
    path = "/timer_limit/items",
    tag = "⏱️ Timer Limit",

    params(
        ("limit" = Option<i32>, Query, description = "Maximum number of timer limits to return"),
        ("offset" = Option<i32>, Query, description = "Number of timer limits to skip"),
    ),
    responses(
        (status = 200, description = "List of timer limits retrieved successfully", body = Vec<TimerLimitModel>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn list_timer_limits(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<ModelOutput<Vec<TimerLimitModel>>>, StatusCode> {
    let service = TimerLimitService::new();
    let result = service.items(&state.db, params).await;
    Ok(Json(result))
}

//------------------------- GetTimerLimit
#[utoipa::path(
    get,
    path = "/timer_limit/item/{id}",
    tag = "⏱️ Timer Limit",

    params(
        ("id" = i32, Path, description = "Timer limit ID")
    ),
    responses(
        (status = 200, description = "Timer limit retrieved successfully", body = TimerLimitModel),
        (status = 404, description = "Timer limit not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_timer_limit(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<TimerLimitModel>>, StatusCode> {
    let service = TimerLimitService::new();
    let result = service.item(&state.db, id).await;
    Ok(Json(result))
}

//------------------------- EnableTimerLimit
#[utoipa::path(
    get,
    path = "/timer_limit/enable/{id}",
    tag = "⏱️ Timer Limit",

    params(
        ("id" = i32, Path, description = "Timer limit ID to enable")
    ),
    responses(
        (status = 200, description = "Timer limit enabled successfully", body = TimerLimitModel),
        (status = 404, description = "Timer limit not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn enable_timer_limit(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<TimerLimitModel>>, StatusCode> {
    let service = TimerLimitService::new();
    let result = service.enable(&state.db, id).await;
    Ok(Json(result))
}

//------------------------- DisableTimerLimit
#[utoipa::path(
    get,
    path = "/timer_limit/disable/{id}",
    tag = "⏱️ Timer Limit",

    params(
        ("id" = i32, Path, description = "Timer limit ID to disable")
    ),
    responses(
        (status = 200, description = "Timer limit disabled successfully", body = TimerLimitModel),
        (status = 404, description = "Timer limit not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn disable_timer_limit(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<TimerLimitModel>>, StatusCode> {
    let service = TimerLimitService::new();
    let result = service.disable(&state.db, id).await;
    Ok(Json(result))
}

//------------------------- UpdateTimerLimit
#[utoipa::path(
    put,
    path = "/timer_limit/update/{id}",
    tag = "⏱️ Timer Limit",

    params(
        ("id" = i32, Path, description = "Timer limit ID to update")
    ),
    request_body = UpdateTimerLimitRequest,
    responses(
        (status = 200, description = "Timer limit updated successfully", body = TimerLimitModel),
        (status = 404, description = "Timer limit not found"),
        (status = 400, description = "Invalid request data"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_timer_limit(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateTimerLimitRequest>,
) -> Result<Json<ModelOutput<TimerLimitModel>>, StatusCode> {
    let service = TimerLimitService::new();
    
    let timer_limit_model = TimerLimitModel {
        id,
        device_id: payload.device_id.unwrap_or_default(),
        command_from_id: payload.command_from_id.unwrap_or_default(),
        command_to_id: payload.command_to_id.unwrap_or_default(),
        value: payload.value.unwrap_or_default(),
        description: payload.description.unwrap_or_default(),
        enable: payload.enable.unwrap_or(true),
    };
    
    let result = service.update(&state.db, timer_limit_model).await;
    Ok(Json(result))
}

//------------------------- CreateTimerLimit
#[utoipa::path(
    post,
    path = "/timer_limit/add",
    tag = "⏱️ Timer Limit",

    request_body = CreateTimerLimitRequest,
    responses(
        (status = 201, description = "Timer limit created successfully", body = TimerLimitModel),
        (status = 400, description = "Invalid request data"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_timer_limit(
    State(state): State<AppState>,
    Json(payload): Json<CreateTimerLimitRequest>,
) -> Result<Json<ModelOutput<TimerLimitModel>>, StatusCode> {
    let service = TimerLimitService::new();
    let timer_limit_model = TimerLimitModel {
        id: 0, // Will be auto-generated
        device_id: payload.device_id,
        command_from_id: payload.command_from_id,
        command_to_id: payload.command_to_id,
        value: payload.value,
        description: payload.description,
        enable: payload.enable,
    };
    
    let result = service.add(&state.db, timer_limit_model).await;
    Ok(Json(result))
}

//------------------------- DeleteTimerLimit
#[utoipa::path(
    delete,
    path = "/timer_limit/delete/{id}",
    tag = "⏱️ Timer Limit",

    params(
        ("id" = i32, Path, description = "Timer limit ID to delete")
    ),
    responses(
        (status = 200, description = "Timer limit deleted successfully"),
        (status = 404, description = "Timer limit not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_timer_limit(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<String>>, StatusCode> {
    let service = TimerLimitService::new();
    let result = service.delete(&state.db, id).await;
    Ok(Json(result))
}
