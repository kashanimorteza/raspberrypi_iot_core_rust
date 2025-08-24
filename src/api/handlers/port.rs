//--------------------------------------------------------------------------------- Location
// src/api/handlers/port.rs

//--------------------------------------------------------------------------------- Description
// Port Handler

//--------------------------------------------------------------------------------- Import
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use std::collections::HashMap;
use utoipa::ToSchema;
use crate::{orm::models::port::Model as PortModel, logics::general::ModelOutput, AppState};
use crate::api::services::port::PortService;

//--------------------------------------------------------------------------------- Request DTOs
#[derive(Deserialize, ToSchema)]
#[schema(description = "Request payload for creating a new port")]
pub struct CreatePortRequest {
    pub user_id: i32,
    pub name: String,
    pub pin: Option<i32>,
    pub port: Option<i32>,
    pub value: Option<i32>,
    pub description: String,
    pub enable: bool,
    pub protocol: String,
    pub r#type: String,
}

#[derive(Deserialize, ToSchema)]
#[schema(description = "Request payload for updating an existing port")]
pub struct UpdatePortRequest {
    pub user_id: Option<i32>,
    pub name: Option<String>,
    pub pin: Option<i32>,
    pub port: Option<i32>,
    pub value: Option<i32>,
    pub description: Option<String>,
    pub enable: Option<bool>,
    pub protocol: Option<String>,
    pub r#type: Option<String>,
}

//--------------------------------------------------------------------------------- Handlers
//------------------------- List
#[utoipa::path(
    get,
    path = "/port/items",
    tag = "🔌 Port",
    params(
        ("limit" = Option<i32>, Query, description = "Maximum number of ports to return"),
        ("offset" = Option<i32>, Query, description = "Number of ports to skip"),
    ),
    responses(
        (status = 200, description = "List of ports retrieved successfully", body = Vec<PortModel>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn list_ports(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<ModelOutput<Vec<PortModel>>>, StatusCode> {
    let service = PortService::new();
    let result = service.items(&state.db, params).await;
    Ok(Json(result))
}

//------------------------- Get
#[utoipa::path(
    get,
    path = "/port/item/{id}",
    tag = "🔌 Port",

    params(
        ("id" = i32, Path, description = "Port ID")
    ),
    responses(
        (status = 200, description = "Port retrieved successfully", body = PortModel),
        (status = 404, description = "Port not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_port(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<PortModel>>, StatusCode> {
    let service = PortService::new();
    let result = service.item(&state.db, id).await;
    Ok(Json(result))
}

//------------------------- Enable
#[utoipa::path(
    get,
    path = "/port/enable/{id}",
    tag = "🔌 Port",

    params(
        ("id" = i32, Path, description = "Port ID to enable")
    ),
    responses(
        (status = 200, description = "Port enabled successfully", body = PortModel),
        (status = 404, description = "Port not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn enable_port(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<PortModel>>, StatusCode> {
    let service = PortService::new();
    let result = service.enable(&state.db, id).await;
    Ok(Json(result))
}

//------------------------- Disable
#[utoipa::path(
    get,
    path = "/port/disable/{id}",
    tag = "🔌 Port",
    params(
        ("id" = i32, Path, description = "Port ID to disable")
    ),
    responses(
        (status = 200, description = "Port disabled successfully", body = PortModel),
        (status = 404, description = "Port not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn disable_port(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<PortModel>>, StatusCode> {
    let service = PortService::new();
    let result = service.disable(&state.db, id).await;
    Ok(Json(result))
}

//------------------------- Update
#[utoipa::path(
    put,
    path = "/port/update/{id}",
    tag = "🔌 Port",

    params(
        ("id" = i32, Path, description = "Port ID to update")
    ),
    request_body = UpdatePortRequest,
    responses(
        (status = 200, description = "Port updated successfully", body = PortModel),
        (status = 404, description = "Port not found"),
        (status = 400, description = "Invalid request data"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_port(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdatePortRequest>,
) -> Result<Json<ModelOutput<PortModel>>, StatusCode> {
    let service = PortService::new();
    
    let port_model = PortModel {
        id,
        user_id: payload.user_id.unwrap_or_default(),
        name: payload.name.unwrap_or_default(),
        pin: payload.pin.unwrap_or(0),
        port: payload.port.unwrap_or(0),
        value: payload.value.unwrap_or(0),
        description: payload.description.unwrap_or_default(),
        enable: payload.enable.unwrap_or(true),
        protocol: payload.protocol.unwrap_or_default(),
        r#type: payload.r#type.unwrap_or_default(),
    };
    
    let result = service.update(&state.db, port_model).await;
    Ok(Json(result))
}

//------------------------- Create
#[utoipa::path(
    post,
    path = "/port/add",
    tag = "🔌 Port",
    request_body = CreatePortRequest,
    responses(
        (status = 201, description = "Port created successfully", body = PortModel),
        (status = 400, description = "Invalid request data"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_port(
    State(state): State<AppState>,
    Json(payload): Json<CreatePortRequest>,
) -> Result<Json<ModelOutput<PortModel>>, StatusCode> {
    let service = PortService::new();
    let port_model = PortModel {
        id: 0, // Will be auto-generated
        user_id: payload.user_id,
        name: payload.name,
        pin: payload.pin.unwrap_or(0),
        port: payload.port.unwrap_or(0),
        value: payload.value.unwrap_or(0),
        description: payload.description,
        enable: payload.enable,
        protocol: payload.protocol,
        r#type: payload.r#type,
    };
    
    let result = service.add(&state.db, port_model).await;
    Ok(Json(result))
}

//------------------------- Delete
#[utoipa::path(
    delete,
    path = "/port/delete/{id}",
    tag = "🔌 Port",

    params(
        ("id" = i32, Path, description = "Port ID to delete")
    ),
    responses(
        (status = 200, description = "Port deleted successfully"),
        (status = 404, description = "Port not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_port(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<String>>, StatusCode> {
    let service = PortService::new();
    let result = service.delete(&state.db, id).await;
    Ok(Json(result))
}

//------------------------- Status
#[utoipa::path(
    get,
    path = "/port/status/{id}",
    tag = "🔌 Port",

    params(
        ("id" = i32, Path, description = "Port ID to toggle status")
    ),
    responses(
        (status = 200, description = "Port status toggled successfully", body = PortModel),
        (status = 404, description = "Port not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn status_port(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<PortModel>>, StatusCode> {
    let service = PortService::new();
    let result = service.status(&state.db, id).await;
    Ok(Json(result))
}
