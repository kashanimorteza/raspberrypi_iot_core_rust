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
use crate::{orm::models::zone_command_if::Model as ZoneCommandIfModel, logics::general::ModelOutput, AppState};
use crate::api::services::zone_command_if::ZoneCommandIfService;

//--------------------------------------------------------------------------------- Request DTOs
#[derive(Deserialize)]
pub struct CreateZoneCommandIfRequest {
    pub name: String,
    pub zone_command_id: i32,
    pub device_id: i32,
    pub command_id: i32,
    pub description: String,
    pub enable: bool,
}

#[derive(Deserialize)]
pub struct UpdateZoneCommandIfRequest {
    pub name: Option<String>,
    pub zone_command_id: Option<i32>,
    pub device_id: Option<i32>,
    pub command_id: Option<i32>,
    pub description: Option<String>,
    pub enable: Option<bool>,
}

//--------------------------------------------------------------------------------- Handlers
pub async fn list_zone_command_ifs(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<ModelOutput<Vec<ZoneCommandIfModel>>>, StatusCode> {
    let service = ZoneCommandIfService::new();
    let result = service.items(&state.db, params).await;
    Ok(Json(result))
}

pub async fn get_zone_command_if(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<ZoneCommandIfModel>>, StatusCode> {
    let service = ZoneCommandIfService::new();
    let result = service.item(&state.db, id).await;
    Ok(Json(result))
}

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

pub async fn delete_zone_command_if(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<String>>, StatusCode> {
    let service = ZoneCommandIfService::new();
    let result = service.delete(&state.db, id).await;
    Ok(Json(result))
}
