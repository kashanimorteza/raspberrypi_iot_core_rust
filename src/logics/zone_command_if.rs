//--------------------------------------------------------------------------------- Location
// src/logics/zone_command_if.rs

//--------------------------------------------------------------------------------- Description
// This file contains logic to add sample zone command ifs using the ORM

//--------------------------------------------------------------------------------- Import
use crate::orm::logics::zone_command_if::ZoneCommandIfORM;
use sea_orm::DatabaseConnection;
use crate::orm::models::zone_command_if::ActiveModel as ZoneCommandIfActiveModel;
use sea_orm::ActiveValue::Set;

//--------------------------------------------------------------------------------- Add Sample Zone Command Ifs Logic
pub async fn add_sample_zone_command_ifs(db: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> 
{
    let zone_command_if_orm = ZoneCommandIfORM::new(true, true);
    let sample_zone_command_ifs = vec![
        // Pool zone command ifs
        ZoneCommandIfActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_command_id: Set(1),
            name: Set("If sensor is on".to_string()),
            device_id: Set(4),
            command_id: Set(7),
            description: Set("Pool shir on condition".to_string()),
            enable: Set(true),
        },
        ZoneCommandIfActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_command_id: Set(2),
            name: Set("If sensor is off".to_string()),
            device_id: Set(4),
            command_id: Set(8),
            description: Set("Pool shir off condition".to_string()),
            enable: Set(true),
        },
        ZoneCommandIfActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_command_id: Set(3),
            name: Set("If sensor is reon".to_string()),
            device_id: Set(4),
            command_id: Set(9),
            description: Set("Pool shir restart condition".to_string()),
            enable: Set(true),
        },
        // Jacuzzi zone command ifs
        ZoneCommandIfActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_command_id: Set(4),
            name: Set("If sensor is on".to_string()),
            device_id: Set(10),
            command_id: Set(20),
            description: Set("Jacuzzi shir on condition".to_string()),
            enable: Set(true),
        },
        ZoneCommandIfActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_command_id: Set(5),
            name: Set("If sensor is off".to_string()),
            device_id: Set(10),
            command_id: Set(21),
            description: Set("Jacuzzi shir off condition".to_string()),
            enable: Set(true),
        },
        ZoneCommandIfActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_command_id: Set(6),
            name: Set("If sensor is reon".to_string()),
            device_id: Set(10),
            command_id: Set(22),
            description: Set("Jacuzzi shir restart condition".to_string()),
            enable: Set(true),
        },
        ZoneCommandIfActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_command_id: Set(7),
            name: Set("If fuse is on".to_string()),
            device_id: Set(8),
            command_id: Set(16),
            description: Set("Jacuzzi erjet on condition".to_string()),
            enable: Set(true),
        },
        ZoneCommandIfActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_command_id: Set(8),
            name: Set("If fuse is off".to_string()),
            device_id: Set(8),
            command_id: Set(17),
            description: Set("Jacuzzi erjet off condition".to_string()),
            enable: Set(true),
        },
        // Sauna-Dry zone command ifs
        ZoneCommandIfActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_command_id: Set(9),
            name: Set("If sensor is on".to_string()),
            device_id: Set(13),
            command_id: Set(27),
            description: Set("Sauna dry heater on condition".to_string()),
            enable: Set(true),
        },
        ZoneCommandIfActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_command_id: Set(10),
            name: Set("If sensor is off".to_string()),
            device_id: Set(13),
            command_id: Set(28),
            description: Set("Sauna dry heater off condition".to_string()),
            enable: Set(true),
        },
        ZoneCommandIfActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_command_id: Set(11),
            name: Set("If sensor is reon".to_string()),
            device_id: Set(13),
            command_id: Set(29),
            description: Set("Sauna dry heater restart condition".to_string()),
            enable: Set(true),
        },
        // Fan zone command ifs
        ZoneCommandIfActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_command_id: Set(12),
            name: Set("If sensor is on".to_string()),
            device_id: Set(15),
            command_id: Set(32),
            description: Set("Fan motor on condition".to_string()),
            enable: Set(true),
        },
        ZoneCommandIfActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_command_id: Set(13),
            name: Set("If sensor is off".to_string()),
            device_id: Set(15),
            command_id: Set(33),
            description: Set("Fan motor off condition".to_string()),
            enable: Set(true),
        },
        ZoneCommandIfActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_command_id: Set(14),
            name: Set("If sensor is reon".to_string()),
            device_id: Set(15),
            command_id: Set(34),
            description: Set("Fan motor restart condition".to_string()),
            enable: Set(true),
        },
        // Steam Sauna zone command ifs
        ZoneCommandIfActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_command_id: Set(15),
            name: Set("If sensor WL is off".to_string()),
            device_id: Set(21),
            command_id: Set(46),
            description: Set("Steam sauna shir on condition".to_string()),
            enable: Set(true),
        },
        ZoneCommandIfActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_command_id: Set(16),
            name: Set("If sensor WH is on".to_string()),
            device_id: Set(20),
            command_id: Set(43),
            description: Set("Steam sauna shir off condition".to_string()),
            enable: Set(true),
        },
        ZoneCommandIfActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_command_id: Set(17),
            name: Set("If sensor WL is on".to_string()),
            device_id: Set(21),
            command_id: Set(45),
            description: Set("Steam sauna heater on condition 1".to_string()),
            enable: Set(true),
        },
        ZoneCommandIfActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_command_id: Set(17),
            name: Set("If sensor is on".to_string()),
            device_id: Set(22),
            command_id: Set(47),
            description: Set("Steam sauna heater on condition 2".to_string()),
            enable: Set(true),
        },
        ZoneCommandIfActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_command_id: Set(18),
            name: Set("If sensor WL is off".to_string()),
            device_id: Set(21),
            command_id: Set(46),
            description: Set("Steam sauna heater off condition 1".to_string()),
            enable: Set(true),
        },
        ZoneCommandIfActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_command_id: Set(19),
            name: Set("If sensor is off".to_string()),
            device_id: Set(22),
            command_id: Set(48),
            description: Set("Steam sauna heater off condition 2".to_string()),
            enable: Set(true),
        },
        ZoneCommandIfActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_command_id: Set(20),
            name: Set("If sensor WL is on".to_string()),
            device_id: Set(21),
            command_id: Set(45),
            description: Set("Steam sauna heater restart condition 1".to_string()),
            enable: Set(true),
        },
        ZoneCommandIfActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_command_id: Set(20),
            name: Set("If sensor is reon".to_string()),
            device_id: Set(22),
            command_id: Set(49),
            description: Set("Steam sauna heater restart condition 2".to_string()),
            enable: Set(true),
        },
    ];

    println!("🚀 Starting to add {} sample zone command ifs...", sample_zone_command_ifs.len());
    
    for (index, zone_command_if) in sample_zone_command_ifs.into_iter().enumerate() 
    {
        let if_name = match &zone_command_if.name 
        {
            Set(name) => name.clone(),
            _ => "Unknown".to_string(),
        };
        println!("📝 Adding zone command if {}: {}", index + 1, if_name);
        
        let result = zone_command_if_orm.add(db, zone_command_if).await;
        if result.status 
        {
            if let Some(added_if) = result.data 
            {
                println!("✅ Successfully added zone command if: {} (ID: {})", added_if.name, added_if.id);
            }
        } 
        else 
        {
            if let Some(error) = result.error 
            {
                println!("❌ Failed to add zone command if: {}", error);
            }
        }
    }
    
    println!("🎉 Finished adding sample zone command ifs!");
    Ok(())
}

//--------------------------------------------------------------------------------- List All Zone Command Ifs Logic
pub async fn list_all_zone_command_ifs(db: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    let zone_command_if_orm = ZoneCommandIfORM::new(true, true);
    let filters = std::collections::HashMap::new();
    
    println!("📋 Fetching all zone command ifs...");
    
    let result = zone_command_if_orm.items(db, filters).await;
    
    if result.status {
        if let Some(ifs) = result.data {
            println!("🔀 Found {} zone command ifs:", ifs.len());
            println!("{:-<80}", "");
            for command_if in ifs {
                let status = if command_if.enable { "✅ Enabled" } else { "❌ Disabled" };
                println!("ID: {} | Name: {} | Zone Command: {} | Device: {} | Command: {} | Status: {}", 
                    command_if.id, command_if.name, command_if.zone_command_id, 
                    command_if.device_id, command_if.command_id, status);
            }
            println!("{:-<80}", "");
        } else {
            println!("📭 No zone command ifs found.");
        }
    } else {
        if let Some(error) = result.error {
            println!("❌ Error fetching zone command ifs: {}", error);
        }
    }
    
    Ok(())
}
