//--------------------------------------------------------------------------------- Location
// src/logics/timer_limit.rs

//--------------------------------------------------------------------------------- Description
// This file contains logic to add sample timer limits using the ORM

//--------------------------------------------------------------------------------- Import
use crate::orm::logics::timer_limit::TimerLimitORM;
use sea_orm::DatabaseConnection;
use crate::orm::models::timer_limit::ActiveModel as TimerLimitActiveModel;
use sea_orm::ActiveValue::Set;

//--------------------------------------------------------------------------------- Add Sample Timer Limits Logic
pub async fn add_sample_timer_limits(db: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> 
{
    let timer_limit_orm = TimerLimitORM::new(true, true);
    let sample_timer_limits = vec![
        // Timer limit for device 9 (Jacuzzi Erjet Motor)
        TimerLimitActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            device_id: Set(9),
            command_from_id: Set(18),
            command_to_id: Set(19),
            value: Set(1),
            description: Set("Timer limit for jacuzzi erjet motor - from on to off with value 1".to_string()),
            enable: Set(true),
        },
        // Timer limit for device 23 (Steam Sauna Temperature Sensor)
        TimerLimitActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            device_id: Set(23),
            command_from_id: Set(52),
            command_to_id: Set(52),
            value: Set(1),
            description: Set("Timer limit for steam sauna temperature sensor - same command with value 1".to_string()),
            enable: Set(true),
        },
    ];

    println!("🚀 Starting to add {} sample timer limits...", sample_timer_limits.len());
    
    for (index, timer_limit) in sample_timer_limits.into_iter().enumerate() 
    {
        let device_id = match &timer_limit.device_id 
        {
            Set(id) => *id,
            _ => 0,
        };
        println!("📝 Adding timer limit {}: Device ID {}", index + 1, device_id);
        
        let result = timer_limit_orm.add(db, timer_limit).await;
        if result.status 
        {
            if let Some(added_limit) = result.data 
            {
                println!("✅ Successfully added timer limit: Device {} (ID: {})", added_limit.device_id, added_limit.id);
            }
        } 
        else 
        {
            if let Some(error) = result.error 
            {
                println!("❌ Failed to add timer limit: {}", error);
            }
        }
    }
    
    println!("🎉 Finished adding sample timer limits!");
    Ok(())
}

//--------------------------------------------------------------------------------- List All Timer Limits Logic
pub async fn list_all_timer_limits(db: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    let timer_limit_orm = TimerLimitORM::new(true, true);
    let filters = std::collections::HashMap::new();
    
    println!("📋 Fetching all timer limits...");
    
    let result = timer_limit_orm.items(db, filters).await;
    
    if result.status {
        if let Some(limits) = result.data {
            println!("⏱️ Found {} timer limits:", limits.len());
            println!("{:-<80}", "");
            for limit in limits {
                let status = if limit.enable { "✅ Enabled" } else { "❌ Disabled" };
                println!("ID: {} | Device: {} | From Command: {} | To Command: {} | Value: {} | Status: {}", 
                    limit.id, limit.device_id, limit.command_from_id, limit.command_to_id, limit.value, status);
            }
            println!("{:-<80}", "");
        } else {
            println!("📭 No timer limits found.");
        }
    } else {
        if let Some(error) = result.error {
            println!("❌ Error fetching timer limits: {}", error);
        }
    }
    
    Ok(())
}
