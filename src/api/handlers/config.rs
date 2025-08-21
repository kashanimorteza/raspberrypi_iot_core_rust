//--------------------------------------------------------------------------------- Location
// src/api/handlers/config.rs

//--------------------------------------------------------------------------------- Description
// Axum handlers for Config CRUD operations

//--------------------------------------------------------------------------------- Import
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use utoipa::ToSchema;
use std::collections::HashMap;
use crate::{orm::models::config::Model as ConfigModel, logics::general::ModelOutput, AppState};
use crate::api::services::config::ConfigService;

//--------------------------------------------------------------------------------- Request DTOs
#[derive(Deserialize, ToSchema)]
#[schema(description = "Request payload for creating a new configuration")]
pub struct CreateConfigRequest {
    #[schema(example = "Production Config")]
    pub name: String,
    #[schema(example = "UTC")]
    pub time_zone: String,
    #[schema(example = "/api/v1")]
    pub path_api: String,
    #[schema(example = "/dashboard")]
    pub path_gui: String,
    #[schema(example = "IoT Core API")]
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

#[derive(Deserialize, ToSchema)]
#[schema(description = "Request payload for updating an existing configuration")]
pub struct UpdateConfigRequest {
    #[schema(example = "Production Config")]
    pub name: Option<String>,
    pub time_zone: Option<String>,
    pub path_api: Option<String>,
    pub path_gui: Option<String>,
    pub webapi_title: Option<String>,
    pub webapi_description: Option<String>,
    pub webapi_version: Option<String>,
    pub webapi_openapi_url: Option<String>,
    pub webapi_docs_url: Option<String>,
    pub webapi_redoc_url: Option<String>,
    pub webapi_key: Option<String>,
    pub webapi_host: Option<String>,
    pub webapi_port: Option<i32>,
    pub webapi_workers: Option<i32>,
    pub nginx_api_host: Option<String>,
    pub nginx_api_port: Option<i32>,
    pub nginx_api_key: Option<String>,
    pub nginx_gui_host: Option<String>,
    pub nginx_gui_port: Option<i32>,
    pub nginx_gui_key: Option<String>,
    pub git_email: Option<String>,
    pub git_name: Option<String>,
    pub git_key: Option<String>,
    pub hotspod_ssid: Option<String>,
    pub hotspod_ip: Option<String>,
    pub hotspod_pass: Option<String>,
    pub wifi_ssid: Option<String>,
    pub wifi_ip: Option<String>,
    pub wifi_pass: Option<String>,
    pub debug: Option<bool>,
    pub log: Option<bool>,
    pub verbose: Option<bool>,
}

//--------------------------------------------------------------------------------- Handlers
#[utoipa::path(
    get,
    path = "/config/items",
    tag = "⚙️ Config",
    summary = "List all configurations",
    description = "Retrieve a list of all system configurations with optional query parameters for filtering",
    params(
        ("limit" = Option<i32>, Query, description = "Maximum number of configs to return"),
        ("offset" = Option<i32>, Query, description = "Number of configs to skip"),
    ),
    responses(
        (status = 200, description = "List of configurations retrieved successfully", body = Vec<ConfigModel>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn list_configs(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<ModelOutput<Vec<ConfigModel>>>, StatusCode> {
    let service = ConfigService::new();
    let result = service.items(&state.db, params).await;
    Ok(Json(result))
}

#[utoipa::path(
    get,
    path = "/config/item/{id}",
    tag = "⚙️ Config",
    summary = "Get configuration by ID",
    description = "Retrieve a specific configuration by its unique identifier",
    params(
        ("id" = i32, Path, description = "Configuration ID")
    ),
    responses(
        (status = 200, description = "Configuration retrieved successfully", body = ConfigModel),
        (status = 404, description = "Configuration not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_config(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<ConfigModel>>, StatusCode> {
    let service = ConfigService::new();
    let result = service.item(&state.db, id).await;
    Ok(Json(result))
}

#[utoipa::path(
    post,
    path = "/config/add",
    tag = "⚙️ Config",
    summary = "Create new configuration",
    description = "Create a new system configuration with the provided details",
    request_body = CreateConfigRequest,
    responses(
        (status = 201, description = "Configuration created successfully", body = ConfigModel),
        (status = 400, description = "Invalid request payload"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_config(
    State(state): State<AppState>,
    Json(payload): Json<CreateConfigRequest>,
) -> Result<Json<ModelOutput<ConfigModel>>, StatusCode> {
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
    Ok(Json(result))
}

#[utoipa::path(
    put,
    path = "/config/update/{id}",
    tag = "⚙️ Config",
    summary = "Update configuration",
    description = "Update an existing configuration with the provided details",
    params(
        ("id" = i32, Path, description = "Configuration ID to update")
    ),
    request_body = UpdateConfigRequest,
    responses(
        (status = 200, description = "Configuration updated successfully", body = ConfigModel),
        (status = 400, description = "Invalid request payload"),
        (status = 404, description = "Configuration not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_config(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateConfigRequest>,
) -> Result<Json<ModelOutput<ConfigModel>>, StatusCode> {
    let service = ConfigService::new();
    
    let config_model = ConfigModel {
        id,
        name: payload.name.unwrap_or_default(),
        time_zone: payload.time_zone.unwrap_or_default(),
        path_api: payload.path_api.unwrap_or_default(),
        path_gui: payload.path_gui.unwrap_or_default(),
        webapi_title: payload.webapi_title.unwrap_or_default(),
        webapi_description: payload.webapi_description.unwrap_or_default(),
        webapi_version: payload.webapi_version.unwrap_or_default(),
        webapi_openapi_url: payload.webapi_openapi_url.unwrap_or_default(),
        webapi_docs_url: payload.webapi_docs_url.unwrap_or_default(),
        webapi_redoc_url: payload.webapi_redoc_url.unwrap_or_default(),
        webapi_key: payload.webapi_key.unwrap_or_default(),
        webapi_host: payload.webapi_host.unwrap_or_default(),
        webapi_port: payload.webapi_port,
        webapi_workers: payload.webapi_workers,
        nginx_api_host: payload.nginx_api_host.unwrap_or_default(),
        nginx_api_port: payload.nginx_api_port,
        nginx_api_key: payload.nginx_api_key.unwrap_or_default(),
        nginx_gui_host: payload.nginx_gui_host.unwrap_or_default(),
        nginx_gui_port: payload.nginx_gui_port,
        nginx_gui_key: payload.nginx_gui_key.unwrap_or_default(),
        git_email: payload.git_email.unwrap_or_default(),
        git_name: payload.git_name.unwrap_or_default(),
        git_key: payload.git_key.unwrap_or_default(),
        hotspod_ssid: payload.hotspod_ssid.unwrap_or_default(),
        hotspod_ip: payload.hotspod_ip.unwrap_or_default(),
        hotspod_pass: payload.hotspod_pass.unwrap_or_default(),
        wifi_ssid: payload.wifi_ssid.unwrap_or_default(),
        wifi_ip: payload.wifi_ip.unwrap_or_default(),
        wifi_pass: payload.wifi_pass.unwrap_or_default(),
        debug: payload.debug.unwrap_or(false),
        log: payload.log.unwrap_or(false),
        verbose: payload.verbose.unwrap_or(false),
    };
    
    let result = service.update(&state.db, config_model).await;
    Ok(Json(result))
}

#[utoipa::path(
    delete,
    path = "/config/delete/{id}",
    tag = "⚙️ Config",
    summary = "Delete configuration",
    description = "Delete a configuration by its unique identifier",
    params(
        ("id" = i32, Path, description = "Configuration ID to delete")
    ),
    responses(
        (status = 200, description = "Configuration deleted successfully"),
        (status = 404, description = "Configuration not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_config(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<String>>, StatusCode> {
    let service = ConfigService::new();
    let result = service.delete(&state.db, id).await;
    Ok(Json(result))
}
