//--------------------------------------------------------------------------------- Location
// src/api/services/timer.rs

//--------------------------------------------------------------------------------- Description
// This is service for timer

//--------------------------------------------------------------------------------- Import
use sea_orm::{DatabaseConnection, Set};
use std::collections::HashMap;
use crate::orm::models::timer::{Model as TimerModel, ActiveModel as TimerActiveModel};
use crate::logics::general::ModelOutput;
use crate::orm::logics::timer::TimerORM;

//--------------------------------------------------------------------------------- Service
pub struct TimerService {
    pub logic: TimerORM,
}

impl TimerService {
    pub fn new() -> Self 
    {
        Self 
        {
            logic: TimerORM::new(true, true),
        }
    }

    pub async fn add(&self, db: &DatabaseConnection, item: TimerModel) -> ModelOutput<TimerModel> 
    {
        let active_timer = TimerActiveModel 
        {
            id: Default::default(),
            user_id: Set(item.user_id),
            name: Set(item.name),
            description: Set(item.description),
            enable: Set(item.enable),
        };

        self.logic.add(db, active_timer).await
    }

    pub async fn items(&self, db: &DatabaseConnection, filters: HashMap<String, String>) -> ModelOutput<Vec<TimerModel>> 
    {
        self.logic.items(db, filters).await
    }

    pub async fn item(&self, db: &DatabaseConnection, id: i32) -> ModelOutput<TimerModel> 
    {
        self.logic.item(db, id).await
    }

    pub async fn update(&self, db: &DatabaseConnection, item: TimerModel) -> ModelOutput<TimerModel> 
    {
        let active_timer = TimerActiveModel 
        {
            id: Set(item.id),
            user_id: Set(item.user_id),
            name: Set(item.name),
            description: Set(item.description),
            enable: Set(item.enable),
        };

        self.logic.update(db, active_timer).await
    }

    pub async fn delete(&self, db: &DatabaseConnection, id: i32) -> ModelOutput<String> 
    {
        self.logic.delete(db, id).await
    }
}
