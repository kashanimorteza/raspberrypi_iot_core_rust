//--------------------------------------------------------------------------------- Location
// src/api/services/port.rs

//--------------------------------------------------------------------------------- Description
// This is service for port

//--------------------------------------------------------------------------------- Import
use sea_orm::{DatabaseConnection, Set};
use std::collections::HashMap;
use crate::orm::models::port::{Model as PortModel, ActiveModel as PortActiveModel};
use crate::logics::general::ModelOutput;
use crate::orm::logics::port::PortORM;

//--------------------------------------------------------------------------------- Service
pub struct PortService {
    pub logic: PortORM,
}

impl PortService {
    pub fn new() -> Self 
    {
        Self 
        {
            logic: PortORM::new(true, true),
        }
    }

    pub async fn add(&self, db: &DatabaseConnection, item: PortModel) -> ModelOutput<PortModel> 
    {
        let active_port = PortActiveModel 
        {
            id: Default::default(),
            user_id: Set(item.user_id),
            name: Set(item.name),
            pin: Set(item.pin),
            port: Set(item.port),
            value: Set(item.value),
            description: Set(item.description),
            enable: Set(item.enable),
            protocol: Set(item.protocol),
            r#type: Set(item.r#type),
        };

        self.logic.add(db, active_port).await
    }

    pub async fn items(&self, db: &DatabaseConnection, filters: HashMap<String, String>) -> ModelOutput<Vec<PortModel>> 
    {
        self.logic.items(db, filters).await
    }

    pub async fn item(&self, db: &DatabaseConnection, id: i32) -> ModelOutput<PortModel> 
    {
        self.logic.item(db, id).await
    }

    pub async fn update(&self, db: &DatabaseConnection, item: PortModel) -> ModelOutput<PortModel> 
    {
        let active_port = PortActiveModel 
        {
            id: Set(item.id),
            user_id: Set(item.user_id),
            name: Set(item.name),
            pin: Set(item.pin),
            port: Set(item.port),
            value: Set(item.value),
            description: Set(item.description),
            enable: Set(item.enable),
            protocol: Set(item.protocol),
            r#type: Set(item.r#type),
        };

        self.logic.update(db, active_port).await
    }

    pub async fn delete(&self, db: &DatabaseConnection, id: i32) -> ModelOutput<String> 
    {
        self.logic.delete(db, id).await
    }
}
