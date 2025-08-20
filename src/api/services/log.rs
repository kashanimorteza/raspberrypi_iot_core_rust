//--------------------------------------------------------------------------------- Location
// src/api/services/log.rs

//--------------------------------------------------------------------------------- Description
// This is service for log

//--------------------------------------------------------------------------------- Import
use sea_orm::{DatabaseConnection, Set};
use std::collections::HashMap;
use crate::orm::models::log::{Model as LogModel, ActiveModel as LogActiveModel};
use crate::logics::general::ModelOutput;
use crate::orm::logics::log::LogORM;

//--------------------------------------------------------------------------------- Service
pub struct LogService {
    pub logic: LogORM,
}

impl LogService {
    pub fn new() -> Self 
    {
        Self 
        {
            logic: LogORM::new(true, true),
        }
    }

    pub async fn add(&self, db: &DatabaseConnection, item: LogModel) -> ModelOutput<LogModel> 
    {
        let active_log = LogActiveModel 
        {
            id: Default::default(),
            date: Set(item.date),
            name: Set(item.name),
            status: Set(item.status),
            data: Set(item.data),
        };

        self.logic.add(db, active_log).await
    }

    pub async fn items(&self, db: &DatabaseConnection, filters: HashMap<String, String>) -> ModelOutput<Vec<LogModel>> 
    {
        self.logic.items(db, filters).await
    }

    pub async fn item(&self, db: &DatabaseConnection, id: i32) -> ModelOutput<LogModel> 
    {
        self.logic.item(db, id).await
    }

    pub async fn update(&self, db: &DatabaseConnection, item: LogModel) -> ModelOutput<LogModel> 
    {
        let active_log = LogActiveModel 
        {
            id: Set(item.id),
            date: Set(item.date),
            name: Set(item.name),
            status: Set(item.status),
            data: Set(item.data),
        };

        self.logic.update(db, active_log).await
    }

    pub async fn delete(&self, db: &DatabaseConnection, id: i32) -> ModelOutput<String> 
    {
        self.logic.delete(db, id).await
    }
}
