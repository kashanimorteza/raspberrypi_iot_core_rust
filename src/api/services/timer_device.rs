//--------------------------------------------------------------------------------- Location
// src/api/services/timer_device.rs

//--------------------------------------------------------------------------------- Description
// This is service for timer_device

//--------------------------------------------------------------------------------- Import
use sea_orm::{DatabaseConnection, Set};
use std::collections::HashMap;
use crate::orm::models::timer_device::{Model as TimerDeviceModel, ActiveModel as TimerDeviceActiveModel};
use crate::logics::general::ModelOutput;
use crate::orm::logics::timer_device::TimerDeviceORM;

//--------------------------------------------------------------------------------- Service
pub struct TimerDeviceService {
    pub logic: TimerDeviceORM,
}

impl TimerDeviceService {
    pub fn new() -> Self 
    {
        Self 
        {
            logic: TimerDeviceORM::new(true, true),
        }
    }

    pub async fn add(&self, db: &DatabaseConnection, item: TimerDeviceModel) -> ModelOutput<TimerDeviceModel> 
    {
        let active_timer_device = TimerDeviceActiveModel 
        {
            id: Default::default(),
            timer_id: Set(item.timer_id),
            device_id: Set(item.device_id),
            command_id: Set(item.command_id),
            description: Set(item.description),
            enable: Set(item.enable),
        };

        self.logic.add(db, active_timer_device).await
    }

    pub async fn items(&self, db: &DatabaseConnection, filters: HashMap<String, String>) -> ModelOutput<Vec<TimerDeviceModel>> 
    {
        self.logic.items(db, filters).await
    }

    pub async fn item(&self, db: &DatabaseConnection, id: i32) -> ModelOutput<TimerDeviceModel> 
    {
        self.logic.item(db, id).await
    }

    pub async fn update(&self, db: &DatabaseConnection, item: TimerDeviceModel) -> ModelOutput<TimerDeviceModel> 
    {
        let active_timer_device = TimerDeviceActiveModel 
        {
            id: Set(item.id),
            timer_id: Set(item.timer_id),
            device_id: Set(item.device_id),
            command_id: Set(item.command_id),
            description: Set(item.description),
            enable: Set(item.enable),
        };

        self.logic.update(db, active_timer_device).await
    }

    pub async fn delete(&self, db: &DatabaseConnection, id: i32) -> ModelOutput<String> 
    {
        self.logic.delete(db, id).await
    }
}
