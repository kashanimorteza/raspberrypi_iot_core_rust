//--------------------------------------------------------------------------------- Location
// src/api/services/timer_limit.rs

//--------------------------------------------------------------------------------- Description
// This is service for timer_limit

//--------------------------------------------------------------------------------- Import
use sea_orm::{DatabaseConnection, Set};
use std::collections::HashMap;
use crate::orm::models::timer_limit::{Model as TimerLimitModel, ActiveModel as TimerLimitActiveModel};
use crate::logics::general::ModelOutput;
use crate::orm::logics::timer_limit::TimerLimitORM;

//--------------------------------------------------------------------------------- Service
pub struct TimerLimitService {
    pub logic: TimerLimitORM,
}

impl TimerLimitService {
    pub fn new() -> Self 
    {
        Self 
        {
            logic: TimerLimitORM::new(true, true),
        }
    }

    pub async fn add(&self, db: &DatabaseConnection, item: TimerLimitModel) -> ModelOutput<TimerLimitModel> 
    {
        let active_timer_limit = TimerLimitActiveModel 
        {
            id: Default::default(),
            device_id: Set(item.device_id),
            command_from_id: Set(item.command_from_id),
            command_to_id: Set(item.command_to_id),
            value: Set(item.value),
            description: Set(item.description),
            enable: Set(item.enable),
        };

        self.logic.add(db, active_timer_limit).await
    }

    pub async fn items(&self, db: &DatabaseConnection, filters: HashMap<String, String>) -> ModelOutput<Vec<TimerLimitModel>> 
    {
        self.logic.items(db, filters).await
    }

    pub async fn item(&self, db: &DatabaseConnection, id: i32) -> ModelOutput<TimerLimitModel> 
    {
        self.logic.item(db, id).await
    }

    pub async fn update(&self, db: &DatabaseConnection, item: TimerLimitModel) -> ModelOutput<TimerLimitModel> 
    {
        let active_timer_limit = TimerLimitActiveModel 
        {
            id: Set(item.id),
            device_id: Set(item.device_id),
            command_from_id: Set(item.command_from_id),
            command_to_id: Set(item.command_to_id),
            value: Set(item.value),
            description: Set(item.description),
            enable: Set(item.enable),
        };

        self.logic.update(db, active_timer_limit).await
    }

    pub async fn delete(&self, db: &DatabaseConnection, id: i32) -> ModelOutput<String> 
    {
        self.logic.delete(db, id).await
    }
}
