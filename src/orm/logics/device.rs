//--------------------------------------------------------------------------------- Location
// src/orm/logics/device.rs

//--------------------------------------------------------------------------------- Description
// ORM logic for device

//--------------------------------------------------------------------------------- Import
use std::collections::HashMap;
use tracing::{info, error, debug};
use sea_orm::{ActiveModelTrait, DbConn, EntityTrait};
use crate::orm::models::device::{ActiveModel as DeviceActiveModel, Entity as DeviceEntity, Model as DeviceModel};
use crate::logics::general::ModelOutput;

//--------------------------------------------------------------------------------- Class
pub struct DeviceORM 
{
    pub verbose: bool,
    pub log: bool,
    pub this_class: String,
    pub module: String,
}

impl DeviceORM
{
    //------------------------- New
    pub fn new(verbose: bool, log: bool) -> Self 
    {
        Self 
        {
            verbose,
            log,
            this_class: "DeviceORM".to_string(),
            module: "device".to_string(),
        }
    }

    //------------------------- Add
    pub async fn add(&self, db: &DbConn, item: DeviceActiveModel) -> ModelOutput<DeviceModel> 
    {
        let this_method = "add";
        if self.verbose { debug!("{}::{} - Starting add operation", self.this_class, this_method); }

        match item.insert(db).await 
        {
            Ok(model) => {
                let output = ModelOutput::success(model, "Device added successfully".to_string());
                if self.verbose { info!("{}::{} - Success: Device added", self.this_class, this_method); }
                if self.log { info!("LOG: {}::{} - Device added", self.this_class, this_method); }
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
    pub async fn items(&self, db: &DbConn, filters: HashMap<String, String>) -> ModelOutput<Vec<DeviceModel>> 
    {
        let this_method = "items";
        if self.verbose { debug!("{}::{} - Starting items operation with filters: {:?}", self.this_class, this_method, filters); }

        match DeviceEntity::find().all(db).await 
        {
            Ok(items) => 
            {
                let output = ModelOutput::success(items, "Devices retrieved successfully".to_string());
                if self.verbose { info!("{}::{} - Success: Devices retrieved", self.this_class, this_method); }
                if self.log { info!("LOG: {}::{} - Devices retrieved", self.this_class, this_method); }
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
    pub async fn item(&self, db: &DbConn, id: i32) -> ModelOutput<DeviceModel> 
    {
        let this_method = "item";
        if self.verbose { debug!("{}::{} - Starting item operation for id: {}", self.this_class, this_method, id); }

        match DeviceEntity::find_by_id(id).one(db).await 
        {
            Ok(Some(model)) => 
            {
                let output = ModelOutput::success(model, "Device retrieved successfully".to_string());
                if self.verbose { info!("{}::{} - Success: Device {} found", self.this_class, this_method, id); }
                if self.log { info!("LOG: {}::{} - Device {} retrieved", self.this_class, this_method, id); }
                output
            }
            Ok(None) => 
            {
                let output = ModelOutput::error("Device not found".to_string());
                if self.verbose { info!("{}::{} - Device {} not found", self.this_class, this_method, id); }
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
    pub async fn update(&self, db: &DbConn, item: DeviceActiveModel) -> ModelOutput<DeviceModel> 
    {
        let this_method = "update";
        if self.verbose { debug!("{}::{} - Starting update operation", self.this_class, this_method); }

        match item.update(db).await 
        {
            Ok(model) => 
            {
                let output = ModelOutput::success(model, "Device updated successfully".to_string());
                if self.verbose { info!("{}::{} - Success: Device updated", self.this_class, this_method); }
                if self.log { info!("LOG: {}::{} - Device updated", self.this_class, this_method); }
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

        match DeviceEntity::delete_by_id(id).exec(db).await 
        {
            Ok(result) => 
            {
                if result.rows_affected > 0 
                {
                    let output = ModelOutput::success("deleted".to_string(), "Device deleted successfully".to_string());
                    if self.verbose { info!("{}::{} - Success: Device {} deleted", self.this_class, this_method, id); }
                    if self.log { info!("LOG: {}::{} - Device {} deleted", self.this_class, this_method, id); }
                    output
                } 
                else 
                {
                    let output = ModelOutput::error("Device not found".to_string());
                    if self.verbose { info!("{}::{} - Device {} not found", self.this_class, this_method, id); }
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
    pub async fn disable(&self, db: &DbConn, id: i32) -> ModelOutput<DeviceModel>
    {
        let this_method = "disable";
        if self.verbose { debug!("{}::{} - Starting disable operation for id: {}", self.this_class, this_method, id); }

        match DeviceEntity::find_by_id(id).one(db).await
        {
            Ok(Some(existing)) =>
            {
                let mut active: DeviceActiveModel = existing.into();
                active.enable = sea_orm::Set(false);

                match active.update(db).await
                {
                    Ok(updated) =>
                    {
                        let output = ModelOutput::success(updated, "Device disabled successfully".to_string());
                        if self.verbose { info!("{}::{} - Success: Device {} disabled", self.this_class, this_method, id); }
                        if self.log { info!("LOG: {}::{} - Device {} disabled", self.this_class, this_method, id); }
                        output
                    }
                    Err(e) =>
                    {
                        let error_msg = format!("Database error in {}::{}: {}", self.this_class, this_method, e);
                        let output = ModelOutput::error(error_msg.clone());
                        error!("{}::{} - Error: {}", self.this_class, this_method, error_msg);
                        output
                    }
                }
            }
            Ok(None) =>
            {
                let output = ModelOutput::error("Device not found".to_string());
                if self.verbose { info!("{}::{} - Device {} not found", self.this_class, this_method, id); }
                output
            }
            Err(e) =>
            {
                let error_msg = format!("Database error in {}::{}: {}", self.this_class, this_method, e);
                let output = ModelOutput::error(error_msg.clone());
                error!("{}::{} - Error: {}", self.this_class, this_method, error_msg);
                output
            }
        }
    }

    //------------------------- Enable
    pub async fn enable(&self, db: &DbConn, id: i32) -> ModelOutput<DeviceModel>
    {
        let this_method = "enable";
        if self.verbose { debug!("{}::{} - Starting enable operation for id: {}", self.this_class, this_method, id); }

        match DeviceEntity::find_by_id(id).one(db).await
        {
            Ok(Some(existing)) =>
            {
                let mut active: DeviceActiveModel = existing.into();
                active.enable = sea_orm::Set(true);

                match active.update(db).await
                {
                    Ok(updated) =>
                    {
                        let output = ModelOutput::success(updated, "Device enabled successfully".to_string());
                        if self.verbose { info!("{}::{} - Success: Device {} enabled", self.this_class, this_method, id); }
                        if self.log { info!("LOG: {}::{} - Device {} enabled", self.this_class, this_method, id); }
                        output
                    }
                    Err(e) =>
                    {
                        let error_msg = format!("Database error in {}::{}: {}", self.this_class, this_method, e);
                        let output = ModelOutput::error(error_msg.clone());
                        error!("{}::{} - Error: {}", self.this_class, this_method, error_msg);
                        output
                    }
                }
            }
            Ok(None) =>
            {
                let output = ModelOutput::error("Device not found".to_string());
                if self.verbose { info!("{}::{} - Device {} not found", self.this_class, this_method, id); }
                output
            }
            Err(e) =>
            {
                let error_msg = format!("Database error in {}::{}: {}", self.this_class, this_method, e);
                let output = ModelOutput::error(error_msg.clone());
                error!("{}::{} - Error: {}", self.this_class, this_method, error_msg);
                output
            }
        }
    }

}
