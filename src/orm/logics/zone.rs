//--------------------------------------------------------------------------------- Location
// src/orm/logics/zone.rs

//--------------------------------------------------------------------------------- Description
// ORM logic for zone

//--------------------------------------------------------------------------------- Import
use std::collections::HashMap;
use tracing::{info, error, debug};
use sea_orm::{ActiveModelTrait, DbConn, EntityTrait};
use crate::orm::models::zone::{ActiveModel as ZoneActiveModel, Entity as ZoneEntity, Model as ZoneModel};
use crate::logics::general::ModelOutput;

//--------------------------------------------------------------------------------- Class
pub struct ZoneORM 
{
    pub verbose: bool,
    pub log: bool,
    pub this_class: String,
    pub module: String,
}

impl ZoneORM
{
    pub fn new(verbose: bool, log: bool) -> Self 
    {
        Self 
        {
            verbose,
            log,
            this_class: "ZoneORM".to_string(),
            module: "zone".to_string(),
        }
    }

    pub async fn add(&self, db: &DbConn, item: ZoneActiveModel) -> ModelOutput<ZoneModel> 
    {
        let this_method = "add";
        if self.verbose { debug!("{}::{} - Starting add operation", self.this_class, this_method); }

        match item.insert(db).await 
        {
            Ok(model) => {
                let output = ModelOutput::success(model, "Zone added successfully".to_string());
                if self.verbose { info!("{}::{} - Success: Zone added", self.this_class, this_method); }
                if self.log { info!("LOG: {}::{} - Zone added", self.this_class, this_method); }
                output
            }
            Err(e) => 
            {
                let error_msg = format!("Database error in {}::{}: {}", self.this_class, this_method, e);
                let output = ModelOutput::error(error_msg.clone());
                error!("{}::{} - Error: {}", self.this_class, this_method, error_msg);
                if self.log { error!("LOG: {}::{} - Error: {}", self.this_class, this_method, error_msg); }
                output
            }
        }
    }

    pub async fn items(&self, db: &DbConn, _filters: HashMap<String, String>) -> ModelOutput<Vec<ZoneModel>> 
    {
        let this_method = "items";
        if self.verbose { debug!("{}::{} - Starting items operation", self.this_class, this_method); }

        match ZoneEntity::find().all(db).await 
        {
            Ok(items) => 
            {
                let output = ModelOutput::success(items, "Zones retrieved successfully".to_string());
                if self.verbose { info!("{}::{} - Success: Zones retrieved", self.this_class, this_method); }
                if self.log { info!("LOG: {}::{} - Zones retrieved", self.this_class, this_method); }
                output
            }
            Err(e) => 
            {
                let error_msg = format!("Database error in {}::{}: {}", self.this_class, this_method, e);
                let output = ModelOutput::error(error_msg.clone());
                error!("{}::{} - Error: {}", self.this_class, this_method, error_msg);
                if self.log { error!("LOG: {}::{} - Error: {}", self.this_class, this_method, error_msg); }
                output
            }
        }
    }

    pub async fn item(&self, db: &DbConn, id: i32) -> ModelOutput<ZoneModel> 
    {
        let this_method = "item";
        if self.verbose { debug!("{}::{} - Starting item operation for id: {}", self.this_class, this_method, id); }

        match ZoneEntity::find_by_id(id).one(db).await 
        {
            Ok(Some(model)) => 
            {
                let output = ModelOutput::success(model, "Zone retrieved successfully".to_string());
                if self.verbose { info!("{}::{} - Success: Zone {} found", self.this_class, this_method, id); }
                if self.log { info!("LOG: {}::{} - Zone {} retrieved", self.this_class, this_method, id); }
                output
            }
            Ok(None) => 
            {
                let output = ModelOutput::error("Zone not found".to_string());
                if self.verbose { info!("{}::{} - Zone {} not found", self.this_class, this_method, id); }
                output
            }
            Err(e) => 
            {
                let error_msg = format!("Database error in {}::{}: {}", self.this_class, this_method, e);
                let output = ModelOutput::error(error_msg.clone());
                error!("{}::{} - Error: {}", self.this_class, this_method, error_msg);
                if self.log { error!("LOG: {}::{} - Error: {}", self.this_class, this_method, error_msg); }
                output
            }
        }
    }

    pub async fn update(&self, db: &DbConn, item: ZoneActiveModel) -> ModelOutput<ZoneModel> 
    {
        let this_method = "update";
        if self.verbose { debug!("{}::{} - Starting update operation", self.this_class, this_method); }

        match item.update(db).await 
        {
            Ok(model) => 
            {
                let output = ModelOutput::success(model, "Zone updated successfully".to_string());
                if self.verbose { info!("{}::{} - Success: Zone updated", self.this_class, this_method); }
                if self.log { info!("LOG: {}::{} - Zone updated", self.this_class, this_method); }
                output
            }
            Err(e) => 
            {
                let error_msg = format!("Database error in {}::{}: {}", self.this_class, this_method, e);
                let output = ModelOutput::error(error_msg.clone());
                error!("{}::{} - Error: {}", self.this_class, this_method, error_msg);
                if self.log { error!("LOG: {}::{} - Error: {}", self.this_class, this_method, error_msg); }
                output
            }
        }
    }

    pub async fn delete(&self, db: &DbConn, id: i32) -> ModelOutput<String> 
    {
        let this_method = "delete";
        if self.verbose { debug!("{}::{} - Starting delete operation for id: {}", self.this_class, this_method, id); }

        match ZoneEntity::delete_by_id(id).exec(db).await 
        {
            Ok(result) => 
            {
                if result.rows_affected > 0 
                {
                    let output = ModelOutput::success("deleted".to_string(), "Zone deleted successfully".to_string());
                    if self.verbose { info!("{}::{} - Success: Zone {} deleted", self.this_class, this_method, id); }
                    if self.log { info!("LOG: {}::{} - Zone {} deleted", self.this_class, this_method, id); }
                    output
                } 
                else 
                {
                    let output = ModelOutput::error("Zone not found".to_string());
                    if self.verbose { info!("{}::{} - Zone {} not found", self.this_class, this_method, id); }
                    output
                }
            }
            Err(e) => 
            {
                let error_msg = format!("Database error in {}::{}: {}", self.this_class, this_method, e);
                let output = ModelOutput::error(error_msg.clone());
                error!("{}::{} - Error: {}", self.this_class, this_method, error_msg);
                if self.log { error!("LOG: {}::{} - Error: {}", self.this_class, this_method, error_msg); }
                output
            }
        }
    }
}
