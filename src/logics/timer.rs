//--------------------------------------------------------------------------------- Location
// src/logics/timer.rs

//--------------------------------------------------------------------------------- Description
// This file contains logic to add sample timers using the ORM

//--------------------------------------------------------------------------------- Import
use crate::orm::logics::timer::TimerORM;
use sea_orm::DatabaseConnection;
use crate::orm::models::timer::ActiveModel as TimerActiveModel;
use sea_orm::ActiveValue::Set;

//--------------------------------------------------------------------------------- Add Sample Timers Logic
pub async fn add_sample_timers(db: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> 
{
    let timer_orm = TimerORM::new(true, true);
    let sample_timers = vec![
        TimerActiveModel 
        {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("Timer-On".to_string()),
            description: Set("Timer for turning devices on during active periods".to_string()),
            enable: Set(true),
        },
        TimerActiveModel 
        {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("Timer-Off".to_string()),
            description: Set("Timer for turning devices off during inactive periods".to_string()),
            enable: Set(true),
        },
    ];

    println!("🚀 Starting to add {} sample timers...", sample_timers.len());
    
    for (index, timer) in sample_timers.into_iter().enumerate() 
    {
        let timer_name = match &timer.name 
        {
            Set(name) => name.clone(),
            _ => "Unknown".to_string(),
        };
        println!("📝 Adding timer {}: {}", index + 1, timer_name);
        
        let result = timer_orm.add(db, timer).await;
        if result.status 
        {
            if let Some(added_timer) = result.data 
            {
                println!("✅ Successfully added timer: {} (ID: {})", added_timer.name, added_timer.id);
            }
        } 
        else 
        {
            if let Some(error) = result.error 
            {
                println!("❌ Failed to add timer: {}", error);
            }
        }
    }
    
    println!("🎉 Finished adding sample timers!");
    Ok(())
}

//--------------------------------------------------------------------------------- List All Timers Logic
pub async fn list_all_timers(db: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    let timer_orm = TimerORM::new(true, true);
    let filters = std::collections::HashMap::new();
    
    println!("📋 Fetching all timers...");
    
    let result = timer_orm.items(db, filters).await;
    
    if result.status {
        if let Some(timers) = result.data {
            println!("⏰ Found {} timers:", timers.len());
            println!("{:-<80}", "");
            for timer in timers {
                let status = if timer.enable { "✅ Enabled" } else { "❌ Disabled" };
                println!("ID: {} | Name: {} | User: {} | Description: {} | Status: {}", 
                    timer.id, timer.name, timer.user_id, timer.description, status);
            }
            println!("{:-<80}", "");
        } else {
            println!("📭 No timers found.");
        }
    } else {
        if let Some(error) = result.error {
            println!("❌ Error fetching timers: {}", error);
        }
    }
    
    Ok(())
}
