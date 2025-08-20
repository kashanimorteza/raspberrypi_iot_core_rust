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
use crate::{orm::models::timer_device::Model as TimerDeviceModel, logics::general::ModelOutput, AppState};
use crate::api::services::timer_device::TimerDeviceService;

//--------------------------------------------------------------------------------- Request DTOs
#[derive(Deserialize)]
pub struct CreateTimerDeviceRequest {
    pub timer_id: i32,
    pub device_id: i32,
    pub command_id: i32,
    pub description: String,
    pub enable: bool,
}

#[derive(Deserialize)]
pub struct UpdateTimerDeviceRequest {
    pub timer_id: Option<i32>,
    pub device_id: Option<i32>,
    pub command_id: Option<i32>,
    pub description: Option<String>,
    pub enable: Option<bool>,
}

//--------------------------------------------------------------------------------- Handlers
pub async fn list_timer_devices(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<ModelOutput<Vec<TimerDeviceModel>>>, StatusCode> {
    let service = TimerDeviceService::new();
    let result = service.items(&state.db, params).await;
    Ok(Json(result))
}

pub async fn get_timer_device(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<TimerDeviceModel>>, StatusCode> {
    let service = TimerDeviceService::new();
    let result = service.item(&state.db, id).await;
    Ok(Json(result))
}

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

pub async fn delete_timer_device(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<String>>, StatusCode> {
    let service = TimerDeviceService::new();
    let result = service.delete(&state.db, id).await;
    Ok(Json(result))
}
