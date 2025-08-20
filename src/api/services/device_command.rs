//--------------------------------------------------------------------------------- Location
// src/api/services/device_command.rs

//--------------------------------------------------------------------------------- Description
// This is service for device_command

//--------------------------------------------------------------------------------- Import
use sea_orm::{DatabaseConnection, Set};
use std::collections::HashMap;
use crate::orm::models::device_command::{Model as DeviceCommandModel, ActiveModel as DeviceCommandActiveModel};
use crate::logics::general::ModelOutput;
use crate::orm::logics::device_command::DeviceCommandORM;

//--------------------------------------------------------------------------------- Service
pub struct DeviceCommandService {
    pub logic: DeviceCommandORM,
}

impl DeviceCommandService {
    pub fn new() -> Self 
    {
        Self 
        {
            logic: DeviceCommandORM::new(true, true),
        }
    }

    pub async fn add(&self, db: &DatabaseConnection, item: DeviceCommandModel) -> ModelOutput<DeviceCommandModel> 
    {
        let active_device_command = DeviceCommandActiveModel 
        {
            id: Default::default(),
            device_id: Set(item.device_id),
            name: Set(item.name),
            value_from: Set(item.value_from),
            value_to: Set(item.value_to),
            delay: Set(item.delay),
            description: Set(item.description),
            reload: Set(item.reload),
            enable: Set(item.enable),
            r#type: Set(item.r#type),
        };

        self.logic.add(db, active_device_command).await
    }

    pub async fn items(&self, db: &DatabaseConnection, filters: HashMap<String, String>) -> ModelOutput<Vec<DeviceCommandModel>> 
    {
        self.logic.items(db, filters).await
    }

    pub async fn item(&self, db: &DatabaseConnection, id: i32) -> ModelOutput<DeviceCommandModel> 
    {
        self.logic.item(db, id).await
    }

    pub async fn update(&self, db: &DatabaseConnection, item: DeviceCommandModel) -> ModelOutput<DeviceCommandModel> 
    {
        let active_device_command = DeviceCommandActiveModel 
        {
            id: Set(item.id),
            device_id: Set(item.device_id),
            name: Set(item.name),
            value_from: Set(item.value_from),
            value_to: Set(item.value_to),
            delay: Set(item.delay),
            description: Set(item.description),
            reload: Set(item.reload),
            enable: Set(item.enable),
            r#type: Set(item.r#type),
        };

        self.logic.update(db, active_device_command).await
    }

    pub async fn delete(&self, db: &DatabaseConnection, id: i32) -> ModelOutput<String> 
    {
        self.logic.delete(db, id).await
    }
}
