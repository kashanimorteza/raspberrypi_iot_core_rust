//--------------------------------------------------------------------------------- Location
// src/api/handlers/zone.rs

//--------------------------------------------------------------------------------- Description
// Axum handlers for Zone CRUD operations

//--------------------------------------------------------------------------------- Import
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use std::collections::HashMap;
use crate::{orm::models::zone::Model as ZoneModel, logics::general::ModelOutput, AppState};
use crate::api::services::zone::ZoneService;

//--------------------------------------------------------------------------------- Request DTOs
#[derive(Deserialize)]
pub struct CreateZoneRequest {
    pub user_id: i32,
    pub name: String,
    pub description: String,
    pub enable: bool,
}

#[derive(Deserialize)]
pub struct UpdateZoneRequest {
    pub user_id: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub enable: Option<bool>,
}

//--------------------------------------------------------------------------------- Handlers
pub async fn list_zones(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<ModelOutput<Vec<ZoneModel>>>, StatusCode> {
    let service = ZoneService::new();
    let result = service.items(&state.db, params).await;
    Ok(Json(result))
}

pub async fn get_zone(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<ZoneModel>>, StatusCode> {
    let service = ZoneService::new();
    let result = service.item(&state.db, id).await;
    Ok(Json(result))
}

pub async fn create_zone(
    State(state): State<AppState>,
    Json(payload): Json<CreateZoneRequest>,
) -> Result<Json<ModelOutput<ZoneModel>>, StatusCode> {
    let service = ZoneService::new();
    let zone_model = ZoneModel {
        id: 0, // Will be auto-generated
        user_id: payload.user_id,
        name: payload.name,
        description: payload.description,
        enable: payload.enable,
    };
    
    let result = service.add(&state.db, zone_model).await;
    Ok(Json(result))
}

pub async fn update_zone(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateZoneRequest>,
) -> Result<Json<ModelOutput<ZoneModel>>, StatusCode> {
    let service = ZoneService::new();
    
    let zone_model = ZoneModel {
        id,
        user_id: payload.user_id.unwrap_or_default(),
        name: payload.name.unwrap_or_default(),
        description: payload.description.unwrap_or_default(),
        enable: payload.enable.unwrap_or(true),
    };
    
    let result = service.update(&state.db, zone_model).await;
    Ok(Json(result))
}

pub async fn delete_zone(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<String>>, StatusCode> {
    let service = ZoneService::new();
    let result = service.delete(&state.db, id).await;
    Ok(Json(result))
}
