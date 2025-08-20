//--------------------------------------------------------------------------------- Location
// src/api/routes/timer.rs

//--------------------------------------------------------------------------------- Description
// This is route for timer

//--------------------------------------------------------------------------------- Import
use serde::Deserialize;
use std::collections::HashMap;
use axum::extract::{Path, Query, State};
use axum::routing::{get, post, put, delete};
use axum::{Json, Router};
use crate::AppState;
use crate::orm::models::timer::Model as TimerModel;
use crate::logics::general::ModelOutput;
use crate::api::services::timer::TimerService;

//--------------------------------------------------------------------------------- DTOs
#[derive(Deserialize)]
pub struct AddTimerRequest 
{
    pub user_id: i32,
    pub name: String,
    pub description: String,
    pub enable: bool,
}

#[derive(Deserialize)]
pub struct UpdateTimerRequest 
{
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub description: String,
    pub enable: bool,
}

//--------------------------------------------------------------------------------- Route Handlers
//-------------------------- [Add]
async fn add(State(state): State<AppState>, Json(payload): Json<AddTimerRequest>) -> Json<ModelOutput<TimerModel>> 
{
    let service = TimerService::new();
    
    let timer_model = TimerModel {
        id: 0, // Will be auto-generated
        user_id: payload.user_id,
        name: payload.name,
        description: payload.description,
        enable: payload.enable,
    };
    
    let result = service.add(&state.db, timer_model).await;
    Json(result)
}

//-------------------------- [Items]
async fn items(State(state): State<AppState>, Query(params): Query<HashMap<String, String>>,) -> Json<ModelOutput<Vec<TimerModel>>> 
{
    let service = TimerService::new();
    let result = service.items(&state.db, params).await;
    Json(result)
}

//-------------------------- [Item]
async fn item(State(state): State<AppState>, Path(id): Path<i32>,) -> Json<ModelOutput<TimerModel>> 
{
    let service = TimerService::new();
    let result = service.item(&state.db, id).await;
    Json(result)
}

//-------------------------- [Update]
async fn update(State(state): State<AppState>, Json(payload): Json<UpdateTimerRequest>, ) -> Json<ModelOutput<TimerModel>> 
{
    let service = TimerService::new();
    
    let timer_model = TimerModel 
    {
        id: payload.id,
        user_id: payload.user_id,
        name: payload.name,
        description: payload.description,
        enable: payload.enable,
    };
    
    let result = service.update(&state.db, timer_model).await;
    Json(result)
}

//-------------------------- [Delete]
async fn delete_timer(State(state): State<AppState>, Path(id): Path<i32>,) -> Json<ModelOutput<String>> 
{
    let service = TimerService::new();
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
        .route("/delete/{id}", delete(delete_timer))
}
