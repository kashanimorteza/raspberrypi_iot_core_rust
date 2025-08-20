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
use crate::{orm::models::zone_command::Model as ZoneCommandModel, logics::general::ModelOutput, AppState};
use crate::api::services::zone_command::ZoneCommandService;

//--------------------------------------------------------------------------------- Request DTOs
#[derive(Deserialize)]
pub struct CreateZoneCommandRequest {
    pub zone_id: i32,
    pub name: String,
    pub description: String,
    pub enable: bool,
}

#[derive(Deserialize)]
pub struct UpdateZoneCommandRequest {
    pub zone_id: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub enable: Option<bool>,
}

//--------------------------------------------------------------------------------- Handlers
pub async fn list_zone_commands(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<ModelOutput<Vec<ZoneCommandModel>>>, StatusCode> {
    let service = ZoneCommandService::new();
    let result = service.items(&state.db, params).await;
    Ok(Json(result))
}

pub async fn get_zone_command(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<ZoneCommandModel>>, StatusCode> {
    let service = ZoneCommandService::new();
    let result = service.item(&state.db, id).await;
    Ok(Json(result))
}

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

pub async fn delete_zone_command(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<String>>, StatusCode> {
    let service = ZoneCommandService::new();
    let result = service.delete(&state.db, id).await;
    Ok(Json(result))
}
