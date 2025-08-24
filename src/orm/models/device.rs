//--------------------------------------------------------------------------------- Location
// src/orm/models/device.rs

//--------------------------------------------------------------------------------- Description
// Device model

//--------------------------------------------------------------------------------- Import
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

//--------------------------------------------------------------------------------- Attribute
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, ToSchema)]
#[sea_orm(table_name = "device")]
#[schema(description = "IoT device model representing physical devices in the system")]

//--------------------------------------------------------------------------------- Model
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub zone_id: i32,
    pub port_id: i32,
    pub power_id: i32,
    pub command_id: i32,
    pub value: i32,
    pub tune: i32,
    #[sea_orm(column_type = "Text")]
    pub date: String,
    #[sea_orm(column_type = "Text")]
    pub address: String,
    #[sea_orm(column_type = "Text")]
    pub name: String,
    #[sea_orm(column_type = "Text")]
    pub description: String,
    pub enable: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
