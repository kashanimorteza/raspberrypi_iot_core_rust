//--------------------------------------------------------------------------------- Location
// src/api/services/device.rs

//--------------------------------------------------------------------------------- Description
// This is service for device

//--------------------------------------------------------------------------------- Import
use sea_orm::{DatabaseConnection, Set};
use std::collections::HashMap;
use crate::orm::models::device::{Model as DeviceModel, ActiveModel as DeviceActiveModel};
use crate::logics::general::ModelOutput;
use crate::orm::logics::device::DeviceORM;

//--------------------------------------------------------------------------------- Service
pub struct DeviceService {
    pub logic: DeviceORM,
}

impl DeviceService {
    pub fn new() -> Self 
    {
        Self 
        {
            logic: DeviceORM::new(true, true),
        }
    }

    pub async fn add(&self, db: &DatabaseConnection, item: DeviceModel) -> ModelOutput<DeviceModel> 
    {
        let active_device = DeviceActiveModel 
        {
            id: Default::default(),
            zone_id: Set(item.zone_id),
            port_id: Set(item.port_id),
            power_id: Set(item.power_id),
            command_id: Set(item.command_id),
            value: Set(item.value),
            tune: Set(item.tune),
            date: Set(item.date),
            address: Set(item.address),
            name: Set(item.name),
            description: Set(item.description),
            enable: Set(item.enable),
        };

        self.logic.add(db, active_device).await
    }

    pub async fn items(&self, db: &DatabaseConnection, filters: HashMap<String, String>) -> ModelOutput<Vec<DeviceModel>> 
    {
        self.logic.items(db, filters).await
    }

    pub async fn item(&self, db: &DatabaseConnection, id: i32) -> ModelOutput<DeviceModel> 
    {
        self.logic.item(db, id).await
    }

    pub async fn update(&self, db: &DatabaseConnection, item: DeviceModel) -> ModelOutput<DeviceModel> 
    {
        let active_device = DeviceActiveModel 
        {
            id: Set(item.id),
            zone_id: Set(item.zone_id),
            port_id: Set(item.port_id),
            power_id: Set(item.power_id),
            command_id: Set(item.command_id),
            value: Set(item.value),
            tune: Set(item.tune),
            date: Set(item.date),
            address: Set(item.address),
            name: Set(item.name),
            description: Set(item.description),
            enable: Set(item.enable),
        };

        self.logic.update(db, active_device).await
    }

    pub async fn delete(&self, db: &DatabaseConnection, id: i32) -> ModelOutput<String> 
    {
        self.logic.delete(db, id).await
    }
}
