//--------------------------------------------------------------------------------- Location
// src/api/handlers/zone_command.rs

//--------------------------------------------------------------------------------- Description
// Axum handlers for ZoneCommand CRUD operations

//--------------------------------------------------------------------------------- Import
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use std::collections::HashMap;
use utoipa::ToSchema;
use crate::{orm::models::zone_command::Model as ZoneCommandModel, logics::general::ModelOutput, AppState};
use crate::api::services::zone_command::ZoneCommandService;

//--------------------------------------------------------------------------------- Request DTOs
#[derive(Deserialize, ToSchema)]
#[schema(description = "Request payload for creating a new zone command")]
pub struct CreateZoneCommandRequest {
    #[schema(example = 1)]
    pub zone_id: i32,
    #[schema(example = "Zone Control Command")]
    pub name: String,
    #[schema(example = "Command to control zone devices")]
    pub description: String,
    #[schema(example = true)]
    pub enable: bool,
}

#[derive(Deserialize, ToSchema)]
#[schema(description = "Request payload for updating an existing zone command")]
pub struct UpdateZoneCommandRequest {
    #[schema(example = 1)]
    pub zone_id: Option<i32>,
    #[schema(example = "Zone Control Command")]
    pub name: Option<String>,
    #[schema(example = "Command to control zone devices")]
    pub description: Option<String>,
    #[schema(example = true)]
    pub enable: Option<bool>,
}

//--------------------------------------------------------------------------------- Handlers
#[utoipa::path(
    get,
    path = "/zone_commands/items",
    tag = "🎯 Zone Commands",

    params(
        ("limit" = Option<i32>, Query, description = "Maximum number of zone commands to return"),
        ("offset" = Option<i32>, Query, description = "Number of zone commands to skip"),
    ),
    responses(
        (status = 200, description = "List of zone commands retrieved successfully", body = Vec<ZoneCommandModel>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn list_zone_commands(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<ModelOutput<Vec<ZoneCommandModel>>>, StatusCode> {
    let service = ZoneCommandService::new();
    let result = service.items(&state.db, params).await;
    Ok(Json(result))
}

#[utoipa::path(
    get,
    path = "/zone_commands/{id}",
    tag = "🎯 Zone Commands",

    params(
        ("id" = i32, Path, description = "Zone command ID")
    ),
    responses(
        (status = 200, description = "Zone command retrieved successfully", body = ZoneCommandModel),
        (status = 404, description = "Zone command not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_zone_command(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<ZoneCommandModel>>, StatusCode> {
    let service = ZoneCommandService::new();
    let result = service.item(&state.db, id).await;
    Ok(Json(result))
}

#[utoipa::path(
    post,
    path = "/zone_commands/add",
    tag = "🎯 Zone Commands",

    request_body = CreateZoneCommandRequest,
    responses(
        (status = 201, description = "Zone command created successfully", body = ZoneCommandModel),
        (status = 400, description = "Invalid request data"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_zone_command(
    State(state): State<AppState>,
    Json(payload): Json<CreateZoneCommandRequest>,
) -> Result<Json<ModelOutput<ZoneCommandModel>>, StatusCode> {
    let service = ZoneCommandService::new();
    let zone_command_model = ZoneCommandModel {
        id: 0, // Will be auto-generated
        zone_id: payload.zone_id,
        name: payload.name,
        description: payload.description,
        enable: payload.enable,
    };
    
    let result = service.add(&state.db, zone_command_model).await;
    Ok(Json(result))
}

#[utoipa::path(
    put,
    path = "/zone_commands/update/{id}",
    tag = "🎯 Zone Commands",

    params(
        ("id" = i32, Path, description = "Zone command ID to update")
    ),
    request_body = UpdateZoneCommandRequest,
    responses(
        (status = 200, description = "Zone command updated successfully", body = ZoneCommandModel),
        (status = 404, description = "Zone command not found"),
        (status = 400, description = "Invalid request data"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_zone_command(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateZoneCommandRequest>,
) -> Result<Json<ModelOutput<ZoneCommandModel>>, StatusCode> {
    let service = ZoneCommandService::new();
    
    let zone_command_model = ZoneCommandModel {
        id,
        zone_id: payload.zone_id.unwrap_or_default(),
        name: payload.name.unwrap_or_default(),
        description: payload.description.unwrap_or_default(),
        enable: payload.enable.unwrap_or(true),
    };
    
    let result = service.update(&state.db, zone_command_model).await;
    Ok(Json(result))
}

#[utoipa::path(
    delete,
    path = "/zone_commands/delete/{id}",
    tag = "🎯 Zone Commands",

    params(
        ("id" = i32, Path, description = "Zone command ID to delete")
    ),
    responses(
        (status = 200, description = "Zone command deleted successfully"),
        (status = 404, description = "Zone command not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_zone_command(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<String>>, StatusCode> {
    let service = ZoneCommandService::new();
    let result = service.delete(&state.db, id).await;
    Ok(Json(result))
}
