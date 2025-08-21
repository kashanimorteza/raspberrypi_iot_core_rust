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
        if result.success 
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
    
    if result.success {
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
