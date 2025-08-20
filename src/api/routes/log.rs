//--------------------------------------------------------------------------------- Location
// src/api/routes/log.rs

//--------------------------------------------------------------------------------- Description
// This is route for log

//--------------------------------------------------------------------------------- Import
use serde::Deserialize;
use std::collections::HashMap;
use axum::extract::{Path, Query, State};
use axum::routing::{get, post, put, delete};
use axum::{Json, Router};
use crate::AppState;
use crate::orm::models::log::Model as LogModel;
use crate::logics::general::ModelOutput;
use crate::api::services::log::LogService;

//--------------------------------------------------------------------------------- DTOs
#[derive(Deserialize)]
pub struct AddLogRequest 
{
    pub date: String,
    pub name: String,
    pub status: bool,
    pub data: String,
}

#[derive(Deserialize)]
pub struct UpdateLogRequest 
{
    pub id: i32,
    pub date: String,
    pub name: String,
    pub status: bool,
    pub data: String,
}

//--------------------------------------------------------------------------------- Route Handlers
//-------------------------- [Add]
async fn add(State(state): State<AppState>, Json(payload): Json<AddLogRequest>) -> Json<ModelOutput<LogModel>> 
{
    let service = LogService::new();
    
    let log_model = LogModel {
        id: 0, // Will be auto-generated
        date: payload.date,
        name: payload.name,
        status: payload.status,
        data: payload.data,
    };
    
    let result = service.add(&state.db, log_model).await;
    Json(result)
}

//-------------------------- [Items]
async fn items(State(state): State<AppState>, Query(params): Query<HashMap<String, String>>,) -> Json<ModelOutput<Vec<LogModel>>> 
{
    let service = LogService::new();
    let result = service.items(&state.db, params).await;
    Json(result)
}

//-------------------------- [Item]
async fn item(State(state): State<AppState>, Path(id): Path<i32>,) -> Json<ModelOutput<LogModel>> 
{
    let service = LogService::new();
    let result = service.item(&state.db, id).await;
    Json(result)
}

//-------------------------- [Update]
async fn update(State(state): State<AppState>, Json(payload): Json<UpdateLogRequest>, ) -> Json<ModelOutput<LogModel>> 
{
    let service = LogService::new();
    
    let log_model = LogModel 
    {
        id: payload.id,
        date: payload.date,
        name: payload.name,
        status: payload.status,
        data: payload.data,
    };
    
    let result = service.update(&state.db, log_model).await;
    Json(result)
}

//-------------------------- [Delete]
async fn delete_log(State(state): State<AppState>, Path(id): Path<i32>,) -> Json<ModelOutput<String>> 
{
    let service = LogService::new();
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
        .route("/delete/{id}", delete(delete_log))
}
