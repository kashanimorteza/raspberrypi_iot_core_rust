//--------------------------------------------------------------------------------- Location
// src/logics/zone_command.rs

//--------------------------------------------------------------------------------- Description
// This file contains logic to add sample zone commands using the ORM

//--------------------------------------------------------------------------------- Import
use crate::orm::logics::zone_command::ZoneCommandORM;
use sea_orm::DatabaseConnection;
use crate::orm::models::zone_command::ActiveModel as ZoneCommandActiveModel;
use sea_orm::ActiveValue::Set;

//--------------------------------------------------------------------------------- Add Sample Zone Commands Logic
pub async fn add_sample_zone_commands(db: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> 
{
    let zone_command_orm = ZoneCommandORM::new(true, true);
    let sample_zone_commands = vec![
        // Pool zone commands
        ZoneCommandActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_id: Set(1),
            name: Set("Shir-On".to_string()),
            description: Set("Turn on pool shir system".to_string()),
            enable: Set(true),
        },
        ZoneCommandActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_id: Set(1),
            name: Set("Shir-Off".to_string()),
            description: Set("Turn off pool shir system".to_string()),
            enable: Set(true),
        },
        ZoneCommandActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_id: Set(1),
            name: Set("Shir-ReOn".to_string()),
            description: Set("Restart pool shir system".to_string()),
            enable: Set(true),
        },
        // Jacuzzi zone commands
        ZoneCommandActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_id: Set(2),
            name: Set("Shir-On".to_string()),
            description: Set("Turn on jacuzzi shir system".to_string()),
            enable: Set(true),
        },
        ZoneCommandActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_id: Set(2),
            name: Set("Shir-Off".to_string()),
            description: Set("Turn off jacuzzi shir system".to_string()),
            enable: Set(true),
        },
        ZoneCommandActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_id: Set(2),
            name: Set("Shir-ReOn".to_string()),
            description: Set("Restart jacuzzi shir system".to_string()),
            enable: Set(true),
        },
        ZoneCommandActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_id: Set(2),
            name: Set("Erjet-On".to_string()),
            description: Set("Turn on jacuzzi erjet system".to_string()),
            enable: Set(true),
        },
        ZoneCommandActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_id: Set(2),
            name: Set("Erjet-Off".to_string()),
            description: Set("Turn off jacuzzi erjet system".to_string()),
            enable: Set(true),
        },
        // Sauna-Dry zone commands
        ZoneCommandActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_id: Set(3),
            name: Set("Hiter-On".to_string()),
            description: Set("Turn on sauna dry heater".to_string()),
            enable: Set(true),
        },
        ZoneCommandActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_id: Set(3),
            name: Set("Hiter-Off".to_string()),
            description: Set("Turn off sauna dry heater".to_string()),
            enable: Set(true),
        },
        ZoneCommandActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_id: Set(3),
            name: Set("Hiter-ReOn".to_string()),
            description: Set("Restart sauna dry heater".to_string()),
            enable: Set(true),
        },
        // Fan zone commands
        ZoneCommandActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_id: Set(4),
            name: Set("Motor-On".to_string()),
            description: Set("Turn on fan motor".to_string()),
            enable: Set(true),
        },
        ZoneCommandActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_id: Set(4),
            name: Set("Motor-Off".to_string()),
            description: Set("Turn off fan motor".to_string()),
            enable: Set(true),
        },
        ZoneCommandActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_id: Set(4),
            name: Set("Motor-ReOn".to_string()),
            description: Set("Restart fan motor".to_string()),
            enable: Set(true),
        },
        // Steam Sauna zone commands
        ZoneCommandActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_id: Set(6),
            name: Set("Shir-On".to_string()),
            description: Set("Turn on steam sauna shir".to_string()),
            enable: Set(true),
        },
        ZoneCommandActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_id: Set(6),
            name: Set("Shir-Off".to_string()),
            description: Set("Turn off steam sauna shir".to_string()),
            enable: Set(true),
        },
        ZoneCommandActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_id: Set(6),
            name: Set("Hiter-On".to_string()),
            description: Set("Turn on steam sauna heater".to_string()),
            enable: Set(true),
        },
        ZoneCommandActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_id: Set(6),
            name: Set("Hiter-Off-1".to_string()),
            description: Set("Turn off steam sauna heater (mode 1)".to_string()),
            enable: Set(true),
        },
        ZoneCommandActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_id: Set(6),
            name: Set("Hiter-Off-2".to_string()),
            description: Set("Turn off steam sauna heater (mode 2)".to_string()),
            enable: Set(true),
        },
        ZoneCommandActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_id: Set(6),
            name: Set("Hiter-ReOn".to_string()),
            description: Set("Restart steam sauna heater".to_string()),
            enable: Set(true),
        },
    ];

    println!("🚀 Starting to add {} sample zone commands...", sample_zone_commands.len());
    
    for (index, zone_command) in sample_zone_commands.into_iter().enumerate() 
    {
        let command_name = match &zone_command.name 
        {
            Set(name) => name.clone(),
            _ => "Unknown".to_string(),
        };
        println!("📝 Adding zone command {}: {}", index + 1, command_name);
        
        let result = zone_command_orm.add(db, zone_command).await;
        if result.status 
        {
            if let Some(added_command) = result.data 
            {
                println!("✅ Successfully added zone command: {} (ID: {})", added_command.name, added_command.id);
            }
        } 
        else 
        {
            if let Some(error) = result.error 
            {
                println!("❌ Failed to add zone command: {}", error);
            }
        }
    }
    
    println!("🎉 Finished adding sample zone commands!");
    Ok(())
}

//--------------------------------------------------------------------------------- List All Zone Commands Logic
pub async fn list_all_zone_commands(db: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    let zone_command_orm = ZoneCommandORM::new(true, true);
    let filters = std::collections::HashMap::new();
    
    println!("📋 Fetching all zone commands...");
    
    let result = zone_command_orm.items(db, filters).await;
    
    if result.status {
        if let Some(commands) = result.data {
            println!("🎮 Found {} zone commands:", commands.len());
            println!("{:-<80}", "");
            for command in commands {
                let status = if command.enable { "✅ Enabled" } else { "❌ Disabled" };
                println!("ID: {} | Name: {} | Zone: {} | Description: {} | Status: {}", 
                    command.id, command.name, command.zone_id, command.description, status);
            }
            println!("{:-<80}", "");
        } else {
            println!("📭 No zone commands found.");
        }
    } else {
        if let Some(error) = result.error {
            println!("❌ Error fetching zone commands: {}", error);
        }
    }
    
    Ok(())
}
