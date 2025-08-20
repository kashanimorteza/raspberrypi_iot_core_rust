//--------------------------------------------------------------------------------- Location
// src/api/services/zone_command.rs

//--------------------------------------------------------------------------------- Description
// This is service for zone_command

//--------------------------------------------------------------------------------- Import
use sea_orm::{DatabaseConnection, Set};
use std::collections::HashMap;
use crate::orm::models::zone_command::{Model as ZoneCommandModel, ActiveModel as ZoneCommandActiveModel};
use crate::logics::general::ModelOutput;
use crate::orm::logics::zone_command::ZoneCommandORM;

//--------------------------------------------------------------------------------- Service
pub struct ZoneCommandService {
    pub logic: ZoneCommandORM,
}

impl ZoneCommandService {
    pub fn new() -> Self 
    {
        Self 
        {
            logic: ZoneCommandORM::new(true, true),
        }
    }

    pub async fn add(&self, db: &DatabaseConnection, item: ZoneCommandModel) -> ModelOutput<ZoneCommandModel> 
    {
        let active_zone_command = ZoneCommandActiveModel 
        {
            id: Default::default(),
            zone_id: Set(item.zone_id),
            name: Set(item.name),
            description: Set(item.description),
            enable: Set(item.enable),
        };

        self.logic.add(db, active_zone_command).await
    }

    pub async fn items(&self, db: &DatabaseConnection, filters: HashMap<String, String>) -> ModelOutput<Vec<ZoneCommandModel>> 
    {
        self.logic.items(db, filters).await
    }

    pub async fn item(&self, db: &DatabaseConnection, id: i32) -> ModelOutput<ZoneCommandModel> 
    {
        self.logic.item(db, id).await
    }

    pub async fn update(&self, db: &DatabaseConnection, item: ZoneCommandModel) -> ModelOutput<ZoneCommandModel> 
    {
        let active_zone_command = ZoneCommandActiveModel 
        {
            id: Set(item.id),
            zone_id: Set(item.zone_id),
            name: Set(item.name),
            description: Set(item.description),
            enable: Set(item.enable),
        };

        self.logic.update(db, active_zone_command).await
    }

    pub async fn delete(&self, db: &DatabaseConnection, id: i32) -> ModelOutput<String> 
    {
        self.logic.delete(db, id).await
    }
}
