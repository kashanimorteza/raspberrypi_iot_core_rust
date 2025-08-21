//--------------------------------------------------------------------------------- Location
// src/models/user.rs

//--------------------------------------------------------------------------------- Description
// CRUD logic methods for the User model

//--------------------------------------------------------------------------------- Import
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

//--------------------------------------------------------------------------------- Action
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, ToSchema)]
#[sea_orm(table_name = "user")]
#[schema(description = "User model representing system users with authentication and contact information")]
pub struct Model 
{
    #[sea_orm(primary_key)]
    #[schema(example = 1)]
    pub id: i32,
    
    #[sea_orm(column_type = "Text")]
    #[schema(example = "John Doe")]
    pub name: String,
    
    #[sea_orm(column_type = "Text")]
    #[schema(example = "johndoe")]
    pub username: String,
    
    #[sea_orm(column_type = "Text")]
    #[schema(example = "hashed_password_here")]
    pub password: String,
    
    #[sea_orm(column_type = "Text")]
    #[schema(example = "api_key_12345")]
    pub key: String,
    
    #[sea_orm(column_type = "Text")]
    #[schema(example = "john.doe@example.com")]
    pub email: String,
    
    #[sea_orm(column_type = "Text")]
    #[schema(example = "+1234567890")]
    pub phone: String,
    
    #[sea_orm(column_type = "Text")]
    #[schema(example = "telegram_123")]
    pub tg_id: String,
    
    #[schema(example = true)]
    pub enable: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
