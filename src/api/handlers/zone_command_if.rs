//--------------------------------------------------------------------------------- Location
// src/api/handlers/zone_command_if.rs

//--------------------------------------------------------------------------------- Description
// Axum handlers for ZoneCommandIf CRUD operations

//--------------------------------------------------------------------------------- Import
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use std::collections::HashMap;
use utoipa::ToSchema;
use crate::{orm::models::zone_command_if::Model as ZoneCommandIfModel, logics::general::ModelOutput, AppState};
use crate::api::services::zone_command_if::ZoneCommandIfService;

//--------------------------------------------------------------------------------- Request DTOs
#[derive(Deserialize, ToSchema)]
#[schema(description = "Request payload for creating a new zone command if condition")]
pub struct CreateZoneCommandIfRequest {
    #[schema(example = "Temperature Check")]
    pub name: String,
    #[schema(example = 1)]
    pub zone_command_id: i32,
    #[schema(example = 1)]
    pub device_id: i32,
    #[schema(example = 1)]
    pub command_id: i32,
    #[schema(example = "Check temperature condition")]
    pub description: String,
    #[schema(example = true)]
    pub enable: bool,
}

#[derive(Deserialize, ToSchema)]
#[schema(description = "Request payload for updating an existing zone command if condition")]
pub struct UpdateZoneCommandIfRequest {
    #[schema(example = "Temperature Check")]
    pub name: Option<String>,
    #[schema(example = 1)]
    pub zone_command_id: Option<i32>,
    #[schema(example = 1)]
    pub device_id: Option<i32>,
    #[schema(example = 1)]
    pub command_id: Option<i32>,
    #[schema(example = "Check temperature condition")]
    pub description: Option<String>,
    #[schema(example = true)]
    pub enable: Option<bool>,
}

//--------------------------------------------------------------------------------- Handlers
#[utoipa::path(
    get,
    path = "/zone_command_ifs/items",
    tag = "🔀 Zone Command Conditions",

    params(
        ("limit" = Option<i32>, Query, description = "Maximum number of zone command conditions to return"),
        ("offset" = Option<i32>, Query, description = "Number of zone command conditions to skip"),
    ),
    responses(
        (status = 200, description = "List of zone command conditions retrieved successfully", body = Vec<ZoneCommandIfModel>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn list_zone_command_ifs(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<ModelOutput<Vec<ZoneCommandIfModel>>>, StatusCode> {
    let service = ZoneCommandIfService::new();
    let result = service.items(&state.db, params).await;
    Ok(Json(result))
}

#[utoipa::path(
    get,
    path = "/zone_command_ifs/{id}",
    tag = "🔀 Zone Command Conditions",

    params(
        ("id" = i32, Path, description = "Zone command condition ID")
    ),
    responses(
        (status = 200, description = "Zone command condition retrieved successfully", body = ZoneCommandIfModel),
        (status = 404, description = "Zone command condition not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_zone_command_if(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<ZoneCommandIfModel>>, StatusCode> {
    let service = ZoneCommandIfService::new();
    let result = service.item(&state.db, id).await;
    Ok(Json(result))
}

#[utoipa::path(
    post,
    path = "/zone_command_ifs/add",
    tag = "🔀 Zone Command Conditions",

    request_body = CreateZoneCommandIfRequest,
    responses(
        (status = 201, description = "Zone command condition created successfully", body = ZoneCommandIfModel),
        (status = 400, description = "Invalid request data"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_zone_command_if(
    State(state): State<AppState>,
    Json(payload): Json<CreateZoneCommandIfRequest>,
) -> Result<Json<ModelOutput<ZoneCommandIfModel>>, StatusCode> {
    let service = ZoneCommandIfService::new();
    let zone_command_if_model = ZoneCommandIfModel {
        id: 0, // Will be auto-generated
        name: payload.name,
        zone_command_id: payload.zone_command_id,
        device_id: payload.device_id,
        command_id: payload.command_id,
        description: payload.description,
        enable: payload.enable,
    };
    
    let result = service.add(&state.db, zone_command_if_model).await;
    Ok(Json(result))
}

#[utoipa::path(
    put,
    path = "/zone_command_ifs/update/{id}",
    tag = "🔀 Zone Command Conditions",

    params(
        ("id" = i32, Path, description = "Zone command condition ID to update")
    ),
    request_body = UpdateZoneCommandIfRequest,
    responses(
        (status = 200, description = "Zone command condition updated successfully", body = ZoneCommandIfModel),
        (status = 404, description = "Zone command condition not found"),
        (status = 400, description = "Invalid request data"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_zone_command_if(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateZoneCommandIfRequest>,
) -> Result<Json<ModelOutput<ZoneCommandIfModel>>, StatusCode> {
    let service = ZoneCommandIfService::new();
    
    let zone_command_if_model = ZoneCommandIfModel {
        id,
        name: payload.name.unwrap_or_default(),
        zone_command_id: payload.zone_command_id.unwrap_or_default(),
        device_id: payload.device_id.unwrap_or_default(),
        command_id: payload.command_id.unwrap_or_default(),
        description: payload.description.unwrap_or_default(),
        enable: payload.enable.unwrap_or(true),
    };
    
    let result = service.update(&state.db, zone_command_if_model).await;
    Ok(Json(result))
}

#[utoipa::path(
    delete,
    path = "/zone_command_ifs/delete/{id}",
    tag = "🔀 Zone Command Conditions",

    params(
        ("id" = i32, Path, description = "Zone command condition ID to delete")
    ),
    responses(
        (status = 200, description = "Zone command condition deleted successfully"),
        (status = 404, description = "Zone command condition not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_zone_command_if(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<String>>, StatusCode> {
    let service = ZoneCommandIfService::new();
    let result = service.delete(&state.db, id).await;
    Ok(Json(result))
}
