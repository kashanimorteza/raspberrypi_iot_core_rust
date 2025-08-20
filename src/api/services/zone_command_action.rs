//--------------------------------------------------------------------------------- Location
// src/api/services/zone_command_action.rs

//--------------------------------------------------------------------------------- Description
// This is service for zone_command_action

//--------------------------------------------------------------------------------- Import
use sea_orm::{DatabaseConnection, Set};
use std::collections::HashMap;
use crate::orm::models::zone_command_action::{Model as ZoneCommandActionModel, ActiveModel as ZoneCommandActionActiveModel};
use crate::logics::general::ModelOutput;
use crate::orm::logics::zone_command_action::ZoneCommandActionORM;

//--------------------------------------------------------------------------------- Service
pub struct ZoneCommandActionService {
    pub logic: ZoneCommandActionORM,
}

impl ZoneCommandActionService {
    pub fn new() -> Self 
    {
        Self 
        {
            logic: ZoneCommandActionORM::new(true, true),
        }
    }

    pub async fn add(&self, db: &DatabaseConnection, item: ZoneCommandActionModel) -> ModelOutput<ZoneCommandActionModel> 
    {
        let active_zone_command_action = ZoneCommandActionActiveModel 
        {
            id: Default::default(),
            name: Set(item.name),
            zone_command_id: Set(item.zone_command_id),
            device_id: Set(item.device_id),
            command_id: Set(item.command_id),
            description: Set(item.description),
            enable: Set(item.enable),
        };

        self.logic.add(db, active_zone_command_action).await
    }

    pub async fn items(&self, db: &DatabaseConnection, filters: HashMap<String, String>) -> ModelOutput<Vec<ZoneCommandActionModel>> 
    {
        self.logic.items(db, filters).await
    }

    pub async fn item(&self, db: &DatabaseConnection, id: i32) -> ModelOutput<ZoneCommandActionModel> 
    {
        self.logic.item(db, id).await
    }

    pub async fn update(&self, db: &DatabaseConnection, item: ZoneCommandActionModel) -> ModelOutput<ZoneCommandActionModel> 
    {
        let active_zone_command_action = ZoneCommandActionActiveModel 
        {
            id: Set(item.id),
            name: Set(item.name),
            zone_command_id: Set(item.zone_command_id),
            device_id: Set(item.device_id),
            command_id: Set(item.command_id),
            description: Set(item.description),
            enable: Set(item.enable),
        };

        self.logic.update(db, active_zone_command_action).await
    }

    pub async fn delete(&self, db: &DatabaseConnection, id: i32) -> ModelOutput<String> 
    {
        self.logic.delete(db, id).await
    }
}
