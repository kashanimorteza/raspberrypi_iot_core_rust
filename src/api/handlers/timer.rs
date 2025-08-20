//--------------------------------------------------------------------------------- Location
// src/api/handlers/timer.rs

//--------------------------------------------------------------------------------- Description
// Axum handlers for Timer CRUD operations

//--------------------------------------------------------------------------------- Import
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use std::collections::HashMap;
use crate::{orm::models::timer::Model as TimerModel, logics::general::ModelOutput, AppState};
use crate::api::services::timer::TimerService;

//--------------------------------------------------------------------------------- Request DTOs
#[derive(Deserialize)]
pub struct CreateTimerRequest {
    pub user_id: i32,
    pub name: String,
    pub description: String,
    pub enable: bool,
}

#[derive(Deserialize)]
pub struct UpdateTimerRequest {
    pub user_id: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub enable: Option<bool>,
}

//--------------------------------------------------------------------------------- Handlers
pub async fn list_timers(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<ModelOutput<Vec<TimerModel>>>, StatusCode> {
    let service = TimerService::new();
    let result = service.items(&state.db, params).await;
    Ok(Json(result))
}

pub async fn get_timer(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<TimerModel>>, StatusCode> {
    let service = TimerService::new();
    let result = service.item(&state.db, id).await;
    Ok(Json(result))
}

pub async fn create_timer(
    State(state): State<AppState>,
    Json(payload): Json<CreateTimerRequest>,
) -> Result<Json<ModelOutput<TimerModel>>, StatusCode> {
    let service = TimerService::new();
    let timer_model = TimerModel {
        id: 0, // Will be auto-generated
        user_id: payload.user_id,
        name: payload.name,
        description: payload.description,
        enable: payload.enable,
    };
    
    let result = service.add(&state.db, timer_model).await;
    Ok(Json(result))
}

pub async fn update_timer(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateTimerRequest>,
) -> Result<Json<ModelOutput<TimerModel>>, StatusCode> {
    let service = TimerService::new();
    
    let timer_model = TimerModel {
        id,
        user_id: payload.user_id.unwrap_or_default(),
        name: payload.name.unwrap_or_default(),
        description: payload.description.unwrap_or_default(),
        enable: payload.enable.unwrap_or(true),
    };
    
    let result = service.update(&state.db, timer_model).await;
    Ok(Json(result))
}

pub async fn delete_timer(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<String>>, StatusCode> {
    let service = TimerService::new();
    let result = service.delete(&state.db, id).await;
    Ok(Json(result))
}
