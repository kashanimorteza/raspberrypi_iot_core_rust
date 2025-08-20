//--------------------------------------------------------------------------------- Location
// src/orm/logics/timer.rs

//--------------------------------------------------------------------------------- Description
// ORM logic for timer

//--------------------------------------------------------------------------------- Import
use std::collections::HashMap;
use tracing::{info, error, debug};
use sea_orm::{ActiveModelTrait, DbConn, EntityTrait};
use crate::orm::models::timer::{ActiveModel as TimerActiveModel, Entity as TimerEntity, Model as TimerModel};
use crate::logics::general::ModelOutput;

//--------------------------------------------------------------------------------- Class
pub struct TimerORM 
{
    pub verbose: bool,
    pub log: bool,
    pub this_class: String,
    pub module: String,
}

impl TimerORM
{
    //-------------------------- [Init]
    pub fn new(verbose: bool, log: bool) -> Self 
    {
        Self 
        {
            verbose,
            log,
            this_class: "TimerORM".to_string(),
            module: "timer".to_string(),
        }
    }

    //-------------------------- [Add]
    pub async fn add(&self, db: &DbConn, item: TimerActiveModel) -> ModelOutput<TimerModel> 
    {
        let this_method = "add";
        if self.verbose { debug!("{}::{} - Starting add operation", self.this_class, this_method); }

        match item.insert(db).await 
        {
            Ok(model) => {
                let output = ModelOutput::success(model, "Timer added successfully".to_string());
                if self.verbose { info!("{}::{} - Success: Timer added", self.this_class, this_method); }
                if self.log { info!("LOG: {}::{} - Timer added", self.this_class, this_method); }
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
    pub async fn items(&self, db: &DbConn, filters: HashMap<String, String>) -> ModelOutput<Vec<TimerModel>> 
    {
        let this_method = "items";
        if self.verbose { debug!("{}::{} - Starting items operation with filters: {:?}", self.this_class, this_method, filters); }

        match TimerEntity::find().all(db).await 
        {
            Ok(items) => 
            {
                let output = ModelOutput::success(items, "Timers retrieved successfully".to_string());
                if self.verbose { info!("{}::{} - Success: Timers retrieved", self.this_class, this_method); }
                if self.log { info!("LOG: {}::{} - Timers retrieved", self.this_class, this_method); }
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
    pub async fn item(&self, db: &DbConn, id: i32) -> ModelOutput<TimerModel> 
    {
        let this_method = "item";
        if self.verbose { debug!("{}::{} - Starting item operation for id: {}", self.this_class, this_method, id); }

        match TimerEntity::find_by_id(id).one(db).await 
        {
            Ok(Some(model)) => 
            {
                let output = ModelOutput::success(model, "Timer retrieved successfully".to_string());
                if self.verbose { info!("{}::{} - Success: Timer {} found", self.this_class, this_method, id); }
                if self.log { info!("LOG: {}::{} - Timer {} retrieved", self.this_class, this_method, id); }
                output
            }
            Ok(None) => 
            {
                let output = ModelOutput::error("Timer not found".to_string());
                if self.verbose { info!("{}::{} - Timer {} not found", self.this_class, this_method, id); }
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
    pub async fn update(&self, db: &DbConn, item: TimerActiveModel) -> ModelOutput<TimerModel> 
    {
        let this_method = "update";
        if self.verbose { debug!("{}::{} - Starting update operation", self.this_class, this_method); }

        match item.update(db).await 
        {
            Ok(model) => 
            {
                let output = ModelOutput::success(model, "Timer updated successfully".to_string());
                if self.verbose { info!("{}::{} - Success: Timer updated", self.this_class, this_method); }
                if self.log { info!("LOG: {}::{} - Timer updated", self.this_class, this_method); }
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

        match TimerEntity::delete_by_id(id).exec(db).await 
        {
            Ok(result) => 
            {
                if result.rows_affected > 0 
                {
                    let output = ModelOutput::success("deleted".to_string(), "Timer deleted successfully".to_string());
                    if self.verbose { info!("{}::{} - Success: Timer {} deleted", self.this_class, this_method, id); }
                    if self.log { info!("LOG: {}::{} - Timer {} deleted", self.this_class, this_method, id); }
                    output
                } 
                else 
                {
                    let output = ModelOutput::error("Timer not found".to_string());
                    if self.verbose { info!("{}::{} - Timer {} not found", self.this_class, this_method, id); }
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
