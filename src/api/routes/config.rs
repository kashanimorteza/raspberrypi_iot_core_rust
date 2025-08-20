//--------------------------------------------------------------------------------- Location
// src/api/routes/config.rs

//--------------------------------------------------------------------------------- Description
// This is route for config

//--------------------------------------------------------------------------------- Import
use serde::Deserialize;
use std::collections::HashMap;
use axum::extract::{Path, Query, State};
use axum::routing::{get, post, delete};
use axum::{Json, Router};
use crate::AppState;
use crate::orm::models::config::Model as ConfigModel;
use crate::logics::general::ModelOutput;
use crate::api::services::config::ConfigService;

//--------------------------------------------------------------------------------- DTOs
#[derive(Deserialize)]
pub struct AddConfigRequest 
{
    pub name: String,
    pub time_zone: String,
    pub path_api: String,
    pub path_gui: String,
    pub webapi_title: String,
    pub webapi_description: String,
    pub webapi_version: String,
    pub webapi_openapi_url: String,
    pub webapi_docs_url: String,
    pub webapi_redoc_url: String,
    pub webapi_key: String,
    pub webapi_host: String,
    pub webapi_port: Option<i32>,
    pub webapi_workers: Option<i32>,
    pub nginx_api_host: String,
    pub nginx_api_port: Option<i32>,
    pub nginx_api_key: String,
    pub nginx_gui_host: String,
    pub nginx_gui_port: Option<i32>,
    pub nginx_gui_key: String,
    pub git_email: String,
    pub git_name: String,
    pub git_key: String,
    pub hotspod_ssid: String,
    pub hotspod_ip: String,
    pub hotspod_pass: String,
    pub wifi_ssid: String,
    pub wifi_ip: String,
    pub wifi_pass: String,
    pub debug: bool,
    pub log: bool,
    pub verbose: bool,
}

//--------------------------------------------------------------------------------- Route Handlers
//-------------------------- [Add]
async fn add(State(state): State<AppState>, Json(payload): Json<AddConfigRequest>) -> Json<ModelOutput<ConfigModel>> 
{
    let service = ConfigService::new();
    
    let config_model = ConfigModel {
        id: 0, // Will be auto-generated
        name: payload.name,
        time_zone: payload.time_zone,
        path_api: payload.path_api,
        path_gui: payload.path_gui,
        webapi_title: payload.webapi_title,
        webapi_description: payload.webapi_description,
        webapi_version: payload.webapi_version,
        webapi_openapi_url: payload.webapi_openapi_url,
        webapi_docs_url: payload.webapi_docs_url,
        webapi_redoc_url: payload.webapi_redoc_url,
        webapi_key: payload.webapi_key,
        webapi_host: payload.webapi_host,
        webapi_port: payload.webapi_port,
        webapi_workers: payload.webapi_workers,
        nginx_api_host: payload.nginx_api_host,
        nginx_api_port: payload.nginx_api_port,
        nginx_api_key: payload.nginx_api_key,
        nginx_gui_host: payload.nginx_gui_host,
        nginx_gui_port: payload.nginx_gui_port,
        nginx_gui_key: payload.nginx_gui_key,
        git_email: payload.git_email,
        git_name: payload.git_name,
        git_key: payload.git_key,
        hotspod_ssid: payload.hotspod_ssid,
        hotspod_ip: payload.hotspod_ip,
        hotspod_pass: payload.hotspod_pass,
        wifi_ssid: payload.wifi_ssid,
        wifi_ip: payload.wifi_ip,
        wifi_pass: payload.wifi_pass,
        debug: payload.debug,
        log: payload.log,
        verbose: payload.verbose,
    };
    
    let result = service.add(&state.db, config_model).await;
    Json(result)
}

//-------------------------- [Items]
async fn items(State(state): State<AppState>, Query(params): Query<HashMap<String, String>>,) -> Json<ModelOutput<Vec<ConfigModel>>> 
{
    let service = ConfigService::new();
    let result = service.items(&state.db, params).await;
    Json(result)
}

//-------------------------- [Item]
async fn item(State(state): State<AppState>, Path(id): Path<i32>,) -> Json<ModelOutput<ConfigModel>> 
{
    let service = ConfigService::new();
    let result = service.item(&state.db, id).await;
    Json(result)
}

//-------------------------- [Delete]
async fn delete_config(State(state): State<AppState>, Path(id): Path<i32>,) -> Json<ModelOutput<String>> 
{
    let service = ConfigService::new();
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
        .route("/delete/{id}", delete(delete_config))
}
