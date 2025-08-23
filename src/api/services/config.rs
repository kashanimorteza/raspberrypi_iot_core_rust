//--------------------------------------------------------------------------------- Location
// src/api/services/config.rs

//--------------------------------------------------------------------------------- Description
// This is service for config - Rust equivalent of Python service layer

//--------------------------------------------------------------------------------- Import
use sea_orm::{DatabaseConnection, Set};
use std::collections::HashMap;
use crate::orm::models::config::{Model as ConfigModel, ActiveModel as ConfigActiveModel};
use crate::logics::general::ModelOutput;
use crate::orm::logics::config::ConfigORM;

//--------------------------------------------------------------------------------- Service
pub struct ConfigService 
{
    pub logic: ConfigORM,
}

impl ConfigService 
{
    //------------------------- New
    pub fn new() -> Self 
    {
        Self 
        {
            logic: ConfigORM::new(true, true), // verbose=true, log=true
        }
    }

    //------------------------- Items
    pub async fn items(&self, db: &DatabaseConnection, filters: HashMap<String, String>) -> ModelOutput<Vec<ConfigModel>> 
    {
        self.logic.items(db, filters).await
    }

    //------------------------- Item
    pub async fn item(&self, db: &DatabaseConnection, id: i32) -> ModelOutput<ConfigModel> 
    {
        self.logic.item(db, id).await
    }

    //------------------------- Enable
    pub async fn enable(&self, _db: &DatabaseConnection, _id: i32) -> ModelOutput<ConfigModel> 
    {
        ModelOutput::error("Config does not support enable/disable functionality".to_string())
    }

    //------------------------- Disable
    pub async fn disable(&self, _db: &DatabaseConnection, _id: i32) -> ModelOutput<ConfigModel> 
    {
        ModelOutput::error("Config does not support enable/disable functionality".to_string())
    }

    //------------------------- Update
    pub async fn update(&self, db: &DatabaseConnection, item: ConfigModel) -> ModelOutput<ConfigModel> 
    {
        let active_config = ConfigActiveModel 
        {
            id: Set(item.id),
            name: Set(item.name),
            time_zone: Set(item.time_zone),
            path_api: Set(item.path_api),
            path_gui: Set(item.path_gui),
            webapi_title: Set(item.webapi_title),
            webapi_description: Set(item.webapi_description),
            webapi_version: Set(item.webapi_version),
            webapi_openapi_url: Set(item.webapi_openapi_url),
            webapi_docs_url: Set(item.webapi_docs_url),
            webapi_redoc_url: Set(item.webapi_redoc_url),
            webapi_key: Set(item.webapi_key),
            webapi_host: Set(item.webapi_host),
            webapi_port: Set(item.webapi_port),
            webapi_workers: Set(item.webapi_workers),
            nginx_api_host: Set(item.nginx_api_host),
            nginx_api_port: Set(item.nginx_api_port),
            nginx_api_key: Set(item.nginx_api_key),
            nginx_gui_host: Set(item.nginx_gui_host),
            nginx_gui_port: Set(item.nginx_gui_port),
            nginx_gui_key: Set(item.nginx_gui_key),
            git_email: Set(item.git_email),
            git_name: Set(item.git_name),
            git_key: Set(item.git_key),
            hotspod_ssid: Set(item.hotspod_ssid),
            hotspod_ip: Set(item.hotspod_ip),
            hotspod_pass: Set(item.hotspod_pass),
            wifi_ssid: Set(item.wifi_ssid),
            wifi_ip: Set(item.wifi_ip),
            wifi_pass: Set(item.wifi_pass),
            debug: Set(item.debug),
            log: Set(item.log),
            verbose: Set(item.verbose),
        };

        self.logic.update(db, active_config).await
    }

    //------------------------- Add
    pub async fn add(&self, db: &DatabaseConnection, item: ConfigModel) -> ModelOutput<ConfigModel> 
    {
        let active_config = ConfigActiveModel 
        {
            id: Default::default(),
            name: Set(item.name),
            time_zone: Set(item.time_zone),
            path_api: Set(item.path_api),
            path_gui: Set(item.path_gui),
            webapi_title: Set(item.webapi_title),
            webapi_description: Set(item.webapi_description),
            webapi_version: Set(item.webapi_version),
            webapi_openapi_url: Set(item.webapi_openapi_url),
            webapi_docs_url: Set(item.webapi_docs_url),
            webapi_redoc_url: Set(item.webapi_redoc_url),
            webapi_key: Set(item.webapi_key),
            webapi_host: Set(item.webapi_host),
            webapi_port: Set(item.webapi_port),
            webapi_workers: Set(item.webapi_workers),
            nginx_api_host: Set(item.nginx_api_host),
            nginx_api_port: Set(item.nginx_api_port),
            nginx_api_key: Set(item.nginx_api_key),
            nginx_gui_host: Set(item.nginx_gui_host),
            nginx_gui_port: Set(item.nginx_gui_port),
            nginx_gui_key: Set(item.nginx_gui_key),
            git_email: Set(item.git_email),
            git_name: Set(item.git_name),
            git_key: Set(item.git_key),
            hotspod_ssid: Set(item.hotspod_ssid),
            hotspod_ip: Set(item.hotspod_ip),
            hotspod_pass: Set(item.hotspod_pass),
            wifi_ssid: Set(item.wifi_ssid),
            wifi_ip: Set(item.wifi_ip),
            wifi_pass: Set(item.wifi_pass),
            debug: Set(item.debug),
            log: Set(item.log),
            verbose: Set(item.verbose),
        };

        self.logic.add(db, active_config).await
    }

    //------------------------- Delete
    pub async fn delete(&self, db: &DatabaseConnection, id: i32) -> ModelOutput<String> 
    {
        self.logic.delete(db, id).await
    }

    //------------------------- Status
    pub async fn status(&self, db: &DatabaseConnection, id: i32) -> ModelOutput<ConfigModel> 
    {
        self.logic.status(db, id).await
    }
}
