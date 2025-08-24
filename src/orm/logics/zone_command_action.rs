//--------------------------------------------------------------------------------- Location
// src/orm/logics/zone_command_action.rs

//--------------------------------------------------------------------------------- Description
// ORM logic for zone_command_action

//--------------------------------------------------------------------------------- Import
use std::collections::HashMap;
use tracing::{info, error, debug};
use sea_orm::{ActiveModelTrait, DbConn, EntityTrait, QueryOrder, QueryFilter, ColumnTrait, Condition};
use crate::orm::models::zone_command_action::{ActiveModel as ZoneCommandActionActiveModel, Entity as ZoneCommandActionEntity, Model as ZoneCommandActionModel, Column as ZoneCommandActionColumn};
use crate::logics::general::ModelOutput;

//--------------------------------------------------------------------------------- Class
pub struct ZoneCommandActionORM 
{
    pub verbose: bool,
    pub log: bool,
    pub this_class: String,
    pub module: String,
}

impl ZoneCommandActionORM
{
    //------------------------- New
    pub fn new(verbose: bool, log: bool) -> Self 
    {
        Self 
        {
            verbose,
            log,
            this_class: "ZoneCommandActionORM".to_string(),
            module: "zone_command_action".to_string(),
        }
    }

    //------------------------- Add
    pub async fn add(&self, db: &DbConn, item: ZoneCommandActionActiveModel) -> ModelOutput<ZoneCommandActionModel> 
    {
        let this_method = "add";
        if self.verbose { debug!("{}::{} - Starting add operation", self.this_class, this_method); }

        match item.insert(db).await 
        {
            Ok(model) => {
                let output = ModelOutput::success(model, "zonecommandaction added successfully".to_string());
                if self.verbose { info!("{}::{} - Success: zonecommandaction added", self.this_class, this_method); }
                if self.log { info!("LOG: {}::{} - zonecommandaction added", self.this_class, this_method); }
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
    pub async fn items(&self, db: &DbConn, filters: HashMap<String, String>) -> ModelOutput<Vec<ZoneCommandActionModel>> 
    {
        let this_method = "items";
        if self.verbose { debug!("{}::{} - Starting items operation with filters: {:?}", self.this_class, this_method, filters); }

        let mut query = ZoneCommandActionEntity::find();
        if !filters.is_empty() {
            let mut condition = Condition::all();

            if let Some(id_str) = filters.get("id") { if let Ok(id) = id_str.parse::<i32>() { condition = condition.add(ZoneCommandActionColumn::Id.eq(id)); } }
            if let Some(zone_command_id_str) = filters.get("zone_command_id") { if let Ok(zone_command_id) = zone_command_id_str.parse::<i32>() { condition = condition.add(ZoneCommandActionColumn::ZoneCommandId.eq(zone_command_id)); } }
            if let Some(device_id_str) = filters.get("device_id") { if let Ok(device_id) = device_id_str.parse::<i32>() { condition = condition.add(ZoneCommandActionColumn::DeviceId.eq(device_id)); } }
            if let Some(command_id_str) = filters.get("command_id") { if let Ok(command_id) = command_id_str.parse::<i32>() { condition = condition.add(ZoneCommandActionColumn::CommandId.eq(Some(command_id))); } }
            if let Some(name) = filters.get("name") { condition = condition.add(ZoneCommandActionColumn::Name.contains(name)); }
            if let Some(description) = filters.get("description") { condition = condition.add(ZoneCommandActionColumn::Description.contains(description)); }
            if let Some(enable_str) = filters.get("enable") { if let Ok(enable) = enable_str.parse::<bool>() { condition = condition.add(ZoneCommandActionColumn::Enable.eq(enable)); } }

            query = query.filter(condition);
        }

        match query.order_by_asc(ZoneCommandActionColumn::Id).all(db).await 
        {
            Ok(items) => 
            {
                let message = if filters.is_empty() { "zonecommandactions retrieved successfully".to_string() } else { format!("Filtered zonecommandactions retrieved successfully (found {} items)", items.len()) };
                let output = ModelOutput::success(items, message);
                if self.verbose { info!("{}::{} - Success: zonecommandactions retrieved", self.this_class, this_method); }
                if self.log { info!("LOG: {}::{} - zonecommandactions retrieved", self.this_class, this_method); }
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
    pub async fn item(&self, db: &DbConn, id: i32) -> ModelOutput<ZoneCommandActionModel> 
    {
        let this_method = "item";
        if self.verbose { debug!("{}::{} - Starting item operation for id: {}", self.this_class, this_method, id); }

        match ZoneCommandActionEntity::find_by_id(id).one(db).await 
        {
            Ok(Some(model)) => 
            {
                let output = ModelOutput::success(model, "zonecommandaction retrieved successfully".to_string());
                if self.verbose { info!("{}::{} - Success: zonecommandaction {} found", self.this_class, this_method, id); }
                if self.log { info!("LOG: {}::{} - zonecommandaction {} retrieved", self.this_class, this_method, id); }
                output
            }
            Ok(None) => 
            {
                let output = ModelOutput::error("zonecommandaction not found".to_string());
                if self.verbose { info!("{}::{} - zonecommandaction {} not found", self.this_class, this_method, id); }
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
    pub async fn update(&self, db: &DbConn, item: ZoneCommandActionActiveModel) -> ModelOutput<ZoneCommandActionModel> 
    {
        let this_method = "update";
        if self.verbose { debug!("{}::{} - Starting update operation", self.this_class, this_method); }

        match item.update(db).await 
        {
            Ok(model) => 
            {
                let output = ModelOutput::success(model, "zonecommandaction updated successfully".to_string());
                if self.verbose { info!("{}::{} - Success: zonecommandaction updated", self.this_class, this_method); }
                if self.log { info!("LOG: {}::{} - zonecommandaction updated", self.this_class, this_method); }
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

        match ZoneCommandActionEntity::delete_by_id(id).exec(db).await 
        {
            Ok(result) => 
            {
                if result.rows_affected > 0 
                {
                    let output = ModelOutput::success("deleted".to_string(), "zonecommandaction deleted successfully".to_string());
                    if self.verbose { info!("{}::{} - Success: zonecommandaction {} deleted", self.this_class, this_method, id); }
                    if self.log { info!("LOG: {}::{} - zonecommandaction {} deleted", self.this_class, this_method, id); }
                    output
                } 
                else 
                {
                    let output = ModelOutput::error("zonecommandaction not found".to_string());
                    if self.verbose { info!("{}::{} - zonecommandaction {} not found", self.this_class, this_method, id); }
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
    pub async fn disable(&self, db: &DbConn, id: i32) -> ModelOutput<ZoneCommandActionModel>
    {
        let this_method = "disable";
        if self.verbose { debug!("{}::{} - Starting disable operation for id: {}", self.this_class, this_method, id); }

        match ZoneCommandActionEntity::find_by_id(id).one(db).await
        {
            Ok(Some(existing)) =>
            {
                let mut active: ZoneCommandActionActiveModel = existing.into();
                active.enable = sea_orm::Set(false);

                match active.update(db).await
                {
                    Ok(updated) => ModelOutput::success(updated, "ZoneCommandAction disabled successfully".to_string()),
                    Err(e) => {
                        let error_msg = format!("Database error in {}::{}: {}", self.this_class, this_method, e);
                        error!("{}::{} - Error: {}", self.this_class, this_method, error_msg);
                        ModelOutput::error(error_msg)
                    }
                }
            }
            Ok(None) => ModelOutput::error("ZoneCommandAction not found".to_string()),
            Err(e) => {
                let error_msg = format!("Database error in {}::{}: {}", self.this_class, this_method, e);
                error!("{}::{} - Error: {}", self.this_class, this_method, error_msg);
                ModelOutput::error(error_msg)
            }
        }
    }

    //------------------------- Enable
    pub async fn enable(&self, db: &DbConn, id: i32) -> ModelOutput<ZoneCommandActionModel>
    {
        let this_method = "enable";
        if self.verbose { debug!("{}::{} - Starting enable operation for id: {}", self.this_class, this_method, id); }

        match ZoneCommandActionEntity::find_by_id(id).one(db).await
        {
            Ok(Some(existing)) =>
            {
                let mut active: ZoneCommandActionActiveModel = existing.into();
                active.enable = sea_orm::Set(true);

                match active.update(db).await
                {
                    Ok(updated) => ModelOutput::success(updated, "ZoneCommandAction enabled successfully".to_string()),
                    Err(e) => {
                        let error_msg = format!("Database error in {}::{}: {}", self.this_class, this_method, e);
                        error!("{}::{} - Error: {}", self.this_class, this_method, error_msg);
                        ModelOutput::error(error_msg)
                    }
                }
            }
            Ok(None) => ModelOutput::error("ZoneCommandAction not found".to_string()),
            Err(e) => {
                let error_msg = format!("Database error in {}::{}: {}", self.this_class, this_method, e);
                error!("{}::{} - Error: {}", self.this_class, this_method, error_msg);
                ModelOutput::error(error_msg)
            }
        }
    }

    //------------------------- Status (Toggle Enable)
    pub async fn status(&self, db: &DbConn, id: i32) -> ModelOutput<ZoneCommandActionModel>
    {
        let this_method = "status";
        if self.verbose { debug!("{}::{} - Starting status operation for id: {}", self.this_class, this_method, id); }

        match ZoneCommandActionEntity::find_by_id(id).one(db).await
        {
            Ok(Some(existing)) =>
            {
                // Get the current enable value before moving existing
                let current_enable = existing.enable;
                let mut active: ZoneCommandActionActiveModel = existing.into();
                // Toggle the enable field: if true, set to false; if false, set to true
                active.enable = sea_orm::Set(!current_enable);

                match active.update(db).await
                {
                    Ok(updated) => {
                        let message = if current_enable {
                            "Zone command action disabled successfully".to_string()
                        } else {
                            "Zone command action enabled successfully".to_string()
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
            Ok(None) => ModelOutput::error("Zone command action not found".to_string()),
            Err(e) => {
                let error_msg = format!("Database error in {}::{}: {}", self.this_class, this_method, e);
                error!("{}::{} - Error: {}", self.this_class, this_method, error_msg);
                ModelOutput::error(error_msg)
            }
        }
    }
}