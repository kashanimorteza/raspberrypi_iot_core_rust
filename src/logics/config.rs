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
            name: Set("raspberrypi".to_string()),
            time_zone: Set("Asia/Tehran".to_string()),
            path_api: Set("/root/raspberrypi_iot_core_rust".to_string()),
            path_gui: Set("/root/raspberrypi_iot_gui_flutter".to_string()),
            webapi_title: Set("raspberrypi_iot_core_rust".to_string()),
            webapi_description: Set("raspberrypi_iot_core_rust".to_string()),
            webapi_version: Set("1.0.0".to_string()),
            webapi_openapi_url: Set("schema".to_string()),
            webapi_docs_url: Set("doc1".to_string()),
            webapi_redoc_url: Set("doc2".to_string()),
            webapi_key: Set("".to_string()),
            webapi_host: Set("127.0.0.1".to_string()),
            webapi_port: Set(Some(8000)),
            webapi_workers: Set(Some(1)),
            nginx_api_host: Set("0.0.0.0".to_string()),
            nginx_api_port: Set(Some(8001)),
            nginx_api_key: Set("218df3fa67ee44bf9a60dccc2cc71ce3".to_string()),
            nginx_gui_host: Set("0.0.0.0".to_string()),
            nginx_gui_port: Set(Some(8002)),
            nginx_gui_key: Set("218df3fa67ee44bf9a60dccc2cc71ce3".to_string()),
            git_email: Set("kashani.morteza@gmail.com".to_string()),
            git_name: Set("morteza".to_string()),
            git_key: Set("git_key".to_string()),
            hotspod_ssid: Set("raspberrypi".to_string()),
            hotspod_ip: Set("192.168.1.111".to_string()),
            hotspod_pass: Set("Morteza1001110".to_string()),
            wifi_ssid: Set("Mori-Android".to_string()),
            wifi_ip: Set("192.168.1.111".to_string()),
            wifi_pass: Set("Morteza1001110".to_string()),
            debug: Set(false),
            log: Set(false),
            verbose: Set(false),
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
        if result.status 
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
    
    if result.status {
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
