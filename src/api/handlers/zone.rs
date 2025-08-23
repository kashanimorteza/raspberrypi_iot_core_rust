//--------------------------------------------------------------------------------- Location
// src/api/handlers/zone.rs

//--------------------------------------------------------------------------------- Description
// Axum handlers for Zone CRUD operations

//--------------------------------------------------------------------------------- Import
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use std::collections::HashMap;
use utoipa::ToSchema;
use crate::{orm::models::zone::Model as ZoneModel, logics::general::ModelOutput, AppState};
use crate::api::services::zone::ZoneService;

//--------------------------------------------------------------------------------- Request DTOs
#[derive(Deserialize, ToSchema)]
#[schema(description = "Request payload for creating a new zone")]
pub struct CreateZoneRequest {
    #[schema(example = 1)]
    pub user_id: i32,
    #[schema(example = "Living Room")]
    pub name: String,
    #[schema(example = "Main living area zone")]
    pub description: String,
    #[schema(example = true)]
    pub enable: bool,
}

#[derive(Deserialize, ToSchema)]
#[schema(description = "Request payload for updating an existing zone")]
pub struct UpdateZoneRequest {
    #[schema(example = 1)]
    pub user_id: Option<i32>,
    #[schema(example = "Living Room")]
    pub name: Option<String>,
    #[schema(example = "Main living area zone")]
    pub description: Option<String>,
    #[schema(example = true)]
    pub enable: Option<bool>,
}

//--------------------------------------------------------------------------------- Handlers
//------------------------- ListZones
#[utoipa::path(
    get,
    path = "/zone/items",
    tag = "🏠 Zone",

    params(
        ("limit" = Option<i32>, Query, description = "Maximum number of zones to return"),
        ("offset" = Option<i32>, Query, description = "Number of zones to skip"),
    ),
    responses(
        (status = 200, description = "List of zones retrieved successfully", body = Vec<ZoneModel>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn list_zones(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<ModelOutput<Vec<ZoneModel>>>, StatusCode> {
    let service = ZoneService::new();
    let result = service.items(&state.db, params).await;
    Ok(Json(result))
}

//------------------------- GetZone
#[utoipa::path(
    get,
    path = "/zone/item/{id}",
    tag = "🏠 Zone",
    params(
        ("id" = i32, Path, description = "Zone ID")
    ),
    responses(
        (status = 200, description = "Zone retrieved successfully", body = ZoneModel),
        (status = 404, description = "Zone not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_zone(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<ZoneModel>>, StatusCode> {
    let service = ZoneService::new();
    let result = service.item(&state.db, id).await;
    Ok(Json(result))
}

//------------------------- EnableZone
#[utoipa::path(
    get,
    path = "/zone/enable/{id}",
    tag = "🏠 Zone",

    params(
        ("id" = i32, Path, description = "Zone ID to enable")
    ),
    responses(
        (status = 200, description = "Zone enabled successfully", body = ZoneModel),
        (status = 404, description = "Zone not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn enable_zone(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<ZoneModel>>, StatusCode> {
    let service = ZoneService::new();
    let result = service.enable(&state.db, id).await;
    Ok(Json(result))
}

//------------------------- DisableZone
#[utoipa::path(
    get,
    path = "/zone/disable/{id}",
    tag = "🏠 Zone",

    params(
        ("id" = i32, Path, description = "Zone ID to disable")
    ),
    responses(
        (status = 200, description = "Zone disabled successfully", body = ZoneModel),
        (status = 404, description = "Zone not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn disable_zone(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<ZoneModel>>, StatusCode> {
    let service = ZoneService::new();
    let result = service.disable(&state.db, id).await;
    Ok(Json(result))
}

//------------------------- StatusZone
#[utoipa::path(
    get,
    path = "/zone/status/{id}",
    tag = "🏠 Zone",
    params(
        ("id" = i32, Path, description = "Zone ID to toggle status")
    ),
    responses(
        (status = 200, description = "Zone status toggled successfully", body = ZoneModel),
        (status = 404, description = "Zone not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn status_zone(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<ZoneModel>>, StatusCode> {
    let service = ZoneService::new();
    let result = service.status(&state.db, id).await;
    Ok(Json(result))
}

//------------------------- UpdateZone
#[utoipa::path(
    put,
    path = "/zone/update/{id}",
    tag = "🏠 Zone",
    params(
        ("id" = i32, Path, description = "Zone ID to update")
    ),
    request_body = UpdateZoneRequest,
    responses(
        (status = 200, description = "Zone updated successfully", body = ZoneModel),
        (status = 404, description = "Zone not found"),
        (status = 400, description = "Invalid request data"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_zone(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateZoneRequest>,
) -> Result<Json<ModelOutput<ZoneModel>>, StatusCode> {
    let service = ZoneService::new();
    
    let zone_model = ZoneModel {
        id,
        user_id: payload.user_id.unwrap_or_default(),
        name: payload.name.unwrap_or_default(),
        description: payload.description.unwrap_or_default(),
        enable: payload.enable.unwrap_or(true),
    };
    
    let result = service.update(&state.db, zone_model).await;
    Ok(Json(result))
}

//------------------------- CreateZone
#[utoipa::path(
    post,
    path = "/zone/add",
    tag = "🏠 Zone",

    request_body = CreateZoneRequest,
    responses(
        (status = 201, description = "Zone created successfully", body = ZoneModel),
        (status = 400, description = "Invalid request data"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_zone(
    State(state): State<AppState>,
    Json(payload): Json<CreateZoneRequest>,
) -> Result<Json<ModelOutput<ZoneModel>>, StatusCode> {
    let service = ZoneService::new();
    let zone_model = ZoneModel {
        id: 0, // Will be auto-generated
        user_id: payload.user_id,
        name: payload.name,
        description: payload.description,
        enable: payload.enable,
    };
    
    let result = service.add(&state.db, zone_model).await;
    Ok(Json(result))
}

//------------------------- DeleteZone
#[utoipa::path(
    delete,
    path = "/zone/delete/{id}",
    tag = "🏠 Zone",

    params(
        ("id" = i32, Path, description = "Zone ID to delete")
    ),
    responses(
        (status = 200, description = "Zone deleted successfully"),
        (status = 404, description = "Zone not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_zone(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModelOutput<String>>, StatusCode> {
    let service = ZoneService::new();
    let result = service.delete(&state.db, id).await;
    Ok(Json(result))
}
