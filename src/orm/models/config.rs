//--------------------------------------------------------------------------------- Location
// src/orm/models/config.rs

//--------------------------------------------------------------------------------- Description
// Config model

//--------------------------------------------------------------------------------- Import
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

//--------------------------------------------------------------------------------- Attribute
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, ToSchema)]
#[sea_orm(table_name = "config")]
#[schema(description = "Config model representing system configuration settings")]

//--------------------------------------------------------------------------------- Model
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "Text")]
    pub name: String,
    #[sea_orm(column_type = "Text")]
    pub time_zone: String,
    #[sea_orm(column_type = "Text")]
    pub path_api: String,
    #[sea_orm(column_type = "Text")]
    pub path_gui: String,
    #[sea_orm(column_type = "Text")]
    pub webapi_title: String,
    #[sea_orm(column_type = "Text")]
    pub webapi_description: String,
    #[sea_orm(column_type = "Text")]
    pub webapi_version: String,
    #[sea_orm(column_type = "Text")]
    pub webapi_openapi_url: String,
    #[sea_orm(column_type = "Text")]
    pub webapi_docs_url: String,
    #[sea_orm(column_type = "Text")]
    pub webapi_redoc_url: String,
    #[sea_orm(column_type = "Text")]
    pub webapi_key: String,
    #[sea_orm(column_type = "Text")]
    pub webapi_host: String,
    pub webapi_port: Option<i32>,
    pub webapi_workers: Option<i32>,
    #[sea_orm(column_type = "Text")]
    pub nginx_api_host: String,
    pub nginx_api_port: Option<i32>,
    #[sea_orm(column_type = "Text")]
    pub nginx_api_key: String,
    #[sea_orm(column_type = "Text")]
    pub nginx_gui_host: String,
    pub nginx_gui_port: Option<i32>,
    #[sea_orm(column_type = "Text")]
    pub nginx_gui_key: String,
    #[sea_orm(column_type = "Text")]
    pub git_email: String,
    #[sea_orm(column_type = "Text")]
    pub git_name: String,
    #[sea_orm(column_type = "Text")]
    pub git_key: String,
    #[sea_orm(column_type = "Text")]
    pub hotspod_ssid: String,
    #[sea_orm(column_type = "Text")]
    pub hotspod_ip: String,
    #[sea_orm(column_type = "Text")]
    pub hotspod_pass: String,
    #[sea_orm(column_type = "Text")]
    pub wifi_ssid: String,
    #[sea_orm(column_type = "Text")]
    pub wifi_ip: String,
    #[sea_orm(column_type = "Text")]
    pub wifi_pass: String,
    pub debug: bool,
    pub log: bool,
    pub verbose: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
