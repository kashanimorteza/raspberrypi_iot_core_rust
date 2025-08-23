//--------------------------------------------------------------------------------- Location
// src/api/handlers/zone_command_action.rs

//--------------------------------------------------------------------------------- Description
// Axum handlers for ZoneCommandAction CRUD operations

//--------------------------------------------------------------------------------- Import
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use std::collections::HashMap;
use utoipa::ToSchema;
use crate::{orm::models::zone_command_action::Model as ZoneCommandActionModel, logics::general::ModelOutput, AppState};
use crate::api::services::zone_command_action::ZoneCommandActionService;

//--------------------------------------------------------------------------------- Request DTOs
#[derive(Deserialize, ToSchema)]
#[schema(description = "Request payload for creating a new zone command action")]
pub struct CreateZoneCommandActionRequest {
    #[schema(example = "Zone Action")]
    pub name: String,
    #[schema(example = 1)]
    pub zone_command_id: i32,
    #[schema(example = 1)]
    pub device_id: i32,
    #[schema(example = 1)]
    pub command_id: Option<i32>,
    #[schema(example = "Action to execute on zone command")]
    pub description: String,
    #[schema(example = true)]
    pub enable: bool,
}

#[derive(Deserialize, ToSchema)]
#[schema(description = "Request payload for updating an existing zone command action")]
pub struct UpdateZoneCommandActionRequest {
    #[schema(example = "Zone Action")]
    pub name: Option<String>,
    #[schema(example = 1)]
    pub zone_command_id: Option<i32>,
    #[schema(example = 1)]
    pub device_id: Option<i32>,
    #[schema(example = 1)]
    pub command_id: Option<i32>,
    #[schema(example = "Action to execute on zone command")]
    pub description: Option<String>,
    #[schema(example = true)]
    pub enable: Option<bool>,
}

//--------------------------------------------------------------------------------- Handlers
#[utoipa::path(
    get,
    path = "/zone_command_action/items",
    tag = "⚡ Zone Command Action",

    params(
        ("limit" = Option<i32>, Query, description = "Maximum number of zone command actions to return"),
        ("offset" = Option<i32>, Query, description = "Number of zone command actions to skip"),
    ),
    responses(
        (status = 200, description = "List of zone command actions retrieved successfully", body = Vec<ZoneCommandActionModel>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn list_zone_command_actions(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<ModelOutput<Vec<ZoneCommandActionModel>>>, StatusCode> {
    let service = ZoneCommandActionService::new();
    let result = service.items(&state.db, params).await;
    Ok(Json(result))
}

#[utoipa::path(
    get,
    path = "/zone_command_action/item/{id}",
    tag = "⚡ Zone Command Action",

    params(
        ("id" = i32, Path, description = "Zone command action ID")
    ),
    responses(
        (status = 200, description = "Zone command action retrieved successfully", body = ZoneCommandActionModel),
        (status = 404, description = "Zone command action not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_zone_command_action(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<ZoneCommandActionModel>>, StatusCode> {
    let service = ZoneCommandActionService::new();
    let result = service.item(&state.db, id).await;
    Ok(Json(result))
}

#[utoipa::path(
    post,
    path = "/zone_command_action/add",
    tag = "⚡ Zone Command Action",

    request_body = CreateZoneCommandActionRequest,
    responses(
        (status = 201, description = "Zone command action created successfully", body = ZoneCommandActionModel),
        (status = 400, description = "Invalid request data"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_zone_command_action(
    State(state): State<AppState>,
    Json(payload): Json<CreateZoneCommandActionRequest>,
) -> Result<Json<ModelOutput<ZoneCommandActionModel>>, StatusCode> {
    let service = ZoneCommandActionService::new();
    let zone_command_action_model = ZoneCommandActionModel {
        id: 0, // Will be auto-generated
        name: payload.name,
        zone_command_id: payload.zone_command_id,
        device_id: payload.device_id,
        command_id: payload.command_id,
        description: payload.description,
        enable: payload.enable,
    };
    
    let result = service.add(&state.db, zone_command_action_model).await;
    Ok(Json(result))
}

#[utoipa::path(
    put,
    path = "/zone_command_action/update/{id}",
    tag = "⚡ Zone Command Action",

    params(
        ("id" = i32, Path, description = "Zone command action ID to update")
    ),
    request_body = UpdateZoneCommandActionRequest,
    responses(
        (status = 200, description = "Zone command action updated successfully", body = ZoneCommandActionModel),
        (status = 404, description = "Zone command action not found"),
        (status = 400, description = "Invalid request data"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_zone_command_action(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateZoneCommandActionRequest>,
) -> Result<Json<ModelOutput<ZoneCommandActionModel>>, StatusCode> {
    let service = ZoneCommandActionService::new();
    
    let zone_command_action_model = ZoneCommandActionModel {
        id,
        name: payload.name.unwrap_or_default(),
        zone_command_id: payload.zone_command_id.unwrap_or_default(),
        device_id: payload.device_id.unwrap_or_default(),
        command_id: payload.command_id,
        description: payload.description.unwrap_or_default(),
        enable: payload.enable.unwrap_or(true),
    };
    
    let result = service.update(&state.db, zone_command_action_model).await;
    Ok(Json(result))
}

#[utoipa::path(
    delete,
    path = "/zone_command_action/delete/{id}",
    tag = "⚡ Zone Command Action",

    params(
        ("id" = i32, Path, description = "Zone command action ID to delete")
    ),
    responses(
        (status = 200, description = "Zone command action deleted successfully"),
        (status = 404, description = "Zone command action not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_zone_command_action(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<String>>, StatusCode> {
    let service = ZoneCommandActionService::new();
    let result = service.delete(&state.db, id).await;
    Ok(Json(result))
}
