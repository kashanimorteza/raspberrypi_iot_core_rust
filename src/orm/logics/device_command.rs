//--------------------------------------------------------------------------------- Location
// src/orm/logics/device_command.rs

//--------------------------------------------------------------------------------- Description
// ORM logic for device_command

//--------------------------------------------------------------------------------- Import
use std::collections::HashMap;
use tracing::{info, error, debug};
use sea_orm::{ActiveModelTrait, DbConn, EntityTrait, QueryOrder};
use crate::orm::models::device_command::{ActiveModel as DeviceCommandActiveModel, Entity as DeviceCommandEntity, Model as DeviceCommandModel, Column as DeviceCommandColumn};
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

        match DeviceCommandEntity::find().order_by_asc(DeviceCommandColumn::Id).all(db).await 
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

    //------------------------- Disable
    pub async fn disable(&self, db: &DbConn, id: i32) -> ModelOutput<DeviceCommandModel>
    {
        let this_method = "disable";
        if self.verbose { debug!("{}::{} - Starting disable operation for id: {}", self.this_class, this_method, id); }

        match DeviceCommandEntity::find_by_id(id).one(db).await
        {
            Ok(Some(existing)) =>
            {
                let mut active: DeviceCommandActiveModel = existing.into();
                active.enable = sea_orm::Set(false);

                match active.update(db).await
                {
                    Ok(updated) => {
                        let output = ModelOutput::success(updated, "Device command disabled successfully".to_string());
                        if self.verbose { info!("{}::{} - Success: Device command {} disabled", self.this_class, this_method, id); }
                        if self.log { info!("LOG: {}::{} - Device command {} disabled", self.this_class, this_method, id); }
                        output
                    },
                    Err(e) => {
                        let error_msg = format!("Database error in {}::{}: {}", self.this_class, this_method, e);
                        let output = ModelOutput::error(error_msg.clone());
                        error!("{}::{} - Error: {}", self.this_class, this_method, error_msg);
                        if self.log { error!("LOG: {}::{} - Error: {}", self.this_class, this_method, error_msg); }
                        output
                    }
                }
            }
            Ok(None) => {
                let output = ModelOutput::error("Device command not found".to_string());
                if self.verbose { info!("{}::{} - Device command {} not found", self.this_class, this_method, id); }
                output
            }
            Err(e) => {
                let error_msg = format!("Database error in {}::{}: {}", self.this_class, this_method, e);
                let output = ModelOutput::error(error_msg.clone());
                error!("{}::{} - Error: {}", self.this_class, this_method, error_msg);
                if self.log { error!("LOG: {}::{} - Error: {}", self.this_class, this_method, error_msg); }
                output
            }
        }
    }

    //------------------------- Enable
    pub async fn enable(&self, db: &DbConn, id: i32) -> ModelOutput<DeviceCommandModel>
    {
        let this_method = "enable";
        if self.verbose { debug!("{}::{} - Starting enable operation for id: {}", self.this_class, this_method, id); }

        match DeviceCommandEntity::find_by_id(id).one(db).await
        {
            Ok(Some(existing)) =>
            {
                let mut active: DeviceCommandActiveModel = existing.into();
                active.enable = sea_orm::Set(true);

                match active.update(db).await
                {
                    Ok(updated) => {
                        let output = ModelOutput::success(updated, "Device command enabled successfully".to_string());
                        if self.verbose { info!("{}::{} - Success: Device command {} enabled", self.this_class, this_method, id); }
                        if self.log { info!("LOG: {}::{} - Device command {} enabled", self.this_class, this_method, id); }
                        output
                    },
                    Err(e) => {
                        let error_msg = format!("Database error in {}::{}: {}", self.this_class, this_method, e);
                        let output = ModelOutput::error(error_msg.clone());
                        error!("{}::{} - Error: {}", self.this_class, this_method, error_msg);
                        if self.log { error!("LOG: {}::{} - Error: {}", self.this_class, this_method, error_msg); }
                        output
                    }
                }
            }
            Ok(None) => {
                let output = ModelOutput::error("Device command not found".to_string());
                if self.verbose { info!("{}::{} - Device command {} not found", self.this_class, this_method, id); }
                output
            }
            Err(e) => {
                let error_msg = format!("Database error in {}::{}: {}", self.this_class, this_method, e);
                let output = ModelOutput::error(error_msg.clone());
                error!("{}::{} - Error: {}", self.this_class, this_method, error_msg);
                if self.log { error!("LOG: {}::{} - Error: {}", self.this_class, this_method, error_msg); }
                output
            }
        }
    }

    //------------------------- Status (Toggle Enable)
    pub async fn status(&self, db: &DbConn, id: i32) -> ModelOutput<DeviceCommandModel>
    {
        let this_method = "status";

        match DeviceCommandEntity::find_by_id(id).one(db).await
        {
            Ok(Some(existing)) =>
            {
                // Get the current enable value before moving existing
                let current_enable = existing.enable;
                let mut active: DeviceCommandActiveModel = existing.into();
                // Toggle the enable field: if true, set to false; if false, set to true
                active.enable = sea_orm::Set(!current_enable);

                match active.update(db).await
                {
                    Ok(updated) => {
                        let message = if current_enable {
                            "Device command disabled successfully".to_string()
                        } else {
                            "Device command enabled successfully".to_string()
                        };
                        ModelOutput::success(updated, message)
                    },
                    Err(e) => {
                        let error_msg = format!("Database error in {}::{}: {}", self.this_class, this_method, e);
                        error!("{}::{} - Error: {}", self.this_class, this_method, error_msg);
                        ModelOutput::error(error_msg)
                    }
                }
            }
            Ok(None) => ModelOutput::error("Device command not found".to_string()),
            Err(e) => {
                let error_msg = format!("Database error in {}::{}: {}", self.this_class, this_method, e);
                error!("{}::{} - Error: {}", self.this_class, this_method, error_msg);
                ModelOutput::error(error_msg)
            }
        }
    }
}
