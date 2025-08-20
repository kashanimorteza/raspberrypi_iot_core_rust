//--------------------------------------------------------------------------------- Location
// src/api/routes/zone_command_if.rs

//--------------------------------------------------------------------------------- Description
// This is route for zone_command_if

//--------------------------------------------------------------------------------- Import
use serde::Deserialize;
use std::collections::HashMap;
use axum::extract::{Path, Query, State};
use axum::routing::{get, post, put, delete};
use axum::{Json, Router};
use crate::AppState;
use crate::orm::models::zone_command_if::Model as ZoneCommandIfModel;
use crate::logics::general::ModelOutput;
use crate::api::services::zone_command_if::ZoneCommandIfService;

//--------------------------------------------------------------------------------- DTOs
#[derive(Deserialize)]
pub struct AddZoneCommandIfRequest 
{
    pub name: String,
    pub zone_command_id: i32,
    pub device_id: i32,
    pub command_id: i32,
    pub description: String,
    pub enable: bool,
}

#[derive(Deserialize)]
pub struct UpdateZoneCommandIfRequest 
{
    pub id: i32,
    pub name: String,
    pub zone_command_id: i32,
    pub device_id: i32,
    pub command_id: i32,
    pub description: String,
    pub enable: bool,
}

//--------------------------------------------------------------------------------- Route Handlers
//-------------------------- [Add]
async fn add(State(state): State<AppState>, Json(payload): Json<AddZoneCommandIfRequest>) -> Json<ModelOutput<ZoneCommandIfModel>> 
{
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
    Json(result)
}

//-------------------------- [Items]
async fn items(State(state): State<AppState>, Query(params): Query<HashMap<String, String>>,) -> Json<ModelOutput<Vec<ZoneCommandIfModel>>> 
{
    let service = ZoneCommandIfService::new();
    let result = service.items(&state.db, params).await;
    Json(result)
}

//-------------------------- [Item]
async fn item(State(state): State<AppState>, Path(id): Path<i32>,) -> Json<ModelOutput<ZoneCommandIfModel>> 
{
    let service = ZoneCommandIfService::new();
    let result = service.item(&state.db, id).await;
    Json(result)
}

//-------------------------- [Update]
async fn update(State(state): State<AppState>, Json(payload): Json<UpdateZoneCommandIfRequest>, ) -> Json<ModelOutput<ZoneCommandIfModel>> 
{
    let service = ZoneCommandIfService::new();
    
    let zone_command_if_model = ZoneCommandIfModel 
    {
        id: payload.id,
        name: payload.name,
        zone_command_id: payload.zone_command_id,
        device_id: payload.device_id,
        command_id: payload.command_id,
        description: payload.description,
        enable: payload.enable,
    };
    
    let result = service.update(&state.db, zone_command_if_model).await;
    Json(result)
}

//-------------------------- [Delete]
async fn delete_zone_command_if(State(state): State<AppState>, Path(id): Path<i32>,) -> Json<ModelOutput<String>> 
{
    let service = ZoneCommandIfService::new();
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
        .route("/delete/{id}", delete(delete_zone_command_if))
}
