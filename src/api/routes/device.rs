//--------------------------------------------------------------------------------- Location
// src/api/routes/device.rs

//--------------------------------------------------------------------------------- Description
// This is route for device

//--------------------------------------------------------------------------------- Import
use serde::Deserialize;
use std::collections::HashMap;
use axum::extract::{Path, Query, State};
use axum::routing::{get, post, put, delete};
use axum::{Json, Router};
use crate::AppState;
use crate::orm::models::device::Model as DeviceModel;
use crate::logics::general::ModelOutput;
use crate::api::services::device::DeviceService;

//--------------------------------------------------------------------------------- DTOs
#[derive(Deserialize)]
pub struct AddDeviceRequest 
{
    pub zone_id: i32,
    pub port_id: i32,
    pub power_id: i32,
    pub command_id: i32,
    pub value: i32,
    pub tune: i32,
    pub date: String,
    pub address: String,
    pub name: String,
    pub description: String,
    pub enable: bool,
}

#[derive(Deserialize)]
pub struct UpdateDeviceRequest 
{
    pub id: i32,
    pub zone_id: i32,
    pub port_id: i32,
    pub power_id: i32,
    pub command_id: i32,
    pub value: i32,
    pub tune: i32,
    pub date: String,
    pub address: String,
    pub name: String,
    pub description: String,
    pub enable: bool,
}

//--------------------------------------------------------------------------------- Route Handlers
//-------------------------- [Add]
async fn add(State(state): State<AppState>, Json(payload): Json<AddDeviceRequest>) -> Json<ModelOutput<DeviceModel>> 
{
    let service = DeviceService::new();
    
    let device_model = DeviceModel {
        id: 0, // Will be auto-generated
        zone_id: payload.zone_id,
        port_id: payload.port_id,
        power_id: payload.power_id,
        command_id: payload.command_id,
        value: payload.value,
        tune: payload.tune,
        date: payload.date,
        address: payload.address,
        name: payload.name,
        description: payload.description,
        enable: payload.enable,
    };
    
    let result = service.add(&state.db, device_model).await;
    Json(result)
}

//-------------------------- [Items]
async fn items(State(state): State<AppState>, Query(params): Query<HashMap<String, String>>,) -> Json<ModelOutput<Vec<DeviceModel>>> 
{
    let service = DeviceService::new();
    let result = service.items(&state.db, params).await;
    Json(result)
}

//-------------------------- [Item]
async fn item(State(state): State<AppState>, Path(id): Path<i32>,) -> Json<ModelOutput<DeviceModel>> 
{
    let service = DeviceService::new();
    let result = service.item(&state.db, id).await;
    Json(result)
}

//-------------------------- [Update]
async fn update(State(state): State<AppState>, Json(payload): Json<UpdateDeviceRequest>, ) -> Json<ModelOutput<DeviceModel>> 
{
    let service = DeviceService::new();
    
    let device_model = DeviceModel 
    {
        id: payload.id,
        zone_id: payload.zone_id,
        port_id: payload.port_id,
        power_id: payload.power_id,
        command_id: payload.command_id,
        value: payload.value,
        tune: payload.tune,
        date: payload.date,
        address: payload.address,
        name: payload.name,
        description: payload.description,
        enable: payload.enable,
    };
    
    let result = service.update(&state.db, device_model).await;
    Json(result)
}

//-------------------------- [Delete]
async fn delete_device(State(state): State<AppState>, Path(id): Path<i32>,) -> Json<ModelOutput<String>> 
{
    let service = DeviceService::new();
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
        .route("/delete/{id}", delete(delete_device))
}
