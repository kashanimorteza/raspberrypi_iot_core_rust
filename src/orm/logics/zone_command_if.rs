//--------------------------------------------------------------------------------- Location
// src/orm/logics/zone_command_if.rs

//--------------------------------------------------------------------------------- Description
// ORM logic for zone_command_if

//--------------------------------------------------------------------------------- Import
use std::collections::HashMap;
use tracing::{info, error, debug};
use sea_orm::{ActiveModelTrait, DbConn, EntityTrait};
use crate::orm::models::zone_command_if::{ActiveModel as ZoneCommandIfActiveModel, Entity as ZoneCommandIfEntity, Model as ZoneCommandIfModel};
use crate::logics::general::ModelOutput;

//--------------------------------------------------------------------------------- Class
pub struct ZoneCommandIfORM 
{
    pub verbose: bool,
    pub log: bool,
    pub this_class: String,
    pub module: String,
}

impl ZoneCommandIfORM
{
    //------------------------- New
    pub fn new(verbose: bool, log: bool) -> Self 
    {
        Self 
        {
            verbose,
            log,
            this_class: "ZoneCommandIfORM".to_string(),
            module: "zone_command_if".to_string(),
        }
    }

    //------------------------- Add
    pub async fn add(&self, db: &DbConn, item: ZoneCommandIfActiveModel) -> ModelOutput<ZoneCommandIfModel> 
    {
        let this_method = "add";
        if self.verbose { debug!("{}::{} - Starting add operation", self.this_class, this_method); }

        match item.insert(db).await 
        {
            Ok(model) => {
                let output = ModelOutput::success(model, "zonecommandif added successfully".to_string());
                if self.verbose { info!("{}::{} - Success: zonecommandif added", self.this_class, this_method); }
                if self.log { info!("LOG: {}::{} - zonecommandif added", self.this_class, this_method); }
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
    pub async fn items(&self, db: &DbConn, _filters: HashMap<String, String>) -> ModelOutput<Vec<ZoneCommandIfModel>> 
    {
        let this_method = "items";
        if self.verbose { debug!("{}::{} - Starting items operation", self.this_class, this_method); }

        match ZoneCommandIfEntity::find().all(db).await 
        {
            Ok(items) => 
            {
                let output = ModelOutput::success(items, "zonecommandifs retrieved successfully".to_string());
                if self.verbose { info!("{}::{} - Success: zonecommandifs retrieved", self.this_class, this_method); }
                if self.log { info!("LOG: {}::{} - zonecommandifs retrieved", self.this_class, this_method); }
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
    pub async fn item(&self, db: &DbConn, id: i32) -> ModelOutput<ZoneCommandIfModel> 
    {
        let this_method = "item";
        if self.verbose { debug!("{}::{} - Starting item operation for id: {}", self.this_class, this_method, id); }

        match ZoneCommandIfEntity::find_by_id(id).one(db).await 
        {
            Ok(Some(model)) => 
            {
                let output = ModelOutput::success(model, "zonecommandif retrieved successfully".to_string());
                if self.verbose { info!("{}::{} - Success: zonecommandif {} found", self.this_class, this_method, id); }
                if self.log { info!("LOG: {}::{} - zonecommandif {} retrieved", self.this_class, this_method, id); }
                output
            }
            Ok(None) => 
            {
                let output = ModelOutput::error("zonecommandif not found".to_string());
                if self.verbose { info!("{}::{} - zonecommandif {} not found", self.this_class, this_method, id); }
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
    pub async fn update(&self, db: &DbConn, item: ZoneCommandIfActiveModel) -> ModelOutput<ZoneCommandIfModel> 
    {
        let this_method = "update";
        if self.verbose { debug!("{}::{} - Starting update operation", self.this_class, this_method); }

        match item.update(db).await 
        {
            Ok(model) => 
            {
                let output = ModelOutput::success(model, "zonecommandif updated successfully".to_string());
                if self.verbose { info!("{}::{} - Success: zonecommandif updated", self.this_class, this_method); }
                if self.log { info!("LOG: {}::{} - zonecommandif updated", self.this_class, this_method); }
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

        match ZoneCommandIfEntity::delete_by_id(id).exec(db).await 
        {
            Ok(result) => 
            {
                if result.rows_affected > 0 
                {
                    let output = ModelOutput::success("deleted".to_string(), "zonecommandif deleted successfully".to_string());
                    if self.verbose { info!("{}::{} - Success: zonecommandif {} deleted", self.this_class, this_method, id); }
                    if self.log { info!("LOG: {}::{} - zonecommandif {} deleted", self.this_class, this_method, id); }
                    output
                } 
                else 
                {
                    let output = ModelOutput::error("zonecommandif not found".to_string());
                    if self.verbose { info!("{}::{} - zonecommandif {} not found", self.this_class, this_method, id); }
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
    pub async fn disable(&self, db: &DbConn, id: i32) -> ModelOutput<ZoneCommandIfModel>
    {
        let this_method = "disable";
        if self.verbose { debug!("{}::{} - Starting disable operation for id: {}", self.this_class, this_method, id); }

        match ZoneCommandIfEntity::find_by_id(id).one(db).await
        {
            Ok(Some(existing)) =>
            {
                let mut active: ZoneCommandIfActiveModel = existing.into();
                active.enable = sea_orm::Set(false);

                match active.update(db).await
                {
                    Ok(updated) => ModelOutput::success(updated, "ZoneCommandIf disabled successfully".to_string()),
                    Err(e) => {
                        let error_msg = format!("Database error in {}::{}: {}", self.this_class, this_method, e);
                        error!("{}::{} - Error: {}", self.this_class, this_method, error_msg);
                        ModelOutput::error(error_msg)
                    }
                }
            }
            Ok(None) => ModelOutput::error("ZoneCommandIf not found".to_string()),
            Err(e) => {
                let error_msg = format!("Database error in {}::{}: {}", self.this_class, this_method, e);
                error!("{}::{} - Error: {}", self.this_class, this_method, error_msg);
                ModelOutput::error(error_msg)
            }
        }
    }

    //------------------------- Enable
    pub async fn enable(&self, db: &DbConn, id: i32) -> ModelOutput<ZoneCommandIfModel>
    {
        let this_method = "enable";
        if self.verbose { debug!("{}::{} - Starting enable operation for id: {}", self.this_class, this_method, id); }

        match ZoneCommandIfEntity::find_by_id(id).one(db).await
        {
            Ok(Some(existing)) =>
            {
                let mut active: ZoneCommandIfActiveModel = existing.into();
                active.enable = sea_orm::Set(true);

                match active.update(db).await
                {
                    Ok(updated) => ModelOutput::success(updated, "ZoneCommandIf enabled successfully".to_string()),
                    Err(e) => {
                        let error_msg = format!("Database error in {}::{}: {}", self.this_class, this_method, e);
                        error!("{}::{} - Error: {}", self.this_class, this_method, error_msg);
                        ModelOutput::error(error_msg)
                    }
                }
            }
            Ok(None) => ModelOutput::error("ZoneCommandIf not found".to_string()),
            Err(e) => {
                let error_msg = format!("Database error in {}::{}: {}", self.this_class, this_method, e);
                error!("{}::{} - Error: {}", self.this_class, this_method, error_msg);
                ModelOutput::error(error_msg)
            }
        }
    }

    //------------------------- Status (Toggle Enable)
    pub async fn status(&self, db: &DbConn, id: i32) -> ModelOutput<ZoneCommandIfModel>
    {
        let this_method = "status";
        if self.verbose { debug!("{}::{} - Starting status operation for id: {}", self.this_class, this_method, id); }

        match ZoneCommandIfEntity::find_by_id(id).one(db).await
        {
            Ok(Some(existing)) =>
            {
                // Get the current enable value before moving existing
                let current_enable = existing.enable;
                let mut active: ZoneCommandIfActiveModel = existing.into();
                // Toggle the enable field: if true, set to false; if false, set to true
                active.enable = sea_orm::Set(!current_enable);

                match active.update(db).await
                {
                    Ok(updated) => {
                        let message = if current_enable {
                            "Zone command if disabled successfully".to_string()
                        } else {
                            "Zone command if enabled successfully".to_string()
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
            Ok(None) => ModelOutput::error("Zone command if not found".to_string()),
            Err(e) => {
                let error_msg = format!("Database error in {}::{}: {}", self.this_class, this_method, e);
                error!("{}::{} - Error: {}", self.this_class, this_method, error_msg);
                ModelOutput::error(error_msg)
            }
        }
    }
}