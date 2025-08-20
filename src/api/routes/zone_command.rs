//--------------------------------------------------------------------------------- Location
// src/api/routes/zone_command.rs

//--------------------------------------------------------------------------------- Description
// This is route for zone_command

//--------------------------------------------------------------------------------- Import
use serde::Deserialize;
use std::collections::HashMap;
use axum::extract::{Path, Query, State};
use axum::routing::{get, post, put, delete};
use axum::{Json, Router};
use crate::AppState;
use crate::orm::models::zone_command::Model as ZoneCommandModel;
use crate::logics::general::ModelOutput;
use crate::api::services::zone_command::ZoneCommandService;

//--------------------------------------------------------------------------------- DTOs
#[derive(Deserialize)]
pub struct AddZoneCommandRequest 
{
    pub zone_id: i32,
    pub name: String,
    pub description: String,
    pub enable: bool,
}

#[derive(Deserialize)]
pub struct UpdateZoneCommandRequest 
{
    pub id: i32,
    pub zone_id: i32,
    pub name: String,
    pub description: String,
    pub enable: bool,
}

//--------------------------------------------------------------------------------- Route Handlers
//-------------------------- [Add]
async fn add(State(state): State<AppState>, Json(payload): Json<AddZoneCommandRequest>) -> Json<ModelOutput<ZoneCommandModel>> 
{
    let service = ZoneCommandService::new();
    
    let zone_command_model = ZoneCommandModel {
        id: 0, // Will be auto-generated
        zone_id: payload.zone_id,
        name: payload.name,
        description: payload.description,
        enable: payload.enable,
    };
    
    let result = service.add(&state.db, zone_command_model).await;
    Json(result)
}

//-------------------------- [Items]
async fn items(State(state): State<AppState>, Query(params): Query<HashMap<String, String>>,) -> Json<ModelOutput<Vec<ZoneCommandModel>>> 
{
    let service = ZoneCommandService::new();
    let result = service.items(&state.db, params).await;
    Json(result)
}

//-------------------------- [Item]
async fn item(State(state): State<AppState>, Path(id): Path<i32>,) -> Json<ModelOutput<ZoneCommandModel>> 
{
    let service = ZoneCommandService::new();
    let result = service.item(&state.db, id).await;
    Json(result)
}

//-------------------------- [Update]
async fn update(State(state): State<AppState>, Json(payload): Json<UpdateZoneCommandRequest>, ) -> Json<ModelOutput<ZoneCommandModel>> 
{
    let service = ZoneCommandService::new();
    
    let zone_command_model = ZoneCommandModel 
    {
        id: payload.id,
        zone_id: payload.zone_id,
        name: payload.name,
        description: payload.description,
        enable: payload.enable,
    };
    
    let result = service.update(&state.db, zone_command_model).await;
    Json(result)
}

//-------------------------- [Delete]
async fn delete_zone_command(State(state): State<AppState>, Path(id): Path<i32>,) -> Json<ModelOutput<String>> 
{
    let service = ZoneCommandService::new();
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
        .route("/delete/{id}", delete(delete_zone_command))
}
