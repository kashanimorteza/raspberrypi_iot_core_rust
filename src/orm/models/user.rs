//--------------------------------------------------------------------------------- Location
// src/models/user.rs

//--------------------------------------------------------------------------------- Description
// user model

//--------------------------------------------------------------------------------- Import
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

//--------------------------------------------------------------------------------- Attribute
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, ToSchema)]
#[sea_orm(table_name = "user")]
#[schema(description = "User model representing system users with authentication and contact information")]
//--------------------------------------------------------------------------------- Model
pub struct Model 
{
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "Text", unique)]
    pub name: String,
    #[sea_orm(column_type = "Text")]
    pub username: String,
    #[sea_orm(column_type = "Text")]
    pub password: String,
    #[sea_orm(column_type = "Text")]
    pub key: String,
    #[sea_orm(column_type = "Text")]
    pub email: String,
    #[sea_orm(column_type = "Text")]
    pub phone: String,
    #[sea_orm(column_type = "Text")]
    pub tg_id: String,
    pub enable: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
