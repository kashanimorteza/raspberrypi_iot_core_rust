//--------------------------------------------------------------------------------- Location
// src/logics/zone.rs

//--------------------------------------------------------------------------------- Description
// CRUD logic methods for the Zone model

//--------------------------------------------------------------------------------- Import
use sea_orm::{ActiveModelTrait, ColumnTrait, DbConn, DbErr, EntityTrait, QueryFilter, Set};
use crate::models::zone::{ActiveModel as ZoneActiveModel, Column as ZoneColumn, Entity as ZoneEntity, Model as ZoneModel};

//--------------------------------------------------------------------------------- Methods
impl ZoneModel 
{
    pub async fn select_all(db: &DbConn) -> Result<Vec<ZoneModel>, DbErr> 
    {
        ZoneEntity::find().all(db).await
    }

    pub async fn select_by_id(db: &DbConn, id: i32) -> Result<Option<ZoneModel>, DbErr> 
    {
        ZoneEntity::find_by_id(id).one(db).await
    }

    pub async fn select_by_user_id(db: &DbConn, user_id: i32) -> Result<Vec<ZoneModel>, DbErr> {
        ZoneEntity::find()
            .filter(ZoneColumn::UserId.eq(user_id))
            .all(db)
            .await
    }

    pub async fn insert(db: &DbConn, user_id: i32, name: String, description: String, enable: bool) -> Result<ZoneModel, DbErr> {
        let new_zone = ZoneActiveModel {
            user_id: Set(user_id),
            name: Set(name),
            description: Set(description),
            enable: Set(enable),
            ..Default::default()
        };

        new_zone.insert(db).await
    }

    pub async fn update(
        db: &DbConn,
        id: i32,
        user_id: Option<i32>,
        name: Option<String>,
        description: Option<String>,
        enable: Option<bool>,
    ) -> Result<ZoneModel, DbErr> {
        let model = ZoneEntity::find_by_id(id).one(db).await?;

        let mut active: ZoneActiveModel = match model {
            Some(m) => m.into(),
            None => return Err(DbErr::RecordNotFound(format!("zone {} not found", id))),
        };

        if let Some(v) = user_id { active.user_id = Set(v); }
        if let Some(v) = name { active.name = Set(v); }
        if let Some(v) = description { active.description = Set(v); }
        if let Some(v) = enable { active.enable = Set(v); }

        active.update(db).await
    }

    pub async fn delete(db: &DbConn, id: i32) -> Result<u64, DbErr> {
        let res = ZoneEntity::delete_by_id(id).exec(db).await?;
        Ok(res.rows_affected)
    }
}