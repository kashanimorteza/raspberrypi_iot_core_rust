//--------------------------------------------------------------------------------- Location
// src/orm/models/zone_command_if.rs

//--------------------------------------------------------------------------------- Description
// Zone command if model

//--------------------------------------------------------------------------------- Import
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

//--------------------------------------------------------------------------------- Attribute
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, ToSchema)]
#[sea_orm(table_name = "zone_command_if")]
#[schema(description = "Zone command condition model for defining conditions under which a command executes")]

//--------------------------------------------------------------------------------- Model
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "Text")]
    pub name: String,
    pub zone_command_id: i32,
    pub device_id: i32,
    pub command_id: i32,
    #[sea_orm(column_type = "Text")]
    pub description: String,
    pub enable: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
