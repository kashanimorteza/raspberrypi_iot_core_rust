//--------------------------------------------------------------------------------- Location
// src/orm/logics/zone_command_action.rs

//--------------------------------------------------------------------------------- Description
// ORM logic for zone_command_action

//--------------------------------------------------------------------------------- Import
use std::collections::HashMap;
use tracing::{info, error, debug};
use sea_orm::{ActiveModelTrait, DbConn, EntityTrait};
use crate::orm::models::zone_command_action::{ActiveModel as ZoneCommandActionActiveModel, Entity as ZoneCommandActionEntity, Model as ZoneCommandActionModel};
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

    pub async fn items(&self, db: &DbConn, _filters: HashMap<String, String>) -> ModelOutput<Vec<ZoneCommandActionModel>> 
    {
        let this_method = "items";
        if self.verbose { debug!("{}::{} - Starting items operation", self.this_class, this_method); }

        match ZoneCommandActionEntity::find().all(db).await 
        {
            Ok(items) => 
            {
                let output = ModelOutput::success(items, "zonecommandactions retrieved successfully".to_string());
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
}
    
    //-------------------------- [Disable]
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

    //-------------------------- [Enable]
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