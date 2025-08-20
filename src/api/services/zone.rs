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
    pub fn new() -> Self 
    {
        Self 
        {
            logic: ZoneORM::new(true, true),
        }
    }

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

    pub async fn items(&self, db: &DatabaseConnection, filters: HashMap<String, String>) -> ModelOutput<Vec<ZoneModel>> 
    {
        self.logic.items(db, filters).await
    }

    pub async fn item(&self, db: &DatabaseConnection, id: i32) -> ModelOutput<ZoneModel> 
    {
        self.logic.item(db, id).await
    }

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

    pub async fn delete(&self, db: &DatabaseConnection, id: i32) -> ModelOutput<String> 
    {
        self.logic.delete(db, id).await
    }
}
