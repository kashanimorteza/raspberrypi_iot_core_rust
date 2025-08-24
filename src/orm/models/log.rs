//--------------------------------------------------------------------------------- Location
// src/orm/models/log.rs

//--------------------------------------------------------------------------------- Description
// Log model

//--------------------------------------------------------------------------------- Import
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

//--------------------------------------------------------------------------------- Attribute
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, ToSchema)]
#[sea_orm(table_name = "log")]
#[schema(description = "Log model representing system log entries")]

//--------------------------------------------------------------------------------- Model
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "Text")]
    pub date: String,
    #[sea_orm(column_type = "Text")]
    pub name: String,
    pub status: bool,
    #[sea_orm(column_type = "Text")]
    pub data: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
