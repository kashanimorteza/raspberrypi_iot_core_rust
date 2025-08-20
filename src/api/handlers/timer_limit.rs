//--------------------------------------------------------------------------------- Location
// src/api/handlers/timer_limit.rs

//--------------------------------------------------------------------------------- Description
// Axum handlers for TimerLimit CRUD operations

//--------------------------------------------------------------------------------- Import
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use std::collections::HashMap;
use crate::{orm::models::timer_limit::Model as TimerLimitModel, logics::general::ModelOutput, AppState};
use crate::api::services::timer_limit::TimerLimitService;

//--------------------------------------------------------------------------------- Request DTOs
#[derive(Deserialize)]
pub struct CreateTimerLimitRequest {
    pub device_id: i32,
    pub command_from_id: i32,
    pub command_to_id: i32,
    pub value: i32,
    pub description: String,
    pub enable: bool,
}

#[derive(Deserialize)]
pub struct UpdateTimerLimitRequest {
    pub device_id: Option<i32>,
    pub command_from_id: Option<i32>,
    pub command_to_id: Option<i32>,
    pub value: Option<i32>,
    pub description: Option<String>,
    pub enable: Option<bool>,
}

//--------------------------------------------------------------------------------- Handlers
pub async fn list_timer_limits(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<ModelOutput<Vec<TimerLimitModel>>>, StatusCode> {
    let service = TimerLimitService::new();
    let result = service.items(&state.db, params).await;
    Ok(Json(result))
}

pub async fn get_timer_limit(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<TimerLimitModel>>, StatusCode> {
    let service = TimerLimitService::new();
    let result = service.item(&state.db, id).await;
    Ok(Json(result))
}

pub async fn create_timer_limit(
    State(state): State<AppState>,
    Json(payload): Json<CreateTimerLimitRequest>,
) -> Result<Json<ModelOutput<TimerLimitModel>>, StatusCode> {
    let service = TimerLimitService::new();
    let timer_limit_model = TimerLimitModel {
        id: 0, // Will be auto-generated
        device_id: payload.device_id,
        command_from_id: payload.command_from_id,
        command_to_id: payload.command_to_id,
        value: payload.value,
        description: payload.description,
        enable: payload.enable,
    };
    
    let result = service.add(&state.db, timer_limit_model).await;
    Ok(Json(result))
}

pub async fn update_timer_limit(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateTimerLimitRequest>,
) -> Result<Json<ModelOutput<TimerLimitModel>>, StatusCode> {
    let service = TimerLimitService::new();
    
    let timer_limit_model = TimerLimitModel {
        id,
        device_id: payload.device_id.unwrap_or_default(),
        command_from_id: payload.command_from_id.unwrap_or_default(),
        command_to_id: payload.command_to_id.unwrap_or_default(),
        value: payload.value.unwrap_or_default(),
        description: payload.description.unwrap_or_default(),
        enable: payload.enable.unwrap_or(true),
    };
    
    let result = service.update(&state.db, timer_limit_model).await;
    Ok(Json(result))
}

pub async fn delete_timer_limit(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<String>>, StatusCode> {
    let service = TimerLimitService::new();
    let result = service.delete(&state.db, id).await;
    Ok(Json(result))
}
