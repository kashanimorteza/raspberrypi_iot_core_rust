//--------------------------------------------------------------------------------- Location
// src/api/handlers/device.rs

//--------------------------------------------------------------------------------- Description
// Axum handlers for Device CRUD operations

//--------------------------------------------------------------------------------- Import
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use std::collections::HashMap;
use crate::{orm::models::device::Model as DeviceModel, logics::general::ModelOutput, AppState};
use crate::api::services::device::DeviceService;

//--------------------------------------------------------------------------------- Request DTOs
#[derive(Deserialize)]
pub struct CreateDeviceRequest {
    pub zone_id: i32,
    pub port_id: i32,
    pub power_id: i32,
    pub command_id: i32,
    pub value: i32,
    pub tune: i32,
    pub date: String,
    pub address: String,
    pub name: String,
    pub description: String,
    pub enable: bool,
}

#[derive(Deserialize)]
pub struct UpdateDeviceRequest {
    pub zone_id: Option<i32>,
    pub port_id: Option<i32>,
    pub power_id: Option<i32>,
    pub command_id: Option<i32>,
    pub value: Option<i32>,
    pub tune: Option<i32>,
    pub date: Option<String>,
    pub address: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub enable: Option<bool>,
}

//--------------------------------------------------------------------------------- Handlers
pub async fn list_devices(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<ModelOutput<Vec<DeviceModel>>>, StatusCode> {
    let service = DeviceService::new();
    let result = service.items(&state.db, params).await;
    Ok(Json(result))
}

pub async fn get_device(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<DeviceModel>>, StatusCode> {
    let service = DeviceService::new();
    let result = service.item(&state.db, id).await;
    Ok(Json(result))
}

pub async fn create_device(
    State(state): State<AppState>,
    Json(payload): Json<CreateDeviceRequest>,
) -> Result<Json<ModelOutput<DeviceModel>>, StatusCode> {
    let service = DeviceService::new();
    let device_model = DeviceModel {
        id: 0, // Will be auto-generated
        zone_id: payload.zone_id,
        port_id: payload.port_id,
        power_id: payload.power_id,
        command_id: payload.command_id,
        value: payload.value,
        tune: payload.tune,
        date: payload.date,
        address: payload.address,
        name: payload.name,
        description: payload.description,
        enable: payload.enable,
    };
    
    let result = service.add(&state.db, device_model).await;
    Ok(Json(result))
}

pub async fn update_device(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateDeviceRequest>,
) -> Result<Json<ModelOutput<DeviceModel>>, StatusCode> {
    let service = DeviceService::new();
    
    let device_model = DeviceModel {
        id,
        zone_id: payload.zone_id.unwrap_or_default(),
        port_id: payload.port_id.unwrap_or_default(),
        power_id: payload.power_id.unwrap_or_default(),
        command_id: payload.command_id.unwrap_or_default(),
        value: payload.value.unwrap_or_default(),
        tune: payload.tune.unwrap_or_default(),
        date: payload.date.unwrap_or_default(),
        address: payload.address.unwrap_or_default(),
        name: payload.name.unwrap_or_default(),
        description: payload.description.unwrap_or_default(),
        enable: payload.enable.unwrap_or(true),
    };
    
    let result = service.update(&state.db, device_model).await;
    Ok(Json(result))
}

pub async fn delete_device(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<String>>, StatusCode> {
    let service = DeviceService::new();
    let result = service.delete(&state.db, id).await;
    Ok(Json(result))
}
