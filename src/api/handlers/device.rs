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
use utoipa::ToSchema;
use crate::{orm::models::device::Model as DeviceModel, logics::general::ModelOutput, AppState};
use crate::api::services::device::DeviceService;

//--------------------------------------------------------------------------------- Request DTOs
#[derive(Deserialize, ToSchema)]
#[schema(description = "Request payload for creating a new device")]
pub struct CreateDeviceRequest {
    #[schema(example = 1)]
    pub zone_id: i32,
    #[schema(example = 1)]
    pub port_id: i32,
    #[schema(example = 1)]
    pub power_id: i32,
    #[schema(example = 1)]
    pub command_id: i32,
    #[schema(example = 100)]
    pub value: i32,
    #[schema(example = 50)]
    pub tune: i32,
    #[schema(example = "2024-01-15")]
    pub date: String,
    #[schema(example = "192.168.1.100")]
    pub address: String,
    #[schema(example = "Living Room Sensor")]
    pub name: String,
    #[schema(example = "Temperature and humidity sensor")]
    pub description: String,
    #[schema(example = true)]
    pub enable: bool,
}

#[derive(Deserialize, ToSchema)]
#[schema(description = "Request payload for updating an existing device")]
pub struct UpdateDeviceRequest {
    #[schema(example = 1)]
    pub zone_id: Option<i32>,
    #[schema(example = 1)]
    pub port_id: Option<i32>,
    #[schema(example = 1)]
    pub power_id: Option<i32>,
    #[schema(example = 1)]
    pub command_id: Option<i32>,
    #[schema(example = 100)]
    pub value: Option<i32>,
    #[schema(example = 50)]
    pub tune: Option<i32>,
    #[schema(example = "2024-01-15")]
    pub date: Option<String>,
    #[schema(example = "192.168.1.100")]
    pub address: Option<String>,
    #[schema(example = "Living Room Sensor")]
    pub name: Option<String>,
    #[schema(example = "Temperature and humidity sensor")]
    pub description: Option<String>,
    #[schema(example = true)]
    pub enable: Option<bool>,
}

//--------------------------------------------------------------------------------- Handlers
//------------------------- ListDevices
#[utoipa::path(
    get,
    path = "/device/items",
    tag = "🔧 Device",
    params(
        ("limit" = Option<i32>, Query, description = "Maximum number of devices to return"),
        ("offset" = Option<i32>, Query, description = "Number of devices to skip"),
    ),
    responses(
        (status = 200, description = "List of devices retrieved successfully", body = Vec<DeviceModel>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn list_devices(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<ModelOutput<Vec<DeviceModel>>>, StatusCode> {
    let service = DeviceService::new();
    let result = service.items(&state.db, params).await;
    Ok(Json(result))
}

//------------------------- GetDevice
#[utoipa::path(
    get,
    path = "/device/item/{id}",
    tag = "🔧 Device",

    params(
        ("id" = i32, Path, description = "Device ID")
    ),
    responses(
        (status = 200, description = "Device retrieved successfully", body = DeviceModel),
        (status = 404, description = "Device not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_device(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<DeviceModel>>, StatusCode> {
    let service = DeviceService::new();
    let result = service.item(&state.db, id).await;
    Ok(Json(result))
}

//------------------------- EnableDevice
#[utoipa::path(
    get,
    path = "/device/enable/{id}",
    tag = "🔧 Device",

    params(
        ("id" = i32, Path, description = "Device ID to enable")
    ),
    responses(
        (status = 200, description = "Device enabled successfully", body = DeviceModel),
        (status = 404, description = "Device not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn enable_device(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<DeviceModel>>, StatusCode> {
    let service = DeviceService::new();
    let result = service.enable(&state.db, id).await;
    Ok(Json(result))
}

//------------------------- DisableDevice
#[utoipa::path(
    get,
    path = "/device/disable/{id}",
    tag = "🔧 Device",

    params(
        ("id" = i32, Path, description = "Device ID to disable")
    ),
    responses(
        (status = 200, description = "Device disabled successfully", body = DeviceModel),
        (status = 404, description = "Device not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn disable_device(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<DeviceModel>>, StatusCode> {
    let service = DeviceService::new();
    let result = service.disable(&state.db, id).await;
    Ok(Json(result))
}

//------------------------- UpdateDevice
#[utoipa::path(
    put,
    path = "/device/update/{id}",
    tag = "🔧 Device",

    params(
        ("id" = i32, Path, description = "Device ID to update")
    ),
    request_body = UpdateDeviceRequest,
    responses(
        (status = 200, description = "Device updated successfully", body = DeviceModel),
        (status = 404, description = "Device not found"),
        (status = 400, description = "Invalid request data"),
        (status = 500, description = "Internal server error")
    )
)]
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

//------------------------- CreateDevice
#[utoipa::path(
    post,
    path = "/device/add",
    tag = "🔧 Device",

    request_body = CreateDeviceRequest,
    responses(
        (status = 201, description = "Device created successfully", body = DeviceModel),
        (status = 400, description = "Invalid request data"),
        (status = 500, description = "Internal server error")
    )
)]
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

//------------------------- DeleteDevice
#[utoipa::path(
    delete,
    path = "/device/delete/{id}",
    tag = "🔧 Device",

    params(
        ("id" = i32, Path, description = "Device ID to delete")
    ),
    responses(
        (status = 200, description = "Device deleted successfully"),
        (status = 404, description = "Device not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_device(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<String>>, StatusCode> {
    let service = DeviceService::new();
    let result = service.delete(&state.db, id).await;
    Ok(Json(result))
}

//------------------------- StatusDevice
#[utoipa::path(
    get,
    path = "/device/status/{id}",
    tag = "📱 Device",

    params(
        ("id" = i32, Path, description = "Device ID to toggle status")
    ),
    responses(
        (status = 200, description = "Device status toggled successfully", body = DeviceModel),
        (status = 404, description = "Device not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn status_device(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<DeviceModel>>, StatusCode> {
    let service = DeviceService::new();
    let result = service.status(&state.db, id).await;
    Ok(Json(result))
}
