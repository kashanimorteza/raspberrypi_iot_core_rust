//--------------------------------------------------------------------------------- Location
// src/orm/logics/config.rs

//--------------------------------------------------------------------------------- Description
// ORM logic for config

//--------------------------------------------------------------------------------- Import
use std::collections::HashMap;
use tracing::{info, error, debug};
use sea_orm::{ActiveModelTrait, DbConn, EntityTrait};
use crate::orm::models::config::{ActiveModel as ConfigActiveModel, Entity as ConfigEntity, Model as ConfigModel};
use crate::logics::general::ModelOutput;

//--------------------------------------------------------------------------------- Class
pub struct ConfigORM 
{
    pub verbose: bool,
    pub log: bool,
    pub this_class: String,
    pub module: String,
}

impl ConfigORM
{
    //------------------------- New
    pub fn new(verbose: bool, log: bool) -> Self 
    {
        Self 
        {
            verbose,
            log,
            this_class: "ConfigORM".to_string(),
            module: "config".to_string(),
        }
    }

    //------------------------- Add
    pub async fn add(&self, db: &DbConn, item: ConfigActiveModel) -> ModelOutput<ConfigModel> 
    {
        let this_method = "add";
        if self.verbose { debug!("{}::{} - Starting add operation", self.this_class, this_method); }

        match item.insert(db).await 
        {
            Ok(model) => {
                let output = ModelOutput::success(model, "Config added successfully".to_string());
                if self.verbose { info!("{}::{} - Success: Config added", self.this_class, this_method); }
                if self.log { info!("LOG: {}::{} - Config added", self.this_class, this_method); }
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
    pub async fn items(&self, db: &DbConn, filters: HashMap<String, String>) -> ModelOutput<Vec<ConfigModel>> 
    {
        let this_method = "items";
        if self.verbose { debug!("{}::{} - Starting items operation with filters: {:?}", self.this_class, this_method, filters); }

        match ConfigEntity::find().all(db).await 
        {
            Ok(items) => 
            {
                let output = ModelOutput::success(items, "Configs retrieved successfully".to_string());
                if self.verbose { info!("{}::{} - Success: Configs retrieved", self.this_class, this_method); }
                if self.log { info!("LOG: {}::{} - Configs retrieved", self.this_class, this_method); }
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
    pub async fn item(&self, db: &DbConn, id: i32) -> ModelOutput<ConfigModel> 
    {
        let this_method = "item";
        if self.verbose { debug!("{}::{} - Starting item operation for id: {}", self.this_class, this_method, id); }

        match ConfigEntity::find_by_id(id).one(db).await 
        {
            Ok(Some(model)) => 
            {
                let output = ModelOutput::success(model, "Config retrieved successfully".to_string());
                if self.verbose { info!("{}::{} - Success: Config {} found", self.this_class, this_method, id); }
                if self.log { info!("LOG: {}::{} - Config {} retrieved", self.this_class, this_method, id); }
                output
            }
            Ok(None) => 
            {
                let output = ModelOutput::error("Config not found".to_string());
                if self.verbose { info!("{}::{} - Config {} not found", self.this_class, this_method, id); }
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
    pub async fn update(&self, db: &DbConn, item: ConfigActiveModel) -> ModelOutput<ConfigModel> 
    {
        let this_method = "update";
        if self.verbose { debug!("{}::{} - Starting update operation", self.this_class, this_method); }

        match item.update(db).await 
        {
            Ok(model) => 
            {
                let output = ModelOutput::success(model, "Config updated successfully".to_string());
                if self.verbose { info!("{}::{} - Success: Config updated", self.this_class, this_method); }
                if self.log { info!("LOG: {}::{} - Config updated", self.this_class, this_method); }
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

        match ConfigEntity::delete_by_id(id).exec(db).await 
        {
            Ok(result) => 
            {
                if result.rows_affected > 0 
                {
                    let output = ModelOutput::success("deleted".to_string(), "Config deleted successfully".to_string());
                    if self.verbose { info!("{}::{} - Success: Config {} deleted", self.this_class, this_method, id); }
                    if self.log { info!("LOG: {}::{} - Config {} deleted", self.this_class, this_method, id); }
                    output
                } 
                else 
                {
                    let output = ModelOutput::error("Config not found".to_string());
                    if self.verbose { info!("{}::{} - Config {} not found", self.this_class, this_method, id); }
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

    //------------------------- Status (Toggle Debug)
    pub async fn status(&self, db: &DbConn, id: i32) -> ModelOutput<ConfigModel>
    {
        let this_method = "status";

        match ConfigEntity::find_by_id(id).one(db).await
        {
            Ok(Some(existing)) =>
            {
                // Get the current debug value before moving existing
                let current_debug = existing.debug;
                let mut active: ConfigActiveModel = existing.into();
                // Toggle the debug field: if true, set to false; if false, set to true
                active.debug = sea_orm::Set(!current_debug);

                match active.update(db).await
                {
                    Ok(updated) => {
                        let message = if current_debug {
                            "Config debug disabled successfully".to_string()
                        } else {
                            "Config debug enabled successfully".to_string()
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
            Ok(None) => ModelOutput::error("Config not found".to_string()),
            Err(e) => {
                let error_msg = format!("Database error in {}::{}: {}", self.this_class, this_method, e);
                error!("{}::{} - Error: {}", self.this_class, this_method, error_msg);
                ModelOutput::error(error_msg)
            }
        }
    }
}


