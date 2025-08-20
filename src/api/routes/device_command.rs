//--------------------------------------------------------------------------------- Location
// src/api/routes/device_command.rs

//--------------------------------------------------------------------------------- Description
// This is route for device_command

//--------------------------------------------------------------------------------- Import
use serde::Deserialize;
use std::collections::HashMap;
use axum::extract::{Path, Query, State};
use axum::routing::{get, post, put, delete};
use axum::{Json, Router};
use crate::AppState;
use crate::orm::models::device_command::Model as DeviceCommandModel;
use crate::logics::general::ModelOutput;
use crate::api::services::device_command::DeviceCommandService;

//--------------------------------------------------------------------------------- DTOs
#[derive(Deserialize)]
pub struct AddDeviceCommandRequest 
{
    pub device_id: i32,
    pub name: String,
    pub value_from: Option<i32>,
    pub value_to: Option<i32>,
    pub delay: Option<i32>,
    pub description: String,
    pub reload: bool,
    pub enable: bool,
    pub r#type: String,
}

#[derive(Deserialize)]
pub struct UpdateDeviceCommandRequest 
{
    pub id: i32,
    pub device_id: i32,
    pub name: String,
    pub value_from: Option<i32>,
    pub value_to: Option<i32>,
    pub delay: Option<i32>,
    pub description: String,
    pub reload: bool,
    pub enable: bool,
    pub r#type: String,
}

//--------------------------------------------------------------------------------- Route Handlers
//-------------------------- [Add]
async fn add(State(state): State<AppState>, Json(payload): Json<AddDeviceCommandRequest>) -> Json<ModelOutput<DeviceCommandModel>> 
{
    let service = DeviceCommandService::new();
    
    let device_command_model = DeviceCommandModel {
        id: 0, // Will be auto-generated
        device_id: payload.device_id,
        name: payload.name,
        value_from: payload.value_from,
        value_to: payload.value_to,
        delay: payload.delay,
        description: payload.description,
        reload: payload.reload,
        enable: payload.enable,
        r#type: payload.r#type,
    };
    
    let result = service.add(&state.db, device_command_model).await;
    Json(result)
}

//-------------------------- [Items]
async fn items(State(state): State<AppState>, Query(params): Query<HashMap<String, String>>,) -> Json<ModelOutput<Vec<DeviceCommandModel>>> 
{
    let service = DeviceCommandService::new();
    let result = service.items(&state.db, params).await;
    Json(result)
}

//-------------------------- [Item]
async fn item(State(state): State<AppState>, Path(id): Path<i32>,) -> Json<ModelOutput<DeviceCommandModel>> 
{
    let service = DeviceCommandService::new();
    let result = service.item(&state.db, id).await;
    Json(result)
}

//-------------------------- [Update]
async fn update(State(state): State<AppState>, Json(payload): Json<UpdateDeviceCommandRequest>, ) -> Json<ModelOutput<DeviceCommandModel>> 
{
    let service = DeviceCommandService::new();
    
    let device_command_model = DeviceCommandModel 
    {
        id: payload.id,
        device_id: payload.device_id,
        name: payload.name,
        value_from: payload.value_from,
        value_to: payload.value_to,
        delay: payload.delay,
        description: payload.description,
        reload: payload.reload,
        enable: payload.enable,
        r#type: payload.r#type,
    };
    
    let result = service.update(&state.db, device_command_model).await;
    Json(result)
}

//-------------------------- [Delete]
async fn delete_device_command(State(state): State<AppState>, Path(id): Path<i32>,) -> Json<ModelOutput<String>> 
{
    let service = DeviceCommandService::new();
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
        .route("/delete/{id}", delete(delete_device_command))
}
