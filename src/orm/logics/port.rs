//--------------------------------------------------------------------------------- Location
// src/orm/logics/port.rs

//--------------------------------------------------------------------------------- Description
// ORM logic for port

//--------------------------------------------------------------------------------- Import
use std::collections::HashMap;
use tracing::{info, error, debug};
use sea_orm::{ActiveModelTrait, DbConn, EntityTrait, QueryOrder, QueryFilter, ColumnTrait, Condition};
use crate::orm::models::port::{ActiveModel as PortActiveModel, Entity as PortEntity, Model as PortModel, Column as PortColumn};
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
    //------------------------- New
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

    //------------------------- Add
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

    //------------------------- Items
    pub async fn items(&self, db: &DbConn, filters: HashMap<String, String>) -> ModelOutput<Vec<PortModel>> 
    {
        let this_method = "items";
        if self.verbose { debug!("{}::{} - Starting items operation with filters: {:?}", self.this_class, this_method, filters); }

        // Build query with optional filters
        let mut query = PortEntity::find();
        
        // Apply filters if they exist
        if !filters.is_empty() 
        {
            let mut condition = Condition::all();
            
            // Filter by user_id
            if let Some(user_id_str) = filters.get("user_id") {
                if let Ok(user_id) = user_id_str.parse::<i32>() {
                    condition = condition.add(PortColumn::UserId.eq(user_id));
                    if self.verbose { debug!("{}::{} - Adding filter: user_id = {}", self.this_class, this_method, user_id); }
                }
            }
            
            // Filter by name (partial match)
            if let Some(name) = filters.get("name") {
                condition = condition.add(PortColumn::Name.contains(name));
                if self.verbose { debug!("{}::{} - Adding filter: name contains '{}'", self.this_class, this_method, name); }
            }
            
            // Filter by pin
            if let Some(pin_str) = filters.get("pin") {
                if let Ok(pin) = pin_str.parse::<i32>() {
                    condition = condition.add(PortColumn::Pin.eq(pin));
                    if self.verbose { debug!("{}::{} - Adding filter: pin = {}", self.this_class, this_method, pin); }
                }
            }
            
            // Filter by port
            if let Some(port_str) = filters.get("port") {
                if let Ok(port) = port_str.parse::<i32>() {
                    condition = condition.add(PortColumn::Port.eq(port));
                    if self.verbose { debug!("{}::{} - Adding filter: port = {}", self.this_class, this_method, port); }
                }
            }
            
            // Filter by enable status
            if let Some(enable_str) = filters.get("enable") {
                if let Ok(enable) = enable_str.parse::<bool>() {
                    condition = condition.add(PortColumn::Enable.eq(enable));
                    if self.verbose { debug!("{}::{} - Adding filter: enable = {}", self.this_class, this_method, enable); }
                }
            }
            
            // Filter by protocol (partial match)
            if let Some(protocol) = filters.get("protocol") {
                condition = condition.add(PortColumn::Protocol.contains(protocol));
                if self.verbose { debug!("{}::{} - Adding filter: protocol contains '{}'", self.this_class, this_method, protocol); }
            }
            
            // Filter by type (partial match)
            if let Some(type_val) = filters.get("type") {
                condition = condition.add(PortColumn::Type.contains(type_val));
                if self.verbose { debug!("{}::{} - Adding filter: type contains '{}'", self.this_class, this_method, type_val); }
            }
            
            query = query.filter(condition);
        }
        
        // Execute query with ordering
        match query.order_by_asc(PortColumn::Id).all(db).await 
        {
            Ok(items) => 
            {
                let message = if filters.is_empty() {
                    "All ports retrieved successfully".to_string()
                } else {
                    format!("Filtered ports retrieved successfully (found {} items)", items.len())
                };
                
                let output = ModelOutput::success(items, message);
                if self.verbose { info!("{}::{} - Success: Ports retrieved with filters", self.this_class, this_method); }
                if self.log { info!("LOG: {}::{} - Ports retrieved with filters", self.this_class, this_method); }
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

    //------------------------- Update
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

    //------------------------- Delete
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

    //------------------------- Disable
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

    //------------------------- Enable
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

    //------------------------- Status (Toggle Enable)
    pub async fn status(&self, db: &DbConn, id: i32) -> ModelOutput<PortModel>
    {
        let this_method = "status";

        match PortEntity::find_by_id(id).one(db).await
        {
            Ok(Some(existing)) =>
            {
                // Get the current enable value before moving existing
                let current_enable = existing.enable;
                let mut active: PortActiveModel = existing.into();
                // Toggle the enable field: if true, set to false; if false, set to true
                active.enable = sea_orm::Set(!current_enable);

                match active.update(db).await
                {
                    Ok(updated) => {
                        let message = if current_enable {
                            "Port disabled successfully".to_string()
                        } else {
                            "Port enabled successfully".to_string()
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
            Ok(None) => ModelOutput::error("Port not found".to_string()),
            Err(e) => {
                let error_msg = format!("Database error in {}::{}: {}", self.this_class, this_method, e);
                error!("{}::{} - Error: {}", self.this_class, this_method, error_msg);
                ModelOutput::error(error_msg)
            }
        }
    }
}