//--------------------------------------------------------------------------------- Location
// src/api/routes/port.rs

//--------------------------------------------------------------------------------- Description
// This is route for port

//--------------------------------------------------------------------------------- Import
use serde::Deserialize;
use std::collections::HashMap;
use axum::extract::{Path, Query, State};
use axum::routing::{get, post, put, delete};
use axum::{Json, Router};
use crate::AppState;
use crate::orm::models::port::Model as PortModel;
use crate::logics::general::ModelOutput;
use crate::api::services::port::PortService;

//--------------------------------------------------------------------------------- DTOs
#[derive(Deserialize)]
pub struct AddPortRequest 
{
    pub user_id: i32,
    pub name: String,
    pub pin: Option<i32>,
    pub port: Option<i32>,
    pub value: Option<i32>,
    pub description: String,
    pub enable: bool,
    pub protocol: String,
    pub r#type: String,
}

#[derive(Deserialize)]
pub struct UpdatePortRequest 
{
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub pin: Option<i32>,
    pub port: Option<i32>,
    pub value: Option<i32>,
    pub description: String,
    pub enable: bool,
    pub protocol: String,
    pub r#type: String,
}

//--------------------------------------------------------------------------------- Route Handlers
//-------------------------- [Add]
async fn add(State(state): State<AppState>, Json(payload): Json<AddPortRequest>) -> Json<ModelOutput<PortModel>> 
{
    let service = PortService::new();
    
    let port_model = PortModel {
        id: 0, // Will be auto-generated
        user_id: payload.user_id,
        name: payload.name,
        pin: payload.pin,
        port: payload.port,
        value: payload.value,
        description: payload.description,
        enable: payload.enable,
        protocol: payload.protocol,
        r#type: payload.r#type,
    };
    
    let result = service.add(&state.db, port_model).await;
    Json(result)
}

//-------------------------- [Items]
async fn items(State(state): State<AppState>, Query(params): Query<HashMap<String, String>>,) -> Json<ModelOutput<Vec<PortModel>>> 
{
    let service = PortService::new();
    let result = service.items(&state.db, params).await;
    Json(result)
}

//-------------------------- [Item]
async fn item(State(state): State<AppState>, Path(id): Path<i32>,) -> Json<ModelOutput<PortModel>> 
{
    let service = PortService::new();
    let result = service.item(&state.db, id).await;
    Json(result)
}

//-------------------------- [Update]
async fn update(State(state): State<AppState>, Json(payload): Json<UpdatePortRequest>, ) -> Json<ModelOutput<PortModel>> 
{
    let service = PortService::new();
    
    let port_model = PortModel 
    {
        id: payload.id,
        user_id: payload.user_id,
        name: payload.name,
        pin: payload.pin,
        port: payload.port,
        value: payload.value,
        description: payload.description,
        enable: payload.enable,
        protocol: payload.protocol,
        r#type: payload.r#type,
    };
    
    let result = service.update(&state.db, port_model).await;
    Json(result)
}

//-------------------------- [Delete]
async fn delete_port(State(state): State<AppState>, Path(id): Path<i32>,) -> Json<ModelOutput<String>> 
{
    let service = PortService::new();
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
        .route("/delete/{id}", delete(delete_port))
}
