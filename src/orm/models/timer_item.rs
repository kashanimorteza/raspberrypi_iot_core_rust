//--------------------------------------------------------------------------------- Location
// src/orm/models/timer_item.rs

//--------------------------------------------------------------------------------- Description
// Timer item model

//--------------------------------------------------------------------------------- Import
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

//--------------------------------------------------------------------------------- Attribute
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, ToSchema)]
#[sea_orm(table_name = "timer_item")]
#[schema(description = "Timer item model representing individual components or actions within a timer")]

//--------------------------------------------------------------------------------- Model
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub timer_id: i32,
    #[sea_orm(column_type = "Text")]
    pub name: String,
    #[sea_orm(column_type = "Text")]
    pub value_from: String,
    #[sea_orm(column_type = "Text")]
    pub value_to: String,
    #[sea_orm(column_type = "Text")]
    pub description: String,
    pub enable: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
