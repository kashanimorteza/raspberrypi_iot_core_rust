//--------------------------------------------------------------------------------- Location
// src/api/handlers/timer_device.rs

//--------------------------------------------------------------------------------- Description
// Axum handlers for TimerDevice CRUD operations

//--------------------------------------------------------------------------------- Import
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use std::collections::HashMap;
use utoipa::ToSchema;
use crate::{orm::models::timer_device::Model as TimerDeviceModel, logics::general::ModelOutput, AppState};
use crate::api::services::timer_device::TimerDeviceService;

//--------------------------------------------------------------------------------- Request DTOs
#[derive(Deserialize, ToSchema)]
#[schema(description = "Request payload for creating a new timer device")]
pub struct CreateTimerDeviceRequest {
    #[schema(example = 1)]
    pub timer_id: i32,
    #[schema(example = 1)]
    pub device_id: i32,
    #[schema(example = 1)]
    pub command_id: i32,
    #[schema(example = "Timer device for LED control")]
    pub description: String,
    #[schema(example = true)]
    pub enable: bool,
}

#[derive(Deserialize, ToSchema)]
#[schema(description = "Request payload for updating an existing timer device")]
pub struct UpdateTimerDeviceRequest {
    #[schema(example = 1)]
    pub timer_id: Option<i32>,
    #[schema(example = 1)]
    pub device_id: Option<i32>,
    #[schema(example = 1)]
    pub command_id: Option<i32>,
    #[schema(example = "Timer device for LED control")]
    pub description: Option<String>,
    #[schema(example = true)]
    pub enable: Option<bool>,
}

//--------------------------------------------------------------------------------- Handlers
//------------------------- ListTimerDevices
#[utoipa::path(
    get,
    path = "/timer_device/items",
    tag = "🔗 Timer Device",

    params(
        ("limit" = Option<i32>, Query, description = "Maximum number of timer devices to return"),
        ("offset" = Option<i32>, Query, description = "Number of timer devices to skip"),
    ),
    responses(
        (status = 200, description = "List of timer devices retrieved successfully", body = Vec<TimerDeviceModel>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn list_timer_devices(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<ModelOutput<Vec<TimerDeviceModel>>>, StatusCode> {
    let service = TimerDeviceService::new();
    let result = service.items(&state.db, params).await;
    Ok(Json(result))
}

//------------------------- GetTimerDevice
#[utoipa::path(
    get,
    path = "/timer_device/item/{id}",
    tag = "🔗 Timer Device",

    params(
        ("id" = i32, Path, description = "Timer device ID")
    ),
    responses(
        (status = 200, description = "Timer device retrieved successfully", body = TimerDeviceModel),
        (status = 404, description = "Timer device not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_timer_device(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<TimerDeviceModel>>, StatusCode> {
    let service = TimerDeviceService::new();
    let result = service.item(&state.db, id).await;
    Ok(Json(result))
}

//------------------------- EnableTimerDevice
#[utoipa::path(
    get,
    path = "/timer_device/enable/{id}",
    tag = "🔗 Timer Device",

    params(
        ("id" = i32, Path, description = "Timer device ID to enable")
    ),
    responses(
        (status = 200, description = "Timer device enabled successfully", body = TimerDeviceModel),
        (status = 404, description = "Timer device not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn enable_timer_device(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<TimerDeviceModel>>, StatusCode> {
    let service = TimerDeviceService::new();
    let result = service.enable(&state.db, id).await;
    Ok(Json(result))
}

//------------------------- DisableTimerDevice
#[utoipa::path(
    get,
    path = "/timer_device/disable/{id}",
    tag = "🔗 Timer Device",

    params(
        ("id" = i32, Path, description = "Timer device ID to disable")
    ),
    responses(
        (status = 200, description = "Timer device disabled successfully", body = TimerDeviceModel),
        (status = 404, description = "Timer device not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn disable_timer_device(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<TimerDeviceModel>>, StatusCode> {
    let service = TimerDeviceService::new();
    let result = service.disable(&state.db, id).await;
    Ok(Json(result))
}

//------------------------- UpdateTimerDevice
#[utoipa::path(
    put,
    path = "/timer_device/update/{id}",
    tag = "🔗 Timer Device",

    params(
        ("id" = i32, Path, description = "Timer device ID to update")
    ),
    request_body = UpdateTimerDeviceRequest,
    responses(
        (status = 200, description = "Timer device updated successfully", body = TimerDeviceModel),
        (status = 404, description = "Timer device not found"),
        (status = 400, description = "Invalid request data"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_timer_device(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateTimerDeviceRequest>,
) -> Result<Json<ModelOutput<TimerDeviceModel>>, StatusCode> {
    let service = TimerDeviceService::new();
    
    let timer_device_model = TimerDeviceModel {
        id,
        timer_id: payload.timer_id.unwrap_or_default(),
        device_id: payload.device_id.unwrap_or_default(),
        command_id: payload.command_id.unwrap_or_default(),
        description: payload.description.unwrap_or_default(),
        enable: payload.enable.unwrap_or(true),
    };
    
    let result = service.update(&state.db, timer_device_model).await;
    Ok(Json(result))
}

//------------------------- CreateTimerDevice
#[utoipa::path(
    post,
    path = "/timer_device/add",
    tag = "🔗 Timer Device",

    request_body = CreateTimerDeviceRequest,
    responses(
        (status = 201, description = "Timer device created successfully", body = TimerDeviceModel),
        (status = 400, description = "Invalid request data"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_timer_device(
    State(state): State<AppState>,
    Json(payload): Json<CreateTimerDeviceRequest>,
) -> Result<Json<ModelOutput<TimerDeviceModel>>, StatusCode> {
    let service = TimerDeviceService::new();
    let timer_device_model = TimerDeviceModel {
        id: 0, // Will be auto-generated
        timer_id: payload.timer_id,
        device_id: payload.device_id,
        command_id: payload.command_id,
        description: payload.description,
        enable: payload.enable,
    };
    
    let result = service.add(&state.db, timer_device_model).await;
    Ok(Json(result))
}

//------------------------- DeleteTimerDevice
#[utoipa::path(
    delete,
    path = "/timer_device/delete/{id}",
    tag = "🔗 Timer Device",

    params(
        ("id" = i32, Path, description = "Timer device ID to delete")
    ),
    responses(
        (status = 200, description = "Timer device deleted successfully"),
        (status = 404, description = "Timer device not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_timer_device(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<String>>, StatusCode> {
    let service = TimerDeviceService::new();
    let result = service.delete(&state.db, id).await;
    Ok(Json(result))
}

//------------------------- StatusTimerDevice
#[utoipa::path(
    get,
    path = "/timer_device/status/{id}",
    tag = "⏰📱 Timer Device",

    params(
        ("id" = i32, Path, description = "Timer Device ID to toggle status")
    ),
    responses(
        (status = 200, description = "Timer Device status toggled successfully", body = TimerDeviceModel),
        (status = 404, description = "Timer Device not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn status_timer_device(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<TimerDeviceModel>>, StatusCode> {
    let service = TimerDeviceService::new();
    let result = service.status(&state.db, id).await;
    Ok(Json(result))
}
