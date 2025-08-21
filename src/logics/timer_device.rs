//--------------------------------------------------------------------------------- Location
// src/logics/timer_device.rs

//--------------------------------------------------------------------------------- Description
// This file contains logic to add sample timer devices using the ORM

//--------------------------------------------------------------------------------- Import
use crate::orm::logics::timer_device::TimerDeviceORM;
use sea_orm::DatabaseConnection;
use crate::orm::models::timer_device::ActiveModel as TimerDeviceActiveModel;
use sea_orm::ActiveValue::Set;

//--------------------------------------------------------------------------------- Add Sample Timer Devices Logic
pub async fn add_sample_timer_devices(db: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> 
{
    let timer_device_orm = TimerDeviceORM::new(true, true);
    let sample_timer_devices = vec![
        // Timer-On devices (timer_id=1)
        TimerDeviceActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            timer_id: Set(1),
            device_id: Set(1),
            command_id: Set(1),
            description: Set("Timer on for device 1 with command 1".to_string()),
            enable: Set(true),
        },
        TimerDeviceActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            timer_id: Set(1),
            device_id: Set(5),
            command_id: Set(10),
            description: Set("Timer on for device 5 with command 10".to_string()),
            enable: Set(true),
        },
        TimerDeviceActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            timer_id: Set(1),
            device_id: Set(16),
            command_id: Set(35),
            description: Set("Timer on for device 16 with command 35".to_string()),
            enable: Set(true),
        },
        // Timer-Off devices (timer_id=2)
        TimerDeviceActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            timer_id: Set(2),
            device_id: Set(1),
            command_id: Set(2),
            description: Set("Timer off for device 1 with command 2".to_string()),
            enable: Set(true),
        },
        TimerDeviceActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            timer_id: Set(2),
            device_id: Set(5),
            command_id: Set(11),
            description: Set("Timer off for device 5 with command 11".to_string()),
            enable: Set(true),
        },
        TimerDeviceActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            timer_id: Set(2),
            device_id: Set(16),
            command_id: Set(36),
            description: Set("Timer off for device 16 with command 36".to_string()),
            enable: Set(true),
        },
    ];

    println!("🚀 Starting to add {} sample timer devices...", sample_timer_devices.len());
    
    for (index, timer_device) in sample_timer_devices.into_iter().enumerate() 
    {
        let device_id = match &timer_device.device_id 
        {
            Set(id) => *id,
            _ => 0,
        };
        println!("📝 Adding timer device {}: Device ID {}", index + 1, device_id);
        
        let result = timer_device_orm.add(db, timer_device).await;
        if result.success 
        {
            if let Some(added_device) = result.data 
            {
                println!("✅ Successfully added timer device: Device {} (ID: {})", added_device.device_id, added_device.id);
            }
        } 
        else 
        {
            if let Some(error) = result.error 
            {
                println!("❌ Failed to add timer device: {}", error);
            }
        }
    }
    
    println!("🎉 Finished adding sample timer devices!");
    Ok(())
}

//--------------------------------------------------------------------------------- List All Timer Devices Logic
pub async fn list_all_timer_devices(db: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    let timer_device_orm = TimerDeviceORM::new(true, true);
    let filters = std::collections::HashMap::new();
    
    println!("📋 Fetching all timer devices...");
    
    let result = timer_device_orm.items(db, filters).await;
    
    if result.success {
        if let Some(devices) = result.data {
            println!("⏰ Found {} timer devices:", devices.len());
            println!("{:-<80}", "");
            for device in devices {
                let status = if device.enable { "✅ Enabled" } else { "❌ Disabled" };
                println!("ID: {} | Timer: {} | Device: {} | Command: {} | Status: {}", 
                    device.id, device.timer_id, device.device_id, device.command_id, status);
            }
            println!("{:-<80}", "");
        } else {
            println!("📭 No timer devices found.");
        }
    } else {
        if let Some(error) = result.error {
            println!("❌ Error fetching timer devices: {}", error);
        }
    }
    
    Ok(())
}
