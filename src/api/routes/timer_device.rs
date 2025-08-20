//--------------------------------------------------------------------------------- Location
// src/api/routes/timer_device.rs

//--------------------------------------------------------------------------------- Description
// This is route for timer_device

//--------------------------------------------------------------------------------- Import
use serde::Deserialize;
use std::collections::HashMap;
use axum::extract::{Path, Query, State};
use axum::routing::{get, post, put, delete};
use axum::{Json, Router};
use crate::AppState;
use crate::orm::models::timer_device::Model as TimerDeviceModel;
use crate::logics::general::ModelOutput;
use crate::api::services::timer_device::TimerDeviceService;

//--------------------------------------------------------------------------------- DTOs
#[derive(Deserialize)]
pub struct AddTimerDeviceRequest 
{
    pub timer_id: i32,
    pub device_id: i32,
    pub command_id: i32,
    pub description: String,
    pub enable: bool,
}

#[derive(Deserialize)]
pub struct UpdateTimerDeviceRequest 
{
    pub id: i32,
    pub timer_id: i32,
    pub device_id: i32,
    pub command_id: i32,
    pub description: String,
    pub enable: bool,
}

//--------------------------------------------------------------------------------- Route Handlers
//-------------------------- [Add]
async fn add(State(state): State<AppState>, Json(payload): Json<AddTimerDeviceRequest>) -> Json<ModelOutput<TimerDeviceModel>> 
{
    let service = TimerDeviceService::new();
    
    let timer_device_model = TimerDeviceModel {
        id: 0, // Will be auto-generated
        timer_id: payload.timer_id,
        device_id: payload.device_id,
        command_id: payload.command_id,
        description: payload.description,
        enable: payload.enable,
    };
    
    let result = service.add(&state.db, timer_device_model).await;
    Json(result)
}

//-------------------------- [Items]
async fn items(State(state): State<AppState>, Query(params): Query<HashMap<String, String>>,) -> Json<ModelOutput<Vec<TimerDeviceModel>>> 
{
    let service = TimerDeviceService::new();
    let result = service.items(&state.db, params).await;
    Json(result)
}

//-------------------------- [Item]
async fn item(State(state): State<AppState>, Path(id): Path<i32>,) -> Json<ModelOutput<TimerDeviceModel>> 
{
    let service = TimerDeviceService::new();
    let result = service.item(&state.db, id).await;
    Json(result)
}

//-------------------------- [Update]
async fn update(State(state): State<AppState>, Json(payload): Json<UpdateTimerDeviceRequest>, ) -> Json<ModelOutput<TimerDeviceModel>> 
{
    let service = TimerDeviceService::new();
    
    let timer_device_model = TimerDeviceModel 
    {
        id: payload.id,
        timer_id: payload.timer_id,
        device_id: payload.device_id,
        command_id: payload.command_id,
        description: payload.description,
        enable: payload.enable,
    };
    
    let result = service.update(&state.db, timer_device_model).await;
    Json(result)
}

//-------------------------- [Delete]
async fn delete_timer_device(State(state): State<AppState>, Path(id): Path<i32>,) -> Json<ModelOutput<String>> 
{
    let service = TimerDeviceService::new();
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
        .route("/delete/{id}", delete(delete_timer_device))
}
