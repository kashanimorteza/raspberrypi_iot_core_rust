//--------------------------------------------------------------------------------- Location
// src/orm/logics/config.rs

//--------------------------------------------------------------------------------- Description
// ORM logic for config

//--------------------------------------------------------------------------------- Import
use std::collections::HashMap;
use tracing::{info, error, debug};
use sea_orm::{ActiveModelTrait, DbConn, EntityTrait, QueryOrder, QueryFilter, ColumnTrait, Condition};
use crate::orm::models::config::{ActiveModel as ConfigActiveModel, Entity as ConfigEntity, Model as ConfigModel, Column as ConfigColumn};
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

        let mut query = ConfigEntity::find();
        if !filters.is_empty() {
            let mut condition = Condition::all();

            if let Some(id_str) = filters.get("id") {
                if let Ok(id) = id_str.parse::<i32>() {
                    condition = condition.add(ConfigColumn::Id.eq(id));
                }
            }
            if let Some(name) = filters.get("name") {
                condition = condition.add(ConfigColumn::Name.contains(name));
            }
            if let Some(webapi_host) = filters.get("webapi_host") {
                condition = condition.add(ConfigColumn::WebapiHost.contains(webapi_host));
            }
            if let Some(webapi_port_str) = filters.get("webapi_port") {
                if let Ok(webapi_port) = webapi_port_str.parse::<i32>() {
                    condition = condition.add(ConfigColumn::WebapiPort.eq(Some(webapi_port)));
                }
            }
            if let Some(debug_str) = filters.get("debug") {
                if let Ok(debug) = debug_str.parse::<bool>() {
                    condition = condition.add(ConfigColumn::Debug.eq(debug));
                }
            }
            if let Some(log_str) = filters.get("log") {
                if let Ok(log) = log_str.parse::<bool>() {
                    condition = condition.add(ConfigColumn::Log.eq(log));
                }
            }
            if let Some(verbose_str) = filters.get("verbose") {
                if let Ok(verbose) = verbose_str.parse::<bool>() {
                    condition = condition.add(ConfigColumn::Verbose.eq(verbose));
                }
            }

            query = query.filter(condition);
        }

        match query.order_by_asc(ConfigColumn::Id).all(db).await 
        {
            Ok(items) => 
            {
                let message = if filters.is_empty() { "Configs retrieved successfully".to_string() } else { format!("Filtered configs retrieved successfully (found {} items)", items.len()) };
                let output = ModelOutput::success(items, message);
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


