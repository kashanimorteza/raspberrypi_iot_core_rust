//--------------------------------------------------------------------------------- Location
// src/api/services/timer_item.rs

//--------------------------------------------------------------------------------- Description
// This is service for timer_item

//--------------------------------------------------------------------------------- Import
use sea_orm::{DatabaseConnection, Set};
use std::collections::HashMap;
use crate::orm::models::timer_item::{Model as TimerItemModel, ActiveModel as TimerItemActiveModel};
use crate::logics::general::ModelOutput;
use crate::orm::logics::timer_item::TimerItemORM;

//--------------------------------------------------------------------------------- Service
pub struct TimerItemService {
    pub logic: TimerItemORM,
}

impl TimerItemService {
    //------------------------- New
    pub fn new() -> Self 
    {
        Self 
        {
            logic: TimerItemORM::new(true, true),
        }
    }

    //------------------------- Items
    pub async fn items(&self, db: &DatabaseConnection, filters: HashMap<String, String>) -> ModelOutput<Vec<TimerItemModel>> 
    {
        self.logic.items(db, filters).await
    }

    //------------------------- Item
    pub async fn item(&self, db: &DatabaseConnection, id: i32) -> ModelOutput<TimerItemModel> 
    {
        self.logic.item(db, id).await
    }

    //------------------------- Enable
    pub async fn enable(&self, db: &DatabaseConnection, id: i32) -> ModelOutput<TimerItemModel> 
    {
        self.logic.enable(db, id).await
    }

    //------------------------- Disable
    pub async fn disable(&self, db: &DatabaseConnection, id: i32) -> ModelOutput<TimerItemModel> 
    {
        self.logic.disable(db, id).await
    }

    //------------------------- Update
    pub async fn update(&self, db: &DatabaseConnection, item: TimerItemModel) -> ModelOutput<TimerItemModel> 
    {
        let active_timer_item = TimerItemActiveModel 
        {
            id: Set(item.id),
            timer_id: Set(item.timer_id),
            name: Set(item.name),
            value_from: Set(item.value_from),
            value_to: Set(item.value_to),
            description: Set(item.description),
            enable: Set(item.enable),
        };

        self.logic.update(db, active_timer_item).await
    }

    //------------------------- Add
    pub async fn add(&self, db: &DatabaseConnection, item: TimerItemModel) -> ModelOutput<TimerItemModel> 
    {
        let active_timer_item = TimerItemActiveModel 
        {
            id: Default::default(),
            timer_id: Set(item.timer_id),
            name: Set(item.name),
            value_from: Set(item.value_from),
            value_to: Set(item.value_to),
            description: Set(item.description),
            enable: Set(item.enable),
        };

        self.logic.add(db, active_timer_item).await
    }

    //------------------------- Delete
    pub async fn delete(&self, db: &DatabaseConnection, id: i32) -> ModelOutput<String> 
    {
        self.logic.delete(db, id).await
    }
}
