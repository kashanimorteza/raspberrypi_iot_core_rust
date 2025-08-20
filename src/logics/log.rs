//--------------------------------------------------------------------------------- Location
// src/logics/log.rs

//--------------------------------------------------------------------------------- Description
// This file contains logic to add sample logs using the ORM

//--------------------------------------------------------------------------------- Import
use crate::orm::logics::log::LogORM;
use sea_orm::DatabaseConnection;
use crate::orm::models::log::ActiveModel as LogActiveModel;
use sea_orm::ActiveValue::Set;

//--------------------------------------------------------------------------------- Add Sample Logs Logic
pub async fn add_sample_logs(db: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> 
{
    let log_orm = LogORM::new(true, true);
    let sample_logs = vec![
        LogActiveModel 
        {
            id: sea_orm::ActiveValue::NotSet,
            date: Set("2024-01-01 10:00:00".to_string()),
            name: Set("System Startup".to_string()),
            status: Set(true),
            data: Set("IoT Core system started successfully".to_string()),
        },
        LogActiveModel 
        {
            id: sea_orm::ActiveValue::NotSet,
            date: Set("2024-01-01 10:05:00".to_string()),
            name: Set("Device Connection".to_string()),
            status: Set(true),
            data: Set("Temperature sensor connected to zone 1".to_string()),
        },
    ];

    println!("🚀 Starting to add {} sample logs...", sample_logs.len());
    
    for (index, log) in sample_logs.into_iter().enumerate() 
    {
        let log_name = match &log.name 
        {
            Set(name) => name.clone(),
            _ => "Unknown".to_string(),
        };
        println!("📝 Adding log {}: {}", index + 1, log_name);
        
        let result = log_orm.add(db, log).await;
        if result.success 
        {
            if let Some(added_log) = result.data 
            {
                println!("✅ Successfully added log: {} (ID: {})", added_log.name, added_log.id);
            }
        } 
        else 
        {
            if let Some(error) = result.error 
            {
                println!("❌ Failed to add log: {}", error);
            }
        }
    }
    
    println!("🎉 Finished adding sample logs!");
    Ok(())
}

//--------------------------------------------------------------------------------- List All Logs Logic
pub async fn list_all_logs(db: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    let log_orm = LogORM::new(true, true);
    let filters = std::collections::HashMap::new();
    
    println!("📋 Fetching all logs...");
    
    let result = log_orm.items(db, filters).await;
    
    if result.success {
        if let Some(logs) = result.data {
            println!("📝 Found {} logs:", logs.len());
            println!("{:-<80}", "");
            for log in logs {
                let status = if log.status { "✅ Success" } else { "❌ Error" };
                println!("ID: {} | Date: {} | Name: {} | Status: {} | Data: {}", 
                    log.id, log.date, log.name, status, log.data);
            }
            println!("{:-<80}", "");
        } else {
            println!("📭 No logs found.");
        }
    } else {
        if let Some(error) = result.error {
            println!("❌ Error fetching logs: {}", error);
        }
    }
    
    Ok(())
}
