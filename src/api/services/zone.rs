//--------------------------------------------------------------------------------- Location
// src/api/services/zone.rs

//--------------------------------------------------------------------------------- Description
// This is service for zone

//--------------------------------------------------------------------------------- Import
use sea_orm::{DatabaseConnection, Set};
use std::collections::HashMap;
use crate::orm::models::zone::{Model as ZoneModel, ActiveModel as ZoneActiveModel};
use crate::logics::general::ModelOutput;
use crate::orm::logics::zone::ZoneORM;

//--------------------------------------------------------------------------------- Service
pub struct ZoneService {
    pub logic: ZoneORM,
}

impl ZoneService {
    //------------------------- New
    pub fn new() -> Self 
    {
        Self 
        {
            logic: ZoneORM::new(true, true),
        }
    }

    //------------------------- Items
    pub async fn items(&self, db: &DatabaseConnection, filters: HashMap<String, String>) -> ModelOutput<Vec<ZoneModel>> 
    {
        self.logic.items(db, filters).await
    }

    //------------------------- Item
    pub async fn item(&self, db: &DatabaseConnection, id: i32) -> ModelOutput<ZoneModel> 
    {
        self.logic.item(db, id).await
    }

    //------------------------- Enable
    pub async fn enable(&self, db: &DatabaseConnection, id: i32) -> ModelOutput<ZoneModel> 
    {
        self.logic.enable(db, id).await
    }

    //------------------------- Disable
    pub async fn disable(&self, db: &DatabaseConnection, id: i32) -> ModelOutput<ZoneModel> 
    {
        self.logic.disable(db, id).await
    }

    //------------------------- Update
    pub async fn update(&self, db: &DatabaseConnection, item: ZoneModel) -> ModelOutput<ZoneModel> 
    {
        let active_zone = ZoneActiveModel 
        {
            id: Set(item.id),
            user_id: Set(item.user_id),
            name: Set(item.name),
            description: Set(item.description),
            enable: Set(item.enable),
        };

        self.logic.update(db, active_zone).await
    }

    //------------------------- Add
    pub async fn add(&self, db: &DatabaseConnection, item: ZoneModel) -> ModelOutput<ZoneModel> 
    {
        let active_zone = ZoneActiveModel 
        {
            id: Default::default(),
            user_id: Set(item.user_id),
            name: Set(item.name),
            description: Set(item.description),
            enable: Set(item.enable),
        };

        self.logic.add(db, active_zone).await
    }

    //------------------------- Delete
    pub async fn delete(&self, db: &DatabaseConnection, id: i32) -> ModelOutput<String> 
    {
        self.logic.delete(db, id).await
    }
}
