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
use utoipa::ToSchema;
use crate::{orm::models::device_command::Model as DeviceCommandModel, logics::general::ModelOutput, AppState};
use crate::api::services::device_command::DeviceCommandService;

//--------------------------------------------------------------------------------- Request DTOs
#[derive(Deserialize, ToSchema)]
#[schema(description = "Request payload for creating a new device command")]
pub struct CreateDeviceCommandRequest {
    #[schema(example = 1)]
    pub device_id: i32,
    #[schema(example = "Turn On LED")]
    pub name: String,
    #[schema(example = 0)]
    pub value_from: Option<i32>,
    #[schema(example = 1)]
    pub value_to: Option<i32>,
    #[schema(example = 1000)]
    pub delay: Option<i32>,
    #[schema(example = "Command to turn on LED")]
    pub description: String,
    #[schema(example = false)]
    pub reload: bool,
    #[schema(example = true)]
    pub enable: bool,
    #[schema(example = "digital")]
    pub r#type: String,
}

#[derive(Deserialize, ToSchema)]
#[schema(description = "Request payload for updating an existing device command")]
pub struct UpdateDeviceCommandRequest {
    #[schema(example = 1)]
    pub device_id: Option<i32>,
    #[schema(example = "Turn On LED")]
    pub name: Option<String>,
    #[schema(example = 0)]
    pub value_from: Option<i32>,
    #[schema(example = 1)]
    pub value_to: Option<i32>,
    #[schema(example = 1000)]
    pub delay: Option<i32>,
    #[schema(example = "Command to turn on LED")]
    pub description: Option<String>,
    #[schema(example = false)]
    pub reload: Option<bool>,
    #[schema(example = true)]
    pub enable: Option<bool>,
    #[schema(example = "digital")]
    pub r#type: Option<String>,
}

//--------------------------------------------------------------------------------- Handlers
#[utoipa::path(
    get,
    path = "/device_commands/items",
    tag = "📡 Device Commands",

    params(
        ("limit" = Option<i32>, Query, description = "Maximum number of device commands to return"),
        ("offset" = Option<i32>, Query, description = "Number of device commands to skip"),
    ),
    responses(
        (status = 200, description = "List of device commands retrieved successfully", body = Vec<DeviceCommandModel>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn list_device_commands(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<ModelOutput<Vec<DeviceCommandModel>>>, StatusCode> {
    let service = DeviceCommandService::new();
    let result = service.items(&state.db, params).await;
    Ok(Json(result))
}

#[utoipa::path(
    get,
    path = "/device_commands/{id}",
    tag = "📡 Device Commands",

    params(
        ("id" = i32, Path, description = "Device command ID")
    ),
    responses(
        (status = 200, description = "Device command retrieved successfully", body = DeviceCommandModel),
        (status = 404, description = "Device command not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_device_command(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<DeviceCommandModel>>, StatusCode> {
    let service = DeviceCommandService::new();
    let result = service.item(&state.db, id).await;
    Ok(Json(result))
}

#[utoipa::path(
    post,
    path = "/device_commands/add",
    tag = "📡 Device Commands",

    request_body = CreateDeviceCommandRequest,
    responses(
        (status = 201, description = "Device command created successfully", body = DeviceCommandModel),
        (status = 400, description = "Invalid request data"),
        (status = 500, description = "Internal server error")
    )
)]
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

#[utoipa::path(
    put,
    path = "/device_commands/update/{id}",
    tag = "📡 Device Commands",

    params(
        ("id" = i32, Path, description = "Device command ID to update")
    ),
    request_body = UpdateDeviceCommandRequest,
    responses(
        (status = 200, description = "Device command updated successfully", body = DeviceCommandModel),
        (status = 404, description = "Device command not found"),
        (status = 400, description = "Invalid request data"),
        (status = 500, description = "Internal server error")
    )
)]
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

#[utoipa::path(
    delete,
    path = "/device_commands/delete/{id}",
    tag = "📡 Device Commands",

    params(
        ("id" = i32, Path, description = "Device command ID to delete")
    ),
    responses(
        (status = 200, description = "Device command deleted successfully"),
        (status = 404, description = "Device command not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_device_command(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<String>>, StatusCode> {
    let service = DeviceCommandService::new();
    let result = service.delete(&state.db, id).await;
    Ok(Json(result))
}
