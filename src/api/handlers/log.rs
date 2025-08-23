//--------------------------------------------------------------------------------- Location
// src/api/handlers/log.rs

//--------------------------------------------------------------------------------- Description
// Axum handlers for Log CRUD operations

//--------------------------------------------------------------------------------- Import
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use std::collections::HashMap;
use utoipa::ToSchema;
use crate::{orm::models::log::Model as LogModel, logics::general::ModelOutput, AppState};
use crate::api::services::log::LogService;

//--------------------------------------------------------------------------------- Request DTOs
#[derive(Deserialize, ToSchema)]
#[schema(description = "Request payload for creating a new log entry")]
pub struct CreateLogRequest {
    #[schema(example = "2024-01-15")]
    pub date: String,
    #[schema(example = "System startup")]
    pub name: String,
    #[schema(example = true)]
    pub status: bool,
    #[schema(example = "System initialized successfully")]
    pub data: String,
}

#[derive(Deserialize, ToSchema)]
#[schema(description = "Request payload for updating an existing log entry")]
pub struct UpdateLogRequest {
    #[schema(example = "2024-01-15")]
    pub date: Option<String>,
    #[schema(example = "System startup")]
    pub name: Option<String>,
    #[schema(example = true)]
    pub status: Option<bool>,
    #[schema(example = "System initialized successfully")]
    pub data: Option<String>,
}

//--------------------------------------------------------------------------------- Handlers
#[utoipa::path(
    get,
    path = "/logs/items",
    tag = "📋 Logs",

    params(
        ("limit" = Option<i32>, Query, description = "Maximum number of logs to return"),
        ("offset" = Option<i32>, Query, description = "Number of logs to skip"),
    ),
    responses(
        (status = 200, description = "List of logs retrieved successfully", body = Vec<LogModel>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn list_logs(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<ModelOutput<Vec<LogModel>>>, StatusCode> {
    let service = LogService::new();
    let result = service.items(&state.db, params).await;
    Ok(Json(result))
}

#[utoipa::path(
    get,
    path = "/logs/{id}",
    tag = "📋 Logs",

    params(
        ("id" = i32, Path, description = "Log ID")
    ),
    responses(
        (status = 200, description = "Log retrieved successfully", body = LogModel),
        (status = 404, description = "Log not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_log(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<LogModel>>, StatusCode> {
    let service = LogService::new();
    let result = service.item(&state.db, id).await;
    Ok(Json(result))
}

#[utoipa::path(
    post,
    path = "/logs/add",
    tag = "📋 Logs",

    request_body = CreateLogRequest,
    responses(
        (status = 201, description = "Log created successfully", body = LogModel),
        (status = 400, description = "Invalid request data"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_log(
    State(state): State<AppState>,
    Json(payload): Json<CreateLogRequest>,
) -> Result<Json<ModelOutput<LogModel>>, StatusCode> {
    let service = LogService::new();
    let log_model = LogModel {
        id: 0, // Will be auto-generated
        date: payload.date,
        name: payload.name,
        status: payload.status,
        data: payload.data,
    };
    
    let result = service.add(&state.db, log_model).await;
    Ok(Json(result))
}

#[utoipa::path(
    put,
    path = "/logs/update/{id}",
    tag = "📋 Logs",

    params(
        ("id" = i32, Path, description = "Log ID to update")
    ),
    request_body = UpdateLogRequest,
    responses(
        (status = 200, description = "Log updated successfully", body = LogModel),
        (status = 404, description = "Log not found"),
        (status = 400, description = "Invalid request data"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_log(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateLogRequest>,
) -> Result<Json<ModelOutput<LogModel>>, StatusCode> {
    let service = LogService::new();
    
    let log_model = LogModel {
        id,
        date: payload.date.unwrap_or_default(),
        name: payload.name.unwrap_or_default(),
        status: payload.status.unwrap_or(false),
        data: payload.data.unwrap_or_default(),
    };
    
    let result = service.update(&state.db, log_model).await;
    Ok(Json(result))
}

#[utoipa::path(
    delete,
    path = "/logs/delete/{id}",
    tag = "📋 Logs",

    params(
        ("id" = i32, Path, description = "Log ID to delete")
    ),
    responses(
        (status = 200, description = "Log deleted successfully"),
        (status = 404, description = "Log not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_log(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<String>>, StatusCode> {
    let service = LogService::new();
    let result = service.delete(&state.db, id).await;
    Ok(Json(result))
}
