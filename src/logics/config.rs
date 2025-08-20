//--------------------------------------------------------------------------------- Location
// src/logics/config.rs

//--------------------------------------------------------------------------------- Description
// This file contains logic to add sample configs using the ORM

//--------------------------------------------------------------------------------- Import
use crate::orm::logics::config::ConfigORM;
use sea_orm::DatabaseConnection;
use crate::orm::models::config::ActiveModel as ConfigActiveModel;
use sea_orm::ActiveValue::Set;

//--------------------------------------------------------------------------------- Add Sample Configs Logic
pub async fn add_sample_configs(db: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> 
{
    let config_orm = ConfigORM::new(true, true);
    let sample_configs = vec![
        ConfigActiveModel 
        {
            id: sea_orm::ActiveValue::NotSet,
            name: Set("Default Config".to_string()),
            time_zone: Set("UTC".to_string()),
            path_api: Set("/api".to_string()),
            path_gui: Set("/gui".to_string()),
            webapi_title: Set("IoT Core API".to_string()),
            webapi_description: Set("Raspberry Pi IoT Core API".to_string()),
            webapi_version: Set("1.0.0".to_string()),
            webapi_openapi_url: Set("/openapi.json".to_string()),
            webapi_docs_url: Set("/docs".to_string()),
            webapi_redoc_url: Set("/redoc".to_string()),
            webapi_key: Set("default-api-key".to_string()),
            webapi_host: Set("0.0.0.0".to_string()),
            webapi_port: Set(Some(8000)),
            webapi_workers: Set(Some(4)),
            nginx_api_host: Set("localhost".to_string()),
            nginx_api_port: Set(Some(80)),
            nginx_api_key: Set("nginx-api-key".to_string()),
            nginx_gui_host: Set("localhost".to_string()),
            nginx_gui_port: Set(Some(3000)),
            nginx_gui_key: Set("nginx-gui-key".to_string()),
            git_email: Set("admin@example.com".to_string()),
            git_name: Set("Admin".to_string()),
            git_key: Set("git-key".to_string()),
            hotspod_ssid: Set("IoT-Hotspot".to_string()),
            hotspod_ip: Set("192.168.4.1".to_string()),
            hotspod_pass: Set("hotspot123".to_string()),
            wifi_ssid: Set("Home-WiFi".to_string()),
            wifi_ip: Set("192.168.1.100".to_string()),
            wifi_pass: Set("wifi123".to_string()),
            debug: Set(true),
            log: Set(true),
            verbose: Set(true),
        },
    ];

    println!("🚀 Starting to add {} sample configs...", sample_configs.len());
    
    for (index, config) in sample_configs.into_iter().enumerate() 
    {
        let config_name = match &config.name 
        {
            Set(name) => name.clone(),
            _ => "Unknown".to_string(),
        };
        println!("📝 Adding config {}: {}", index + 1, config_name);
        
        let result = config_orm.add(db, config).await;
        if result.success 
        {
            if let Some(added_config) = result.data 
            {
                println!("✅ Successfully added config: {} (ID: {})", added_config.name, added_config.id);
            }
        } 
        else 
        {
            if let Some(error) = result.error 
            {
                println!("❌ Failed to add config: {}", error);
            }
        }
    }
    
    println!("🎉 Finished adding sample configs!");
    Ok(())
}

//--------------------------------------------------------------------------------- List All Configs Logic
pub async fn list_all_configs(db: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    let config_orm = ConfigORM::new(true, true);
    let filters = std::collections::HashMap::new();
    
    println!("📋 Fetching all configs...");
    
    let result = config_orm.items(db, filters).await;
    
    if result.success {
        if let Some(configs) = result.data {
            println!("⚙️ Found {} configs:", configs.len());
            println!("{:-<80}", "");
            for config in configs {
                let debug_status = if config.debug { "✅ Debug" } else { "❌ No Debug" };
                println!("ID: {} | Name: {} | API Host: {} | API Port: {} | Status: {}", 
                    config.id, config.name, config.webapi_host, config.webapi_port.unwrap_or(0), debug_status);
            }
            println!("{:-<80}", "");
        } else {
            println!("📭 No configs found.");
        }
    } else {
        if let Some(error) = result.error {
            println!("❌ Error fetching configs: {}", error);
        }
    }
    
    Ok(())
}
