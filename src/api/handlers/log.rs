//--------------------------------------------------------------------------------- Location
// src/api/handlers/log.rs

//--------------------------------------------------------------------------------- Description
// Axum handlers for Log CRUD operations

//--------------------------------------------------------------------------------- Import
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use std::collections::HashMap;
use crate::{orm::models::log::Model as LogModel, logics::general::ModelOutput, AppState};
use crate::api::services::log::LogService;

//--------------------------------------------------------------------------------- Request DTOs
#[derive(Deserialize)]
pub struct CreateLogRequest {
    pub date: String,
    pub name: String,
    pub status: bool,
    pub data: String,
}

#[derive(Deserialize)]
pub struct UpdateLogRequest {
    pub date: Option<String>,
    pub name: Option<String>,
    pub status: Option<bool>,
    pub data: Option<String>,
}

//--------------------------------------------------------------------------------- Handlers
pub async fn list_logs(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<ModelOutput<Vec<LogModel>>>, StatusCode> {
    let service = LogService::new();
    let result = service.items(&state.db, params).await;
    Ok(Json(result))
}

pub async fn get_log(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<LogModel>>, StatusCode> {
    let service = LogService::new();
    let result = service.item(&state.db, id).await;
    Ok(Json(result))
}

pub async fn create_log(
    State(state): State<AppState>,
    Json(payload): Json<CreateLogRequest>,
) -> Result<Json<ModelOutput<LogModel>>, StatusCode> {
    let service = LogService::new();
    let log_model = LogModel {
        id: 0, // Will be auto-generated
        date: payload.date,
        name: payload.name,
        status: payload.status,
        data: payload.data,
    };
    
    let result = service.add(&state.db, log_model).await;
    Ok(Json(result))
}

pub async fn update_log(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateLogRequest>,
) -> Result<Json<ModelOutput<LogModel>>, StatusCode> {
    let service = LogService::new();
    
    let log_model = LogModel {
        id,
        date: payload.date.unwrap_or_default(),
        name: payload.name.unwrap_or_default(),
        status: payload.status.unwrap_or(false),
        data: payload.data.unwrap_or_default(),
    };
    
    let result = service.update(&state.db, log_model).await;
    Ok(Json(result))
}

pub async fn delete_log(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<String>>, StatusCode> {
    let service = LogService::new();
    let result = service.delete(&state.db, id).await;
    Ok(Json(result))
}
