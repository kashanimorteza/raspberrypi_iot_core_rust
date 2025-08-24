//--------------------------------------------------------------------------------- Location
// src/orm/models/timer_limit.rs

//--------------------------------------------------------------------------------- Description
// Timer limit model

//--------------------------------------------------------------------------------- Import
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

//--------------------------------------------------------------------------------- Attribute
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, ToSchema)]
#[sea_orm(table_name = "timer_limit")]
#[schema(description = "Timer limit model defining thresholds and associated commands for devices")]

//--------------------------------------------------------------------------------- Model
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub device_id: i32,
    pub command_from_id: i32,
    pub command_to_id: i32,
    pub value: i32,
    #[sea_orm(column_type = "Text")]
    pub description: String,
    pub enable: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
