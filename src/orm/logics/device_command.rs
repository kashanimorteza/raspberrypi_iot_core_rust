//--------------------------------------------------------------------------------- Location
// src/orm/logics/device_command.rs

//--------------------------------------------------------------------------------- Description
// ORM logic for device_command

//--------------------------------------------------------------------------------- Import
use std::collections::HashMap;
use tracing::{info, error, debug};
use sea_orm::{ActiveModelTrait, DbConn, EntityTrait};
use crate::orm::models::device_command::{ActiveModel as DeviceCommandActiveModel, Entity as DeviceCommandEntity, Model as DeviceCommandModel};
use crate::logics::general::ModelOutput;

//--------------------------------------------------------------------------------- Class
pub struct DeviceCommandORM 
{
    pub verbose: bool,
    pub log: bool,
    pub this_class: String,
    pub module: String,
}

impl DeviceCommandORM
{
    //------------------------- New
    pub fn new(verbose: bool, log: bool) -> Self 
    {
        Self 
        {
            verbose,
            log,
            this_class: "DeviceCommandORM".to_string(),
            module: "device_command".to_string(),
        }
    }

    //------------------------- Add
    pub async fn add(&self, db: &DbConn, item: DeviceCommandActiveModel) -> ModelOutput<DeviceCommandModel> 
    {
        let this_method = "add";
        if self.verbose { debug!("{}::{} - Starting add operation", self.this_class, this_method); }

        match item.insert(db).await 
        {
            Ok(model) => {
                let output = ModelOutput::success(model, "DeviceCommand added successfully".to_string());
                if self.verbose { info!("{}::{} - Success: DeviceCommand added", self.this_class, this_method); }
                if self.log { info!("LOG: {}::{} - DeviceCommand added", self.this_class, this_method); }
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

    //------------------------- Items
    pub async fn items(&self, db: &DbConn, filters: HashMap<String, String>) -> ModelOutput<Vec<DeviceCommandModel>> 
    {
        let this_method = "items";
        if self.verbose { debug!("{}::{} - Starting items operation with filters: {:?}", self.this_class, this_method, filters); }

        match DeviceCommandEntity::find().all(db).await 
        {
            Ok(items) => 
            {
                let output = ModelOutput::success(items, "DeviceCommands retrieved successfully".to_string());
                if self.verbose { info!("{}::{} - Success: DeviceCommands retrieved", self.this_class, this_method); }
                if self.log { info!("LOG: {}::{} - DeviceCommands retrieved", self.this_class, this_method); }
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

    //------------------------- Item
    pub async fn item(&self, db: &DbConn, id: i32) -> ModelOutput<DeviceCommandModel> 
    {
        let this_method = "item";
        if self.verbose { debug!("{}::{} - Starting item operation for id: {}", self.this_class, this_method, id); }

        match DeviceCommandEntity::find_by_id(id).one(db).await 
        {
            Ok(Some(model)) => 
            {
                let output = ModelOutput::success(model, "DeviceCommand retrieved successfully".to_string());
                if self.verbose { info!("{}::{} - Success: DeviceCommand {} found", self.this_class, this_method, id); }
                if self.log { info!("LOG: {}::{} - DeviceCommand {} retrieved", self.this_class, this_method, id); }
                output
            }
            Ok(None) => 
            {
                let output = ModelOutput::error("DeviceCommand not found".to_string());
                if self.verbose { info!("{}::{} - DeviceCommand {} not found", self.this_class, this_method, id); }
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

    //------------------------- Update
    pub async fn update(&self, db: &DbConn, item: DeviceCommandActiveModel) -> ModelOutput<DeviceCommandModel> 
    {
        let this_method = "update";
        if self.verbose { debug!("{}::{} - Starting update operation", self.this_class, this_method); }

        match item.update(db).await 
        {
            Ok(model) => 
            {
                let output = ModelOutput::success(model, "DeviceCommand updated successfully".to_string());
                if self.verbose { info!("{}::{} - Success: DeviceCommand updated", self.this_class, this_method); }
                if self.log { info!("LOG: {}::{} - DeviceCommand updated", self.this_class, this_method); }
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

    //------------------------- Delete
    pub async fn delete(&self, db: &DbConn, id: i32) -> ModelOutput<String> 
    {
        let this_method = "delete";
        if self.verbose { debug!("{}::{} - Starting delete operation for id: {}", self.this_class, this_method, id); }

        match DeviceCommandEntity::delete_by_id(id).exec(db).await 
        {
            Ok(result) => 
            {
                if result.rows_affected > 0 
                {
                    let output = ModelOutput::success("deleted".to_string(), "DeviceCommand deleted successfully".to_string());
                    if self.verbose { info!("{}::{} - Success: DeviceCommand {} deleted", self.this_class, this_method, id); }
                    if self.log { info!("LOG: {}::{} - DeviceCommand {} deleted", self.this_class, this_method, id); }
                    output
                } 
                else 
                {
                    let output = ModelOutput::error("DeviceCommand not found".to_string());
                    if self.verbose { info!("{}::{} - DeviceCommand {} not found", self.this_class, this_method, id); }
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
