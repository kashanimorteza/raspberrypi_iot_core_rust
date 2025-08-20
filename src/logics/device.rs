//--------------------------------------------------------------------------------- Location
// src/logics/device.rs

//--------------------------------------------------------------------------------- Description
// This file contains logic to add sample devices using the ORM

//--------------------------------------------------------------------------------- Import
use crate::orm::logics::device::DeviceORM;
use sea_orm::DatabaseConnection;
use crate::orm::models::device::ActiveModel as DeviceActiveModel;
use sea_orm::ActiveValue::Set;

//--------------------------------------------------------------------------------- Add Sample Devices Logic
pub async fn add_sample_devices(db: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> 
{
    let device_orm = DeviceORM::new(true, true);
    let sample_devices = vec![
        DeviceActiveModel 
        {
            id: sea_orm::ActiveValue::NotSet,
            zone_id: Set(1),
            port_id: Set(1),
            power_id: Set(1),
            command_id: Set(1),
            value: Set(0),
            tune: Set(0),
            date: Set("2024-01-01".to_string()),
            address: Set("192.168.1.10".to_string()),
            name: Set("Temperature Sensor".to_string()),
            description: Set("DHT22 Temperature and Humidity Sensor".to_string()),
            enable: Set(true),
        },
        DeviceActiveModel 
        {
            id: sea_orm::ActiveValue::NotSet,
            zone_id: Set(1),
            port_id: Set(2),
            power_id: Set(1),
            command_id: Set(2),
            value: Set(0),
            tune: Set(0),
            date: Set("2024-01-01".to_string()),
            address: Set("192.168.1.11".to_string()),
            name: Set("LED Strip".to_string()),
            description: Set("RGB LED Strip Controller".to_string()),
            enable: Set(true),
        },
    ];

    println!("🚀 Starting to add {} sample devices...", sample_devices.len());
    
    for (index, device) in sample_devices.into_iter().enumerate() 
    {
        let device_name = match &device.name 
        {
            Set(name) => name.clone(),
            _ => "Unknown".to_string(),
        };
        println!("📝 Adding device {}: {}", index + 1, device_name);
        
        let result = device_orm.add(db, device).await;
        if result.success 
        {
            if let Some(added_device) = result.data 
            {
                println!("✅ Successfully added device: {} (ID: {})", added_device.name, added_device.id);
            }
        } 
        else 
        {
            if let Some(error) = result.error 
            {
                println!("❌ Failed to add device: {}", error);
            }
        }
    }
    
    println!("🎉 Finished adding sample devices!");
    Ok(())
}

//--------------------------------------------------------------------------------- List All Devices Logic
pub async fn list_all_devices(db: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    let device_orm = DeviceORM::new(true, true);
    let filters = std::collections::HashMap::new();
    
    println!("📋 Fetching all devices...");
    
    let result = device_orm.items(db, filters).await;
    
    if result.success {
        if let Some(devices) = result.data {
            println!("🔌 Found {} devices:", devices.len());
            println!("{:-<80}", "");
            for device in devices {
                let status = if device.enable { "✅ Enabled" } else { "❌ Disabled" };
                println!("ID: {} | Name: {} | Address: {} | Zone: {} | Status: {}", 
                    device.id, device.name, device.address, device.zone_id, status);
            }
            println!("{:-<80}", "");
        } else {
            println!("📭 No devices found.");
        }
    } else {
        if let Some(error) = result.error {
            println!("❌ Error fetching devices: {}", error);
        }
    }
    
    Ok(())
}
