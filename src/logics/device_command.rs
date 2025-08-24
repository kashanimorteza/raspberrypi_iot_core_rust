//--------------------------------------------------------------------------------- Location
// src/logics/device_command.rs

//--------------------------------------------------------------------------------- Description
// This file contains logic to add sample device commands using the ORM

//--------------------------------------------------------------------------------- Import
use crate::orm::logics::device_command::DeviceCommandORM;
use sea_orm::DatabaseConnection;
use crate::orm::models::device_command::ActiveModel as DeviceCommandActiveModel;
use sea_orm::ActiveValue::Set;

//--------------------------------------------------------------------------------- Add Sample Device Commands Logic
pub async fn add_sample_device_commands(db: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> 
{
    let device_command_orm = DeviceCommandORM::new(true, true);
    let sample_commands = vec![
        // None command
        DeviceCommandActiveModel {
            id: Set(0),
            device_id: Set(0),
            name: Set("none".to_string()),
            value_from: Set(Some(0)),
            value_to: Set(Some(0)),
            delay: Set(Some(0)),
            description: Set("None command".to_string()),
            reload: Set(false),
            enable: Set(true),
            r#type: Set("NONE".to_string()),
        },
        // Pool Motor commands
        DeviceCommandActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            device_id: Set(1),
            name: Set("on".to_string()),
            value_from: Set(Some(1)),
            value_to: Set(Some(1)),
            delay: Set(Some(0)),
            description: Set("Turn on pool motor".to_string()),
            reload: Set(false),
            enable: Set(true),
            r#type: Set("NONE".to_string()),
        },
        DeviceCommandActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            device_id: Set(1),
            name: Set("off".to_string()),
            value_from: Set(Some(0)),
            value_to: Set(Some(0)),
            delay: Set(Some(0)),
            description: Set("Turn off pool motor".to_string()),
            reload: Set(true),
            enable: Set(true),
            r#type: Set("NONE".to_string()),
        },
        // Pool Shir commands
        DeviceCommandActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            device_id: Set(2),
            name: Set("on".to_string()),
            value_from: Set(Some(1)),
            value_to: Set(Some(1)),
            delay: Set(Some(0)),
            description: Set("Turn on pool shir".to_string()),
            reload: Set(false),
            enable: Set(true),
            r#type: Set("NONE".to_string()),
        },
        DeviceCommandActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            device_id: Set(2),
            name: Set("off".to_string()),
            value_from: Set(Some(0)),
            value_to: Set(Some(0)),
            delay: Set(Some(0)),
            description: Set("Turn off pool shir".to_string()),
            reload: Set(true),
            enable: Set(true),
            r#type: Set("NONE".to_string()),
        },
        // Pool Light commands
        DeviceCommandActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            device_id: Set(3),
            name: Set("on".to_string()),
            value_from: Set(Some(1)),
            value_to: Set(Some(1)),
            delay: Set(Some(0)),
            description: Set("Turn on pool light".to_string()),
            reload: Set(false),
            enable: Set(true),
            r#type: Set("NONE".to_string()),
        },
        DeviceCommandActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            device_id: Set(3),
            name: Set("off".to_string()),
            value_from: Set(Some(0)),
            value_to: Set(Some(0)),
            delay: Set(Some(0)),
            description: Set("Turn off pool light".to_string()),
            reload: Set(true),
            enable: Set(true),
            r#type: Set("NONE".to_string()),
        },
        // Pool Sensor commands
        DeviceCommandActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            device_id: Set(4),
            name: Set("on".to_string()),
            value_from: Set(Some(0)),
            value_to: Set(Some(30)),
            delay: Set(Some(0)),
            description: Set("Pool sensor on condition".to_string()),
            reload: Set(false),
            enable: Set(true),
            r#type: Set("DTU".to_string()),
        },
        DeviceCommandActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            device_id: Set(4),
            name: Set("off".to_string()),
            value_from: Set(Some(30)),
            value_to: Set(Some(1000)),
            delay: Set(Some(0)),
            description: Set("Pool sensor off condition".to_string()),
            reload: Set(true),
            enable: Set(true),
            r#type: Set("NONE".to_string()),
        },
        DeviceCommandActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            device_id: Set(4),
            name: Set("re-on".to_string()),
            value_from: Set(Some(0)),
            value_to: Set(Some(25)),
            delay: Set(Some(0)),
            description: Set("Pool sensor restart condition".to_string()),
            reload: Set(false),
            enable: Set(true),
            r#type: Set("UTD".to_string()),
        },
        // Jacuzzi (devices 5-9): simple on/off
        // Device 5
        DeviceCommandActiveModel { id: sea_orm::ActiveValue::NotSet, device_id: Set(5), name: Set("on".to_string()),  value_from: Set(Some(1)), value_to: Set(Some(1)), delay: Set(Some(0)), description: Set("Jacuzzi device 5 on".to_string()),  reload: Set(false), enable: Set(true), r#type: Set("NONE".to_string()) },
        DeviceCommandActiveModel { id: sea_orm::ActiveValue::NotSet, device_id: Set(5), name: Set("off".to_string()), value_from: Set(Some(0)), value_to: Set(Some(0)), delay: Set(Some(0)), description: Set("Jacuzzi device 5 off".to_string()), reload: Set(true),  enable: Set(true), r#type: Set("NONE".to_string()) },
        // Device 6
        DeviceCommandActiveModel { id: sea_orm::ActiveValue::NotSet, device_id: Set(6), name: Set("on".to_string()),  value_from: Set(Some(1)), value_to: Set(Some(1)), delay: Set(Some(0)), description: Set("Jacuzzi device 6 on".to_string()),  reload: Set(false), enable: Set(true), r#type: Set("NONE".to_string()) },
        DeviceCommandActiveModel { id: sea_orm::ActiveValue::NotSet, device_id: Set(6), name: Set("off".to_string()), value_from: Set(Some(0)), value_to: Set(Some(0)), delay: Set(Some(0)), description: Set("Jacuzzi device 6 off".to_string()), reload: Set(true),  enable: Set(true), r#type: Set("NONE".to_string()) },
        // Device 7
        DeviceCommandActiveModel { id: sea_orm::ActiveValue::NotSet, device_id: Set(7), name: Set("on".to_string()),  value_from: Set(Some(1)), value_to: Set(Some(1)), delay: Set(Some(0)), description: Set("Jacuzzi device 7 on".to_string()),  reload: Set(false), enable: Set(true), r#type: Set("NONE".to_string()) },
        DeviceCommandActiveModel { id: sea_orm::ActiveValue::NotSet, device_id: Set(7), name: Set("off".to_string()), value_from: Set(Some(0)), value_to: Set(Some(0)), delay: Set(Some(0)), description: Set("Jacuzzi device 7 off".to_string()), reload: Set(true),  enable: Set(true), r#type: Set("NONE".to_string()) },
        // Device 8
        DeviceCommandActiveModel { id: sea_orm::ActiveValue::NotSet, device_id: Set(8), name: Set("on".to_string()),  value_from: Set(Some(1)), value_to: Set(Some(1)), delay: Set(Some(0)), description: Set("Jacuzzi device 8 on".to_string()),  reload: Set(false), enable: Set(true), r#type: Set("NONE".to_string()) },
        DeviceCommandActiveModel { id: sea_orm::ActiveValue::NotSet, device_id: Set(8), name: Set("off".to_string()), value_from: Set(Some(0)), value_to: Set(Some(0)), delay: Set(Some(0)), description: Set("Jacuzzi device 8 off".to_string()), reload: Set(true),  enable: Set(true), r#type: Set("NONE".to_string()) },
        // Device 9
        DeviceCommandActiveModel { id: sea_orm::ActiveValue::NotSet, device_id: Set(9), name: Set("on".to_string()),  value_from: Set(Some(1)), value_to: Set(Some(1)), delay: Set(Some(0)), description: Set("Jacuzzi device 9 on".to_string()),  reload: Set(false), enable: Set(true), r#type: Set("NONE".to_string()) },
        DeviceCommandActiveModel { id: sea_orm::ActiveValue::NotSet, device_id: Set(9), name: Set("off".to_string()), value_from: Set(Some(0)), value_to: Set(Some(0)), delay: Set(Some(0)), description: Set("Jacuzzi device 9 off".to_string()), reload: Set(true),  enable: Set(true), r#type: Set("NONE".to_string()) },
        // Device 10: on/off/re-on with ranges
        DeviceCommandActiveModel { id: sea_orm::ActiveValue::NotSet, device_id: Set(10), name: Set("on".to_string()),    value_from: Set(Some(0)),  value_to: Set(Some(50)),   delay: Set(Some(0)), description: Set("Device 10 on range".to_string()),    reload: Set(false), enable: Set(true), r#type: Set("DTU".to_string()) },
        DeviceCommandActiveModel { id: sea_orm::ActiveValue::NotSet, device_id: Set(10), name: Set("off".to_string()),   value_from: Set(Some(50)), value_to: Set(Some(1000)), delay: Set(Some(0)), description: Set("Device 10 off range".to_string()),   reload: Set(true),  enable: Set(true), r#type: Set("NONE".to_string()) },
        DeviceCommandActiveModel { id: sea_orm::ActiveValue::NotSet, device_id: Set(10), name: Set("re-on".to_string()), value_from: Set(Some(0)),  value_to: Set(Some(47)),   delay: Set(Some(0)), description: Set("Device 10 re-on range".to_string()), reload: Set(false), enable: Set(true), r#type: Set("UTD".to_string()) },
        // Souna-Dry: 11,12 simple; 13 with ranges
        // Device 11
        DeviceCommandActiveModel { id: sea_orm::ActiveValue::NotSet, device_id: Set(11), name: Set("on".to_string()),  value_from: Set(Some(1)), value_to: Set(Some(1)), delay: Set(Some(0)), description: Set("Souna-Dry device 11 on".to_string()),  reload: Set(false), enable: Set(true), r#type: Set("NONE".to_string()) },
        DeviceCommandActiveModel { id: sea_orm::ActiveValue::NotSet, device_id: Set(11), name: Set("off".to_string()), value_from: Set(Some(0)), value_to: Set(Some(0)), delay: Set(Some(0)), description: Set("Souna-Dry device 11 off".to_string()), reload: Set(true),  enable: Set(true), r#type: Set("NONE".to_string()) },
        // Device 12
        DeviceCommandActiveModel { id: sea_orm::ActiveValue::NotSet, device_id: Set(12), name: Set("on".to_string()),  value_from: Set(Some(1)), value_to: Set(Some(1)), delay: Set(Some(0)), description: Set("Souna-Dry device 12 on".to_string()),  reload: Set(false), enable: Set(true), r#type: Set("NONE".to_string()) },
        DeviceCommandActiveModel { id: sea_orm::ActiveValue::NotSet, device_id: Set(12), name: Set("off".to_string()), value_from: Set(Some(0)), value_to: Set(Some(0)), delay: Set(Some(0)), description: Set("Souna-Dry device 12 off".to_string()), reload: Set(true),  enable: Set(true), r#type: Set("NONE".to_string()) },
        // Device 13 with DTU/UTD
        DeviceCommandActiveModel { id: sea_orm::ActiveValue::NotSet, device_id: Set(13), name: Set("on".to_string()),    value_from: Set(Some(0)),  value_to: Set(Some(60)),   delay: Set(Some(0)), description: Set("Device 13 on range".to_string()),    reload: Set(false), enable: Set(true), r#type: Set("DTU".to_string()) },
        DeviceCommandActiveModel { id: sea_orm::ActiveValue::NotSet, device_id: Set(13), name: Set("off".to_string()),   value_from: Set(Some(60)), value_to: Set(Some(1000)), delay: Set(Some(0)), description: Set("Device 13 off range".to_string()),   reload: Set(true),  enable: Set(true), r#type: Set("NONE".to_string()) },
        DeviceCommandActiveModel { id: sea_orm::ActiveValue::NotSet, device_id: Set(13), name: Set("re-on".to_string()), value_from: Set(Some(0)),  value_to: Set(Some(55)),   delay: Set(Some(0)), description: Set("Device 13 re-on range".to_string()), reload: Set(false), enable: Set(true), r#type: Set("UTD".to_string()) },
        // Fan: 14 simple; 15 with ranges reversed
        // Device 14
        DeviceCommandActiveModel { id: sea_orm::ActiveValue::NotSet, device_id: Set(14), name: Set("on".to_string()),  value_from: Set(Some(1)), value_to: Set(Some(1)), delay: Set(Some(0)), description: Set("Fan device 14 on".to_string()),  reload: Set(false), enable: Set(true), r#type: Set("NONE".to_string()) },
        DeviceCommandActiveModel { id: sea_orm::ActiveValue::NotSet, device_id: Set(14), name: Set("off".to_string()), value_from: Set(Some(0)), value_to: Set(Some(0)), delay: Set(Some(0)), description: Set("Fan device 14 off".to_string()), reload: Set(true),  enable: Set(true), r#type: Set("NONE".to_string()) },
        // Device 15
        DeviceCommandActiveModel { id: sea_orm::ActiveValue::NotSet, device_id: Set(15), name: Set("on".to_string()),    value_from: Set(Some(100)), value_to: Set(Some(20)), delay: Set(Some(0)), description: Set("Device 15 on DTU".to_string()),    reload: Set(false), enable: Set(true), r#type: Set("DTU".to_string()) },
        DeviceCommandActiveModel { id: sea_orm::ActiveValue::NotSet, device_id: Set(15), name: Set("off".to_string()),   value_from: Set(Some(20)),  value_to: Set(Some(0)),  delay: Set(Some(0)), description: Set("Device 15 off".to_string()),     reload: Set(true),  enable: Set(true), r#type: Set("NONE".to_string()) },
        DeviceCommandActiveModel { id: sea_orm::ActiveValue::NotSet, device_id: Set(15), name: Set("re-on".to_string()), value_from: Set(Some(100)), value_to: Set(Some(23)), delay: Set(Some(0)), description: Set("Device 15 re-on UTD".to_string()), reload: Set(false), enable: Set(true), r#type: Set("UTD".to_string()) },
        // Cold: 16,17 simple on/off
        // Device 16
        DeviceCommandActiveModel { id: sea_orm::ActiveValue::NotSet, device_id: Set(16), name: Set("on".to_string()),  value_from: Set(Some(1)), value_to: Set(Some(1)), delay: Set(Some(0)), description: Set("Cold device 16 on".to_string()),  reload: Set(false), enable: Set(true), r#type: Set("NONE".to_string()) },
        DeviceCommandActiveModel { id: sea_orm::ActiveValue::NotSet, device_id: Set(16), name: Set("off".to_string()), value_from: Set(Some(0)), value_to: Set(Some(0)), delay: Set(Some(0)), description: Set("Cold device 16 off".to_string()), reload: Set(true),  enable: Set(true), r#type: Set("NONE".to_string()) },
        // Device 17
        DeviceCommandActiveModel { id: sea_orm::ActiveValue::NotSet, device_id: Set(17), name: Set("on".to_string()),  value_from: Set(Some(1)), value_to: Set(Some(1)), delay: Set(Some(0)), description: Set("Cold device 17 on".to_string()),  reload: Set(false), enable: Set(true), r#type: Set("NONE".to_string()) },
        DeviceCommandActiveModel { id: sea_orm::ActiveValue::NotSet, device_id: Set(17), name: Set("off".to_string()), value_from: Set(Some(0)), value_to: Set(Some(0)), delay: Set(Some(0)), description: Set("Cold device 17 off".to_string()), reload: Set(true),  enable: Set(true), r#type: Set("NONE".to_string()) },
        // Souna-Steam: 18..22 simple on/off
        // Device 18
        DeviceCommandActiveModel { id: sea_orm::ActiveValue::NotSet, device_id: Set(18), name: Set("on".to_string()),  value_from: Set(Some(1)), value_to: Set(Some(1)), delay: Set(Some(0)), description: Set("Souna-Steam device 18 on".to_string()),  reload: Set(false), enable: Set(true), r#type: Set("NONE".to_string()) },
        DeviceCommandActiveModel { id: sea_orm::ActiveValue::NotSet, device_id: Set(18), name: Set("off".to_string()), value_from: Set(Some(0)), value_to: Set(Some(0)), delay: Set(Some(0)), description: Set("Souna-Steam device 18 off".to_string()), reload: Set(true),  enable: Set(true), r#type: Set("NONE".to_string()) },
        // Device 19
        DeviceCommandActiveModel { id: sea_orm::ActiveValue::NotSet, device_id: Set(19), name: Set("on".to_string()),  value_from: Set(Some(1)), value_to: Set(Some(1)), delay: Set(Some(0)), description: Set("Souna-Steam device 19 on".to_string()),  reload: Set(false), enable: Set(true), r#type: Set("NONE".to_string()) },
        DeviceCommandActiveModel { id: sea_orm::ActiveValue::NotSet, device_id: Set(19), name: Set("off".to_string()), value_from: Set(Some(0)), value_to: Set(Some(0)), delay: Set(Some(0)), description: Set("Souna-Steam device 19 off".to_string()), reload: Set(true),  enable: Set(true), r#type: Set("NONE".to_string()) },
        // Device 20
        DeviceCommandActiveModel { id: sea_orm::ActiveValue::NotSet, device_id: Set(20), name: Set("on".to_string()),  value_from: Set(Some(1)), value_to: Set(Some(1)), delay: Set(Some(0)), description: Set("Souna-Steam device 20 on".to_string()),  reload: Set(false), enable: Set(true), r#type: Set("NONE".to_string()) },
        DeviceCommandActiveModel { id: sea_orm::ActiveValue::NotSet, device_id: Set(20), name: Set("off".to_string()), value_from: Set(Some(0)), value_to: Set(Some(0)), delay: Set(Some(0)), description: Set("Souna-Steam device 20 off".to_string()), reload: Set(true),  enable: Set(true), r#type: Set("NONE".to_string()) },
        // Device 21
        DeviceCommandActiveModel { id: sea_orm::ActiveValue::NotSet, device_id: Set(21), name: Set("on".to_string()),  value_from: Set(Some(1)), value_to: Set(Some(1)), delay: Set(Some(0)), description: Set("Souna-Steam device 21 on".to_string()),  reload: Set(false), enable: Set(true), r#type: Set("NONE".to_string()) },
        DeviceCommandActiveModel { id: sea_orm::ActiveValue::NotSet, device_id: Set(21), name: Set("off".to_string()), value_from: Set(Some(0)), value_to: Set(Some(0)), delay: Set(Some(0)), description: Set("Souna-Steam device 21 off".to_string()), reload: Set(true),  enable: Set(true), r#type: Set("NONE".to_string()) },
        // Device 22
        DeviceCommandActiveModel { id: sea_orm::ActiveValue::NotSet, device_id: Set(22), name: Set("on".to_string()),  value_from: Set(Some(1)), value_to: Set(Some(1)), delay: Set(Some(0)), description: Set("Souna-Steam device 22 on".to_string()),  reload: Set(false), enable: Set(true), r#type: Set("NONE".to_string()) },
        DeviceCommandActiveModel { id: sea_orm::ActiveValue::NotSet, device_id: Set(22), name: Set("off".to_string()), value_from: Set(Some(0)), value_to: Set(Some(0)), delay: Set(Some(0)), description: Set("Souna-Steam device 22 off".to_string()), reload: Set(true),  enable: Set(true), r#type: Set("NONE".to_string()) },
        // Device 23 with ranges
        DeviceCommandActiveModel { id: sea_orm::ActiveValue::NotSet, device_id: Set(23), name: Set("on".to_string()),    value_from: Set(Some(0)),  value_to: Set(Some(60)),   delay: Set(Some(0)), description: Set("Device 23 on range".to_string()),    reload: Set(false), enable: Set(true), r#type: Set("DTU".to_string()) },
        DeviceCommandActiveModel { id: sea_orm::ActiveValue::NotSet, device_id: Set(23), name: Set("off".to_string()),   value_from: Set(Some(60)), value_to: Set(Some(1000)), delay: Set(Some(0)), description: Set("Device 23 off range".to_string()),   reload: Set(true),  enable: Set(true), r#type: Set("NONE".to_string()) },
        DeviceCommandActiveModel { id: sea_orm::ActiveValue::NotSet, device_id: Set(23), name: Set("re-on".to_string()), value_from: Set(Some(0)),  value_to: Set(Some(55)),   delay: Set(Some(0)), description: Set("Device 23 re-on range".to_string()), reload: Set(false), enable: Set(true), r#type: Set("UTD".to_string()) },
        // Device 24 on/off and on-off with delay
        DeviceCommandActiveModel { id: sea_orm::ActiveValue::NotSet, device_id: Set(24), name: Set("on".to_string()),     value_from: Set(Some(1)), value_to: Set(Some(1)), delay: Set(Some(0)),    description: Set("Device 24 on".to_string()),     reload: Set(false), enable: Set(true), r#type: Set("NONE".to_string()) },
        DeviceCommandActiveModel { id: sea_orm::ActiveValue::NotSet, device_id: Set(24), name: Set("off".to_string()),    value_from: Set(Some(0)), value_to: Set(Some(0)), delay: Set(Some(0)),    description: Set("Device 24 off".to_string()),    reload: Set(false), enable: Set(true), r#type: Set("NONE".to_string()) },
        DeviceCommandActiveModel { id: sea_orm::ActiveValue::NotSet, device_id: Set(24), name: Set("on-off".to_string()), value_from: Set(Some(1)), value_to: Set(Some(0)), delay: Set(Some(3000)), description: Set("Device 24 on-off".to_string()), reload: Set(true),  enable: Set(true), r#type: Set("NONE".to_string()) },
        // Abnama: device 25 and 26 simple
        DeviceCommandActiveModel { id: sea_orm::ActiveValue::NotSet, device_id: Set(25), name: Set("on".to_string()),  value_from: Set(Some(1)), value_to: Set(Some(1)), delay: Set(Some(0)), description: Set("Abnama device 25 on".to_string()),  reload: Set(false), enable: Set(true), r#type: Set("NONE".to_string()) },
        DeviceCommandActiveModel { id: sea_orm::ActiveValue::NotSet, device_id: Set(25), name: Set("off".to_string()), value_from: Set(Some(0)), value_to: Set(Some(0)), delay: Set(Some(0)), description: Set("Abnama device 25 off".to_string()), reload: Set(true),  enable: Set(true), r#type: Set("NONE".to_string()) },
        DeviceCommandActiveModel { id: sea_orm::ActiveValue::NotSet, device_id: Set(26), name: Set("on".to_string()),  value_from: Set(Some(1)), value_to: Set(Some(1)), delay: Set(Some(0)), description: Set("Abnama device 26 on".to_string()),  reload: Set(false), enable: Set(true), r#type: Set("NONE".to_string()) },
        DeviceCommandActiveModel { id: sea_orm::ActiveValue::NotSet, device_id: Set(26), name: Set("off".to_string()), value_from: Set(Some(0)), value_to: Set(Some(0)), delay: Set(Some(0)), description: Set("Abnama device 26 off".to_string()), reload: Set(true),  enable: Set(true), r#type: Set("NONE".to_string()) },
    ];

    println!("🚀 Starting to add {} sample device commands...", sample_commands.len());
    
    for (index, command) in sample_commands.into_iter().enumerate() 
    {
        let command_name = match &command.name 
        {
            Set(name) => name.clone(),
            _ => "Unknown".to_string(),
        };
        println!("📝 Adding device command {}: {}", index + 1, command_name);
        
        let result = device_command_orm.add(db, command).await;
        if result.status 
        {
            if let Some(added_command) = result.data 
            {
                println!("✅ Successfully added device command: {} (ID: {})", added_command.name, added_command.id);
            }
        } 
        else 
        {
            if let Some(error) = result.error 
            {
                println!("❌ Failed to add device command: {}", error);
            }
        }
    }
    
    println!("🎉 Finished adding sample device commands!");
    Ok(())
}

//--------------------------------------------------------------------------------- List All Device Commands Logic
pub async fn list_all_device_commands(db: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    let device_command_orm = DeviceCommandORM::new(true, true);
    let filters = std::collections::HashMap::new();
    
    println!("📋 Fetching all device commands...");
    
    let result = device_command_orm.items(db, filters).await;
    
    if result.status {
        if let Some(commands) = result.data {
            println!("🎮 Found {} device commands:", commands.len());
            println!("{:-<80}", "");
            for command in commands {
                let status = if command.enable { "✅ Enabled" } else { "❌ Disabled" };
                println!("ID: {} | Name: {} | Device: {} | Type: {} | From: {} | To: {} | Status: {}", 
                    command.id, command.name, command.device_id, command.r#type, 
                    command.value_from.unwrap_or(0), command.value_to.unwrap_or(0), status);
            }
            println!("{:-<80}", "");
        } else {
            println!("📭 No device commands found.");
        }
    } else {
        if let Some(error) = result.error {
            println!("❌ Error fetching device commands: {}", error);
        }
    }
    
    Ok(())
}
