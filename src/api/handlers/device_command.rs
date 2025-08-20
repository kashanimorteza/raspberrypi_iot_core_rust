//--------------------------------------------------------------------------------- Location
// src/api/handlers/device_command.rs

//--------------------------------------------------------------------------------- Description
// Axum handlers for DeviceCommand CRUD operations

//--------------------------------------------------------------------------------- Import
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use std::collections::HashMap;
use crate::{orm::models::device_command::Model as DeviceCommandModel, logics::general::ModelOutput, AppState};
use crate::api::services::device_command::DeviceCommandService;

//--------------------------------------------------------------------------------- Request DTOs
#[derive(Deserialize)]
pub struct CreateDeviceCommandRequest {
    pub device_id: i32,
    pub name: String,
    pub value_from: Option<i32>,
    pub value_to: Option<i32>,
    pub delay: Option<i32>,
    pub description: String,
    pub reload: bool,
    pub enable: bool,
    pub r#type: String,
}

#[derive(Deserialize)]
pub struct UpdateDeviceCommandRequest {
    pub device_id: Option<i32>,
    pub name: Option<String>,
    pub value_from: Option<i32>,
    pub value_to: Option<i32>,
    pub delay: Option<i32>,
    pub description: Option<String>,
    pub reload: Option<bool>,
    pub enable: Option<bool>,
    pub r#type: Option<String>,
}

//--------------------------------------------------------------------------------- Handlers
pub async fn list_device_commands(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<ModelOutput<Vec<DeviceCommandModel>>>, StatusCode> {
    let service = DeviceCommandService::new();
    let result = service.items(&state.db, params).await;
    Ok(Json(result))
}

pub async fn get_device_command(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<DeviceCommandModel>>, StatusCode> {
    let service = DeviceCommandService::new();
    let result = service.item(&state.db, id).await;
    Ok(Json(result))
}

pub async fn create_device_command(
    State(state): State<AppState>,
    Json(payload): Json<CreateDeviceCommandRequest>,
) -> Result<Json<ModelOutput<DeviceCommandModel>>, StatusCode> {
    let service = DeviceCommandService::new();
    let device_command_model = DeviceCommandModel {
        id: 0, // Will be auto-generated
        device_id: payload.device_id,
        name: payload.name,
        value_from: payload.value_from,
        value_to: payload.value_to,
        delay: payload.delay,
        description: payload.description,
        reload: payload.reload,
        enable: payload.enable,
        r#type: payload.r#type,
    };
    
    let result = service.add(&state.db, device_command_model).await;
    Ok(Json(result))
}

pub async fn update_device_command(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateDeviceCommandRequest>,
) -> Result<Json<ModelOutput<DeviceCommandModel>>, StatusCode> {
    let service = DeviceCommandService::new();
    
    let device_command_model = DeviceCommandModel {
        id,
        device_id: payload.device_id.unwrap_or_default(),
        name: payload.name.unwrap_or_default(),
        value_from: payload.value_from,
        value_to: payload.value_to,
        delay: payload.delay,
        description: payload.description.unwrap_or_default(),
        reload: payload.reload.unwrap_or(false),
        enable: payload.enable.unwrap_or(true),
        r#type: payload.r#type.unwrap_or_default(),
    };
    
    let result = service.update(&state.db, device_command_model).await;
    Ok(Json(result))
}

pub async fn delete_device_command(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<String>>, StatusCode> {
    let service = DeviceCommandService::new();
    let result = service.delete(&state.db, id).await;
    Ok(Json(result))
}
