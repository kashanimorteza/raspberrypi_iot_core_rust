//--------------------------------------------------------------------------------- Location
// src/orm/models/port.rs

//--------------------------------------------------------------------------------- Description
// Port model

//--------------------------------------------------------------------------------- Import
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

//--------------------------------------------------------------------------------- Attribute
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, ToSchema)]
#[sea_orm(table_name = "port")]
#[schema(description = "Port model representing communication ports on a device")]

//--------------------------------------------------------------------------------- Model
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub user_id: i32,
    #[sea_orm(column_type = "Text")]
    pub name: String,
    pub pin: i32,
    pub port: i32,
    pub value: i32,
    #[sea_orm(column_type = "Text")]
    pub description: String,
    pub enable: bool,
    #[sea_orm(column_type = "Text")]
    pub protocol: String,
    #[sea_orm(column_type = "Text")]
    pub r#type: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
