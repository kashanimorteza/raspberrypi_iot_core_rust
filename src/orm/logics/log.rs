//--------------------------------------------------------------------------------- Location
// src/orm/logics/log.rs

//--------------------------------------------------------------------------------- Description
// ORM logic for log

//--------------------------------------------------------------------------------- Import
use std::collections::HashMap;
use tracing::{info, error, debug};
use sea_orm::{ActiveModelTrait, DbConn, EntityTrait};
use crate::orm::models::log::{ActiveModel as LogActiveModel, Entity as LogEntity, Model as LogModel};
use crate::logics::general::ModelOutput;

//--------------------------------------------------------------------------------- Class
pub struct LogORM 
{
    pub verbose: bool,
    pub log: bool,
    pub this_class: String,
    pub module: String,
}

impl LogORM
{
    //-------------------------- [Init]
    pub fn new(verbose: bool, log: bool) -> Self 
    {
        Self 
        {
            verbose,
            log,
            this_class: "LogORM".to_string(),
            module: "log".to_string(),
        }
    }

    //-------------------------- [Add]
    pub async fn add(&self, db: &DbConn, item: LogActiveModel) -> ModelOutput<LogModel> 
    {
        let this_method = "add";
        if self.verbose { debug!("{}::{} - Starting add operation", self.this_class, this_method); }

        match item.insert(db).await 
        {
            Ok(model) => {
                let output = ModelOutput::success(model, "Log added successfully".to_string());
                if self.verbose { info!("{}::{} - Success: Log added", self.this_class, this_method); }
                if self.log { info!("LOG: {}::{} - Log added", self.this_class, this_method); }
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

    //-------------------------- [Items]
    pub async fn items(&self, db: &DbConn, filters: HashMap<String, String>) -> ModelOutput<Vec<LogModel>> 
    {
        let this_method = "items";
        if self.verbose { debug!("{}::{} - Starting items operation with filters: {:?}", self.this_class, this_method, filters); }

        match LogEntity::find().all(db).await 
        {
            Ok(items) => 
            {
                let output = ModelOutput::success(items, "Logs retrieved successfully".to_string());
                if self.verbose { info!("{}::{} - Success: Logs retrieved", self.this_class, this_method); }
                if self.log { info!("LOG: {}::{} - Logs retrieved", self.this_class, this_method); }
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

    //-------------------------- [Item]
    pub async fn item(&self, db: &DbConn, id: i32) -> ModelOutput<LogModel> 
    {
        let this_method = "item";
        if self.verbose { debug!("{}::{} - Starting item operation for id: {}", self.this_class, this_method, id); }

        match LogEntity::find_by_id(id).one(db).await 
        {
            Ok(Some(model)) => 
            {
                let output = ModelOutput::success(model, "Log retrieved successfully".to_string());
                if self.verbose { info!("{}::{} - Success: Log {} found", self.this_class, this_method, id); }
                if self.log { info!("LOG: {}::{} - Log {} retrieved", self.this_class, this_method, id); }
                output
            }
            Ok(None) => 
            {
                let output = ModelOutput::error("Log not found".to_string());
                if self.verbose { info!("{}::{} - Log {} not found", self.this_class, this_method, id); }
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

    //-------------------------- [Update]
    pub async fn update(&self, db: &DbConn, item: LogActiveModel) -> ModelOutput<LogModel> 
    {
        let this_method = "update";
        if self.verbose { debug!("{}::{} - Starting update operation", self.this_class, this_method); }

        match item.update(db).await 
        {
            Ok(model) => 
            {
                let output = ModelOutput::success(model, "Log updated successfully".to_string());
                if self.verbose { info!("{}::{} - Success: Log updated", self.this_class, this_method); }
                if self.log { info!("LOG: {}::{} - Log updated", self.this_class, this_method); }
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

    //-------------------------- [Delete]
    pub async fn delete(&self, db: &DbConn, id: i32) -> ModelOutput<String> 
    {
        let this_method = "delete";
        if self.verbose { debug!("{}::{} - Starting delete operation for id: {}", self.this_class, this_method, id); }

        match LogEntity::delete_by_id(id).exec(db).await 
        {
            Ok(result) => 
            {
                if result.rows_affected > 0 
                {
                    let output = ModelOutput::success("deleted".to_string(), "Log deleted successfully".to_string());
                    if self.verbose { info!("{}::{} - Success: Log {} deleted", self.this_class, this_method, id); }
                    if self.log { info!("LOG: {}::{} - Log {} deleted", self.this_class, this_method, id); }
                    output
                } 
                else 
                {
                    let output = ModelOutput::error("Log not found".to_string());
                    if self.verbose { info!("{}::{} - Log {} not found", self.this_class, this_method, id); }
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


