//--------------------------------------------------------------------------------- Location
// src/orm/logics/timer_item.rs

//--------------------------------------------------------------------------------- Description
// ORM logic for timer_item

//--------------------------------------------------------------------------------- Import
use std::collections::HashMap;
use tracing::{info, error, debug};
use sea_orm::{ActiveModelTrait, DbConn, EntityTrait};
use crate::orm::models::timer_item::{ActiveModel as TimerItemActiveModel, Entity as TimerItemEntity, Model as TimerItemModel};
use crate::logics::general::ModelOutput;

//--------------------------------------------------------------------------------- Class
pub struct TimerItemORM 
{
    pub verbose: bool,
    pub log: bool,
    pub this_class: String,
    pub module: String,
}

impl TimerItemORM
{
    pub fn new(verbose: bool, log: bool) -> Self 
    {
        Self 
        {
            verbose,
            log,
            this_class: "TimerItemORM".to_string(),
            module: "timer_item".to_string(),
        }
    }

    pub async fn add(&self, db: &DbConn, item: TimerItemActiveModel) -> ModelOutput<TimerItemModel> 
    {
        let this_method = "add";
        if self.verbose { debug!("{}::{} - Starting add operation", self.this_class, this_method); }

        match item.insert(db).await 
        {
            Ok(model) => {
                let output = ModelOutput::success(model, "TimerItem added successfully".to_string());
                if self.verbose { info!("{}::{} - Success: TimerItem added", self.this_class, this_method); }
                if self.log { info!("LOG: {}::{} - TimerItem added", self.this_class, this_method); }
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

    pub async fn items(&self, db: &DbConn, _filters: HashMap<String, String>) -> ModelOutput<Vec<TimerItemModel>> 
    {
        let this_method = "items";
        if self.verbose { debug!("{}::{} - Starting items operation", self.this_class, this_method); }

        match TimerItemEntity::find().all(db).await 
        {
            Ok(items) => 
            {
                let output = ModelOutput::success(items, "TimerItems retrieved successfully".to_string());
                if self.verbose { info!("{}::{} - Success: TimerItems retrieved", self.this_class, this_method); }
                if self.log { info!("LOG: {}::{} - TimerItems retrieved", self.this_class, this_method); }
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

    pub async fn item(&self, db: &DbConn, id: i32) -> ModelOutput<TimerItemModel> 
    {
        let this_method = "item";
        if self.verbose { debug!("{}::{} - Starting item operation for id: {}", self.this_class, this_method, id); }

        match TimerItemEntity::find_by_id(id).one(db).await 
        {
            Ok(Some(model)) => 
            {
                let output = ModelOutput::success(model, "TimerItem retrieved successfully".to_string());
                if self.verbose { info!("{}::{} - Success: TimerItem {} found", self.this_class, this_method, id); }
                if self.log { info!("LOG: {}::{} - TimerItem {} retrieved", self.this_class, this_method, id); }
                output
            }
            Ok(None) => 
            {
                let output = ModelOutput::error("TimerItem not found".to_string());
                if self.verbose { info!("{}::{} - TimerItem {} not found", self.this_class, this_method, id); }
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

    pub async fn update(&self, db: &DbConn, item: TimerItemActiveModel) -> ModelOutput<TimerItemModel> 
    {
        let this_method = "update";
        if self.verbose { debug!("{}::{} - Starting update operation", self.this_class, this_method); }

        match item.update(db).await 
        {
            Ok(model) => 
            {
                let output = ModelOutput::success(model, "TimerItem updated successfully".to_string());
                if self.verbose { info!("{}::{} - Success: TimerItem updated", self.this_class, this_method); }
                if self.log { info!("LOG: {}::{} - TimerItem updated", self.this_class, this_method); }
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

        match TimerItemEntity::delete_by_id(id).exec(db).await 
        {
            Ok(result) => 
            {
                if result.rows_affected > 0 
                {
                    let output = ModelOutput::success("deleted".to_string(), "TimerItem deleted successfully".to_string());
                    if self.verbose { info!("{}::{} - Success: TimerItem {} deleted", self.this_class, this_method, id); }
                    if self.log { info!("LOG: {}::{} - TimerItem {} deleted", self.this_class, this_method, id); }
                    output
                } 
                else 
                {
                    let output = ModelOutput::error("TimerItem not found".to_string());
                    if self.verbose { info!("{}::{} - TimerItem {} not found", self.this_class, this_method, id); }
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
    pub async fn disable(&self, db: &DbConn, id: i32) -> ModelOutput<TimerItemModel>
    {
        let this_method = "disable";
        if self.verbose { debug!("{}::{} - Starting disable operation for id: {}", self.this_class, this_method, id); }

        match TimerItemEntity::find_by_id(id).one(db).await
        {
            Ok(Some(existing)) =>
            {
                let mut active: TimerItemActiveModel = existing.into();
                active.enable = sea_orm::Set(false);

                match active.update(db).await
                {
                    Ok(updated) => ModelOutput::success(updated, "TimerItem disabled successfully".to_string()),
                    Err(e) => {
                        let error_msg = format!("Database error in {}::{}: {}", self.this_class, this_method, e);
                        error!("{}::{} - Error: {}", self.this_class, this_method, error_msg);
                        ModelOutput::error(error_msg)
                    }
                }
            }
            Ok(None) => ModelOutput::error("TimerItem not found".to_string()),
            Err(e) => {
                let error_msg = format!("Database error in {}::{}: {}", self.this_class, this_method, e);
                error!("{}::{} - Error: {}", self.this_class, this_method, error_msg);
                ModelOutput::error(error_msg)
            }
        }
    }

    //-------------------------- [Enable]
    pub async fn enable(&self, db: &DbConn, id: i32) -> ModelOutput<TimerItemModel>
    {
        let this_method = "enable";
        if self.verbose { debug!("{}::{} - Starting enable operation for id: {}", self.this_class, this_method, id); }

        match TimerItemEntity::find_by_id(id).one(db).await
        {
            Ok(Some(existing)) =>
            {
                let mut active: TimerItemActiveModel = existing.into();
                active.enable = sea_orm::Set(true);

                match active.update(db).await
                {
                    Ok(updated) => ModelOutput::success(updated, "TimerItem enabled successfully".to_string()),
                    Err(e) => {
                        let error_msg = format!("Database error in {}::{}: {}", self.this_class, this_method, e);
                        error!("{}::{} - Error: {}", self.this_class, this_method, error_msg);
                        ModelOutput::error(error_msg)
                    }
                }
            }
            Ok(None) => ModelOutput::error("TimerItem not found".to_string()),
            Err(e) => {
                let error_msg = format!("Database error in {}::{}: {}", self.this_class, this_method, e);
                error!("{}::{} - Error: {}", self.this_class, this_method, error_msg);
                ModelOutput::error(error_msg)
            }
        }
    }