//--------------------------------------------------------------------------------- Location
// src/api/routes/timer_limit.rs

//--------------------------------------------------------------------------------- Description
// This is route for timer_limit

//--------------------------------------------------------------------------------- Import
use serde::Deserialize;
use std::collections::HashMap;
use axum::extract::{Path, Query, State};
use axum::routing::{get, post, put, delete};
use axum::{Json, Router};
use crate::AppState;
use crate::orm::models::timer_limit::Model as TimerLimitModel;
use crate::logics::general::ModelOutput;
use crate::api::services::timer_limit::TimerLimitService;

//--------------------------------------------------------------------------------- DTOs
#[derive(Deserialize)]
pub struct AddTimerLimitRequest 
{
    pub device_id: i32,
    pub command_from_id: i32,
    pub command_to_id: i32,
    pub value: i32,
    pub description: String,
    pub enable: bool,
}

#[derive(Deserialize)]
pub struct UpdateTimerLimitRequest 
{
    pub id: i32,
    pub device_id: i32,
    pub command_from_id: i32,
    pub command_to_id: i32,
    pub value: i32,
    pub description: String,
    pub enable: bool,
}

//--------------------------------------------------------------------------------- Route Handlers
//-------------------------- [Add]
async fn add(State(state): State<AppState>, Json(payload): Json<AddTimerLimitRequest>) -> Json<ModelOutput<TimerLimitModel>> 
{
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
    Json(result)
}

//-------------------------- [Items]
async fn items(State(state): State<AppState>, Query(params): Query<HashMap<String, String>>,) -> Json<ModelOutput<Vec<TimerLimitModel>>> 
{
    let service = TimerLimitService::new();
    let result = service.items(&state.db, params).await;
    Json(result)
}

//-------------------------- [Item]
async fn item(State(state): State<AppState>, Path(id): Path<i32>,) -> Json<ModelOutput<TimerLimitModel>> 
{
    let service = TimerLimitService::new();
    let result = service.item(&state.db, id).await;
    Json(result)
}

//-------------------------- [Update]
async fn update(State(state): State<AppState>, Json(payload): Json<UpdateTimerLimitRequest>, ) -> Json<ModelOutput<TimerLimitModel>> 
{
    let service = TimerLimitService::new();
    
    let timer_limit_model = TimerLimitModel 
    {
        id: payload.id,
        device_id: payload.device_id,
        command_from_id: payload.command_from_id,
        command_to_id: payload.command_to_id,
        value: payload.value,
        description: payload.description,
        enable: payload.enable,
    };
    
    let result = service.update(&state.db, timer_limit_model).await;
    Json(result)
}

//-------------------------- [Delete]
async fn delete_timer_limit(State(state): State<AppState>, Path(id): Path<i32>,) -> Json<ModelOutput<String>> 
{
    let service = TimerLimitService::new();
    let result = service.delete(&state.db, id).await;
    Json(result)
}

//--------------------------------------------------------------------------------- Router
pub fn router() -> Router<AppState> 
{
    Router::new()
        .route("/add", post(add))
        .route("/items", get(items))
        .route("/{id}", get(item))
        .route("/update", put(update))
        .route("/delete/{id}", delete(delete_timer_limit))
}
