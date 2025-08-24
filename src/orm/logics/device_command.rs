//--------------------------------------------------------------------------------- Location
// src/orm/logics/device_command.rs

//--------------------------------------------------------------------------------- Description
// ORM logic for device_command

//--------------------------------------------------------------------------------- Import
use std::collections::HashMap;
use tracing::{info, error, debug};
use sea_orm::{ActiveModelTrait, DbConn, EntityTrait, QueryOrder, QueryFilter, ColumnTrait, Condition};
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

        let mut query = DeviceCommandEntity::find();
        if !filters.is_empty() {
            let mut condition = Condition::all();

            if let Some(id_str) = filters.get("id") { if let Ok(id) = id_str.parse::<i32>() { condition = condition.add(DeviceCommandColumn::Id.eq(id)); } }
            if let Some(device_id_str) = filters.get("device_id") { if let Ok(device_id) = device_id_str.parse::<i32>() { condition = condition.add(DeviceCommandColumn::DeviceId.eq(device_id)); } }
            if let Some(name) = filters.get("name") { condition = condition.add(DeviceCommandColumn::Name.contains(name)); }
            if let Some(value_from_str) = filters.get("value_from") { if let Ok(value_from) = value_from_str.parse::<i32>() { condition = condition.add(DeviceCommandColumn::ValueFrom.eq(Some(value_from))); } }
            if let Some(value_to_str) = filters.get("value_to") { if let Ok(value_to) = value_to_str.parse::<i32>() { condition = condition.add(DeviceCommandColumn::ValueTo.eq(Some(value_to))); } }
            if let Some(delay_str) = filters.get("delay") { if let Ok(delay) = delay_str.parse::<i32>() { condition = condition.add(DeviceCommandColumn::Delay.eq(Some(delay))); } }
            if let Some(description) = filters.get("description") { condition = condition.add(DeviceCommandColumn::Description.contains(description)); }
            if let Some(reload_str) = filters.get("reload") { if let Ok(reload) = reload_str.parse::<bool>() { condition = condition.add(DeviceCommandColumn::Reload.eq(reload)); } }
            if let Some(enable_str) = filters.get("enable") { if let Ok(enable) = enable_str.parse::<bool>() { condition = condition.add(DeviceCommandColumn::Enable.eq(enable)); } }
            if let Some(type_val) = filters.get("type") { condition = condition.add(DeviceCommandColumn::Type.contains(type_val)); }

            query = query.filter(condition);
        }

        match query.order_by_asc(DeviceCommandColumn::Id).all(db).await 
        {
            Ok(items) => 
            {
                let message = if filters.is_empty() { "DeviceCommands retrieved successfully".to_string() } else { format!("Filtered device commands retrieved successfully (found {} items)", items.len()) };
                let output = ModelOutput::success(items, message);
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
