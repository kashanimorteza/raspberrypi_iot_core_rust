//--------------------------------------------------------------------------------- Location
// src/orm/logics/port.rs

//--------------------------------------------------------------------------------- Description
// ORM logic for port

//--------------------------------------------------------------------------------- Import
use std::collections::HashMap;
use tracing::{info, error, debug};
use sea_orm::{ActiveModelTrait, DbConn, EntityTrait};
use crate::orm::models::port::{ActiveModel as PortActiveModel, Entity as PortEntity, Model as PortModel};
use crate::logics::general::ModelOutput;

//--------------------------------------------------------------------------------- Class
pub struct PortORM 
{
    pub verbose: bool,
    pub log: bool,
    pub this_class: String,
    pub module: String,
}

impl PortORM
{
    //-------------------------- [Init]
    pub fn new(verbose: bool, log: bool) -> Self 
    {
        Self 
        {
            verbose,
            log,
            this_class: "PortORM".to_string(),
            module: "port".to_string(),
        }
    }

    //-------------------------- [Add]
    pub async fn add(&self, db: &DbConn, item: PortActiveModel) -> ModelOutput<PortModel> 
    {
        let this_method = "add";
        if self.verbose { debug!("{}::{} - Starting add operation", self.this_class, this_method); }

        match item.insert(db).await 
        {
            Ok(model) => {
                let output = ModelOutput::success(model, "Port added successfully".to_string());
                if self.verbose { info!("{}::{} - Success: Port added", self.this_class, this_method); }
                if self.log { info!("LOG: {}::{} - Port added", self.this_class, this_method); }
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
    pub async fn items(&self, db: &DbConn, filters: HashMap<String, String>) -> ModelOutput<Vec<PortModel>> 
    {
        let this_method = "items";
        if self.verbose { debug!("{}::{} - Starting items operation with filters: {:?}", self.this_class, this_method, filters); }

        match PortEntity::find().all(db).await 
        {
            Ok(items) => 
            {
                let output = ModelOutput::success(items, "Ports retrieved successfully".to_string());
                if self.verbose { info!("{}::{} - Success: Ports retrieved", self.this_class, this_method); }
                if self.log { info!("LOG: {}::{} - Ports retrieved", self.this_class, this_method); }
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
    pub async fn item(&self, db: &DbConn, id: i32) -> ModelOutput<PortModel> 
    {
        let this_method = "item";
        if self.verbose { debug!("{}::{} - Starting item operation for id: {}", self.this_class, this_method, id); }

        match PortEntity::find_by_id(id).one(db).await 
        {
            Ok(Some(model)) => 
            {
                let output = ModelOutput::success(model, "Port retrieved successfully".to_string());
                if self.verbose { info!("{}::{} - Success: Port {} found", self.this_class, this_method, id); }
                if self.log { info!("LOG: {}::{} - Port {} retrieved", self.this_class, this_method, id); }
                output
            }
            Ok(None) => 
            {
                let output = ModelOutput::error("Port not found".to_string());
                if self.verbose { info!("{}::{} - Port {} not found", self.this_class, this_method, id); }
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
    pub async fn update(&self, db: &DbConn, item: PortActiveModel) -> ModelOutput<PortModel> 
    {
        let this_method = "update";
        if self.verbose { debug!("{}::{} - Starting update operation", self.this_class, this_method); }

        match item.update(db).await 
        {
            Ok(model) => 
            {
                let output = ModelOutput::success(model, "Port updated successfully".to_string());
                if self.verbose { info!("{}::{} - Success: Port updated", self.this_class, this_method); }
                if self.log { info!("LOG: {}::{} - Port updated", self.this_class, this_method); }
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

        match PortEntity::delete_by_id(id).exec(db).await 
        {
            Ok(result) => 
            {
                if result.rows_affected > 0 
                {
                    let output = ModelOutput::success("deleted".to_string(), "Port deleted successfully".to_string());
                    if self.verbose { info!("{}::{} - Success: Port {} deleted", self.this_class, this_method, id); }
                    if self.log { info!("LOG: {}::{} - Port {} deleted", self.this_class, this_method, id); }
                    output
                } 
                else 
                {
                    let output = ModelOutput::error("Port not found".to_string());
                    if self.verbose { info!("{}::{} - Port {} not found", self.this_class, this_method, id); }
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

impl PortORM
{
    //-------------------------- [Disable]
    pub async fn disable(&self, db: &DbConn, id: i32) -> ModelOutput<PortModel>
    {
        let this_method = "disable";
        if self.verbose { debug!("{}::{} - Starting disable operation for id: {}", self.this_class, this_method, id); }

        match PortEntity::find_by_id(id).one(db).await
        {
            Ok(Some(existing)) =>
            {
                let mut active: PortActiveModel = existing.into();
                active.enable = sea_orm::Set(false);

                match active.update(db).await
                {
                    Ok(updated) =>
                    {
                        let output = ModelOutput::success(updated, "Port disabled successfully".to_string());
                        if self.verbose { info!("{}::{} - Success: Port {} disabled", self.this_class, this_method, id); }
                        if self.log { info!("LOG: {}::{} - Port {} disabled", self.this_class, this_method, id); }
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
                let output = ModelOutput::error("Port not found".to_string());
                if self.verbose { info!("{}::{} - Port {} not found", self.this_class, this_method, id); }
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

    //-------------------------- [Enable]
    pub async fn enable(&self, db: &DbConn, id: i32) -> ModelOutput<PortModel>
    {
        let this_method = "enable";
        if self.verbose { debug!("{}::{} - Starting enable operation for id: {}", self.this_class, this_method, id); }

        match PortEntity::find_by_id(id).one(db).await
        {
            Ok(Some(existing)) =>
            {
                let mut active: PortActiveModel = existing.into();
                active.enable = sea_orm::Set(true);

                match active.update(db).await
                {
                    Ok(updated) =>
                    {
                        let output = ModelOutput::success(updated, "Port enabled successfully".to_string());
                        if self.verbose { info!("{}::{} - Success: Port {} enabled", self.this_class, this_method, id); }
                        if self.log { info!("LOG: {}::{} - Port {} enabled", self.this_class, this_method, id); }
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
                let output = ModelOutput::error("Port not found".to_string());
                if self.verbose { info!("{}::{} - Port {} not found", self.this_class, this_method, id); }
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