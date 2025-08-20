//--------------------------------------------------------------------------------- Location
// src/orm/logics/timer_device.rs

//--------------------------------------------------------------------------------- Description
// ORM logic for timer_device

//--------------------------------------------------------------------------------- Import
use std::collections::HashMap;
use tracing::{info, error, debug};
use sea_orm::{ActiveModelTrait, DbConn, EntityTrait};
use crate::orm::models::timer_device::{ActiveModel as TimerDeviceActiveModel, Entity as TimerDeviceEntity, Model as TimerDeviceModel};
use crate::logics::general::ModelOutput;

//--------------------------------------------------------------------------------- Class
pub struct TimerDeviceORM 
{
    pub verbose: bool,
    pub log: bool,
    pub this_class: String,
    pub module: String,
}

impl TimerDeviceORM
{
    pub fn new(verbose: bool, log: bool) -> Self 
    {
        Self 
        {
            verbose,
            log,
            this_class: "TimerDeviceORM".to_string(),
            module: "timer_device".to_string(),
        }
    }

    pub async fn add(&self, db: &DbConn, item: TimerDeviceActiveModel) -> ModelOutput<TimerDeviceModel> 
    {
        let this_method = "add";
        if self.verbose { debug!("{}::{} - Starting add operation", self.this_class, this_method); }

        match item.insert(db).await 
        {
            Ok(model) => {
                let output = ModelOutput::success(model, "TimerDevice added successfully".to_string());
                if self.verbose { info!("{}::{} - Success: TimerDevice added", self.this_class, this_method); }
                if self.log { info!("LOG: {}::{} - TimerDevice added", self.this_class, this_method); }
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

    pub async fn items(&self, db: &DbConn, _filters: HashMap<String, String>) -> ModelOutput<Vec<TimerDeviceModel>> 
    {
        let this_method = "items";
        if self.verbose { debug!("{}::{} - Starting items operation", self.this_class, this_method); }

        match TimerDeviceEntity::find().all(db).await 
        {
            Ok(items) => 
            {
                let output = ModelOutput::success(items, "TimerDevices retrieved successfully".to_string());
                if self.verbose { info!("{}::{} - Success: TimerDevices retrieved", self.this_class, this_method); }
                if self.log { info!("LOG: {}::{} - TimerDevices retrieved", self.this_class, this_method); }
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

    pub async fn item(&self, db: &DbConn, id: i32) -> ModelOutput<TimerDeviceModel> 
    {
        let this_method = "item";
        if self.verbose { debug!("{}::{} - Starting item operation for id: {}", self.this_class, this_method, id); }

        match TimerDeviceEntity::find_by_id(id).one(db).await 
        {
            Ok(Some(model)) => 
            {
                let output = ModelOutput::success(model, "TimerDevice retrieved successfully".to_string());
                if self.verbose { info!("{}::{} - Success: TimerDevice {} found", self.this_class, this_method, id); }
                if self.log { info!("LOG: {}::{} - TimerDevice {} retrieved", self.this_class, this_method, id); }
                output
            }
            Ok(None) => 
            {
                let output = ModelOutput::error("TimerDevice not found".to_string());
                if self.verbose { info!("{}::{} - TimerDevice {} not found", self.this_class, this_method, id); }
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

    pub async fn update(&self, db: &DbConn, item: TimerDeviceActiveModel) -> ModelOutput<TimerDeviceModel> 
    {
        let this_method = "update";
        if self.verbose { debug!("{}::{} - Starting update operation", self.this_class, this_method); }

        match item.update(db).await 
        {
            Ok(model) => 
            {
                let output = ModelOutput::success(model, "TimerDevice updated successfully".to_string());
                if self.verbose { info!("{}::{} - Success: TimerDevice updated", self.this_class, this_method); }
                if self.log { info!("LOG: {}::{} - TimerDevice updated", self.this_class, this_method); }
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

        match TimerDeviceEntity::delete_by_id(id).exec(db).await 
        {
            Ok(result) => 
            {
                if result.rows_affected > 0 
                {
                    let output = ModelOutput::success("deleted".to_string(), "TimerDevice deleted successfully".to_string());
                    if self.verbose { info!("{}::{} - Success: TimerDevice {} deleted", self.this_class, this_method, id); }
                    if self.log { info!("LOG: {}::{} - TimerDevice {} deleted", self.this_class, this_method, id); }
                    output
                } 
                else 
                {
                    let output = ModelOutput::error("TimerDevice not found".to_string());
                    if self.verbose { info!("{}::{} - TimerDevice {} not found", self.this_class, this_method, id); }
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
