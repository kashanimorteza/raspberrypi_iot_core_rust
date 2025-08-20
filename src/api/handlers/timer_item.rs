//--------------------------------------------------------------------------------- Location
// src/api/handlers/timer_item.rs

//--------------------------------------------------------------------------------- Description
// Axum handlers for TimerItem CRUD operations

//--------------------------------------------------------------------------------- Import
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use std::collections::HashMap;
use crate::{orm::models::timer_item::Model as TimerItemModel, logics::general::ModelOutput, AppState};
use crate::api::services::timer_item::TimerItemService;

//--------------------------------------------------------------------------------- Request DTOs
#[derive(Deserialize)]
pub struct CreateTimerItemRequest {
    pub timer_id: i32,
    pub name: String,
    pub value_from: String,
    pub value_to: String,
    pub description: String,
    pub enable: bool,
}

#[derive(Deserialize)]
pub struct UpdateTimerItemRequest {
    pub timer_id: Option<i32>,
    pub name: Option<String>,
    pub value_from: Option<String>,
    pub value_to: Option<String>,
    pub description: Option<String>,
    pub enable: Option<bool>,
}

//--------------------------------------------------------------------------------- Handlers
pub async fn list_timer_items(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<ModelOutput<Vec<TimerItemModel>>>, StatusCode> {
    let service = TimerItemService::new();
    let result = service.items(&state.db, params).await;
    Ok(Json(result))
}

pub async fn get_timer_item(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<TimerItemModel>>, StatusCode> {
    let service = TimerItemService::new();
    let result = service.item(&state.db, id).await;
    Ok(Json(result))
}

pub async fn create_timer_item(
    State(state): State<AppState>,
    Json(payload): Json<CreateTimerItemRequest>,
) -> Result<Json<ModelOutput<TimerItemModel>>, StatusCode> {
    let service = TimerItemService::new();
    let timer_item_model = TimerItemModel {
        id: 0, // Will be auto-generated
        timer_id: payload.timer_id,
        name: payload.name,
        value_from: payload.value_from,
        value_to: payload.value_to,
        description: payload.description,
        enable: payload.enable,
    };
    
    let result = service.add(&state.db, timer_item_model).await;
    Ok(Json(result))
}

pub async fn update_timer_item(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateTimerItemRequest>,
) -> Result<Json<ModelOutput<TimerItemModel>>, StatusCode> {
    let service = TimerItemService::new();
    
    let timer_item_model = TimerItemModel {
        id,
        timer_id: payload.timer_id.unwrap_or_default(),
        name: payload.name.unwrap_or_default(),
        value_from: payload.value_from.unwrap_or_default(),
        value_to: payload.value_to.unwrap_or_default(),
        description: payload.description.unwrap_or_default(),
        enable: payload.enable.unwrap_or(true),
    };
    
    let result = service.update(&state.db, timer_item_model).await;
    Ok(Json(result))
}

pub async fn delete_timer_item(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<String>>, StatusCode> {
    let service = TimerItemService::new();
    let result = service.delete(&state.db, id).await;
    Ok(Json(result))
}
