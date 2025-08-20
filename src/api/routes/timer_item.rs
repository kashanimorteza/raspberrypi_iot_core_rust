//--------------------------------------------------------------------------------- Location
// src/api/routes/timer_item.rs

//--------------------------------------------------------------------------------- Description
// This is route for timer_item

//--------------------------------------------------------------------------------- Import
use serde::Deserialize;
use std::collections::HashMap;
use axum::extract::{Path, Query, State};
use axum::routing::{get, post, put, delete};
use axum::{Json, Router};
use crate::AppState;
use crate::orm::models::timer_item::Model as TimerItemModel;
use crate::logics::general::ModelOutput;
use crate::api::services::timer_item::TimerItemService;

//--------------------------------------------------------------------------------- DTOs
#[derive(Deserialize)]
pub struct AddTimerItemRequest 
{
    pub timer_id: i32,
    pub name: String,
    pub value_from: String,
    pub value_to: String,
    pub description: String,
    pub enable: bool,
}

#[derive(Deserialize)]
pub struct UpdateTimerItemRequest 
{
    pub id: i32,
    pub timer_id: i32,
    pub name: String,
    pub value_from: String,
    pub value_to: String,
    pub description: String,
    pub enable: bool,
}

//--------------------------------------------------------------------------------- Route Handlers
//-------------------------- [Add]
async fn add(State(state): State<AppState>, Json(payload): Json<AddTimerItemRequest>) -> Json<ModelOutput<TimerItemModel>> 
{
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
    Json(result)
}

//-------------------------- [Items]
async fn items(State(state): State<AppState>, Query(params): Query<HashMap<String, String>>,) -> Json<ModelOutput<Vec<TimerItemModel>>> 
{
    let service = TimerItemService::new();
    let result = service.items(&state.db, params).await;
    Json(result)
}

//-------------------------- [Item]
async fn item(State(state): State<AppState>, Path(id): Path<i32>,) -> Json<ModelOutput<TimerItemModel>> 
{
    let service = TimerItemService::new();
    let result = service.item(&state.db, id).await;
    Json(result)
}

//-------------------------- [Update]
async fn update(State(state): State<AppState>, Json(payload): Json<UpdateTimerItemRequest>, ) -> Json<ModelOutput<TimerItemModel>> 
{
    let service = TimerItemService::new();
    
    let timer_item_model = TimerItemModel 
    {
        id: payload.id,
        timer_id: payload.timer_id,
        name: payload.name,
        value_from: payload.value_from,
        value_to: payload.value_to,
        description: payload.description,
        enable: payload.enable,
    };
    
    let result = service.update(&state.db, timer_item_model).await;
    Json(result)
}

//-------------------------- [Delete]
async fn delete_timer_item(State(state): State<AppState>, Path(id): Path<i32>,) -> Json<ModelOutput<String>> 
{
    let service = TimerItemService::new();
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
        .route("/delete/{id}", delete(delete_timer_item))
}
