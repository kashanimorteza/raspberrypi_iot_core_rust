//--------------------------------------------------------------------------------- Location
// src/logics/zone_command_action.rs

//--------------------------------------------------------------------------------- Description
// This file contains logic to add sample zone command actions using the ORM

//--------------------------------------------------------------------------------- Import
use crate::orm::logics::zone_command_action::ZoneCommandActionORM;
use sea_orm::DatabaseConnection;
use crate::orm::models::zone_command_action::ActiveModel as ZoneCommandActionActiveModel;
use sea_orm::ActiveValue::Set;

//--------------------------------------------------------------------------------- Add Sample Zone Command Actions Logic
pub async fn add_sample_zone_command_actions(db: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> 
{
    let zone_command_action_orm = ZoneCommandActionORM::new(true, true);
    let sample_zone_command_actions = vec![
        // Pool zone command actions
        ZoneCommandActionActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_command_id: Set(1),
            name: Set("Turn on shir".to_string()),
            device_id: Set(2),
            command_id: Set(Some(3)),
            description: Set("Pool shir on action".to_string()),
            enable: Set(true),
        },
        ZoneCommandActionActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_command_id: Set(2),
            name: Set("Turn off shir".to_string()),
            device_id: Set(2),
            command_id: Set(Some(4)),
            description: Set("Pool shir off action".to_string()),
            enable: Set(true),
        },
        ZoneCommandActionActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_command_id: Set(3),
            name: Set("Turn on shir".to_string()),
            device_id: Set(2),
            command_id: Set(Some(3)),
            description: Set("Pool shir restart action".to_string()),
            enable: Set(true),
        },
        // Jacuzzi zone command actions
        ZoneCommandActionActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_command_id: Set(4),
            name: Set("Turn on shir".to_string()),
            device_id: Set(6),
            command_id: Set(Some(12)),
            description: Set("Jacuzzi shir on action".to_string()),
            enable: Set(true),
        },
        ZoneCommandActionActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_command_id: Set(5),
            name: Set("Turn off shir".to_string()),
            device_id: Set(6),
            command_id: Set(Some(13)),
            description: Set("Jacuzzi shir off action".to_string()),
            enable: Set(true),
        },
        ZoneCommandActionActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_command_id: Set(6),
            name: Set("Turn on shir".to_string()),
            device_id: Set(6),
            command_id: Set(Some(12)),
            description: Set("Jacuzzi shir restart action".to_string()),
            enable: Set(true),
        },
        ZoneCommandActionActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_command_id: Set(7),
            name: Set("Turn on erjet".to_string()),
            device_id: Set(9),
            command_id: Set(Some(18)),
            description: Set("Jacuzzi erjet on action".to_string()),
            enable: Set(true),
        },
        ZoneCommandActionActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_command_id: Set(8),
            name: Set("Turn off erjet".to_string()),
            device_id: Set(9),
            command_id: Set(Some(19)),
            description: Set("Jacuzzi erjet off action".to_string()),
            enable: Set(true),
        },
        // Sauna-Dry zone command actions
        ZoneCommandActionActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_command_id: Set(9),
            name: Set("Turn on hiter".to_string()),
            device_id: Set(11),
            command_id: Set(Some(23)),
            description: Set("Sauna dry heater on action".to_string()),
            enable: Set(true),
        },
        ZoneCommandActionActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_command_id: Set(10),
            name: Set("Turn off hiter".to_string()),
            device_id: Set(11),
            command_id: Set(Some(24)),
            description: Set("Sauna dry heater off action".to_string()),
            enable: Set(true),
        },
        ZoneCommandActionActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_command_id: Set(11),
            name: Set("Turn on hiter".to_string()),
            device_id: Set(11),
            command_id: Set(Some(23)),
            description: Set("Sauna dry heater restart action".to_string()),
            enable: Set(true),
        },
        // Fan zone command actions
        ZoneCommandActionActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_command_id: Set(12),
            name: Set("Turn on fan".to_string()),
            device_id: Set(14),
            command_id: Set(Some(30)),
            description: Set("Fan motor on action".to_string()),
            enable: Set(true),
        },
        ZoneCommandActionActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_command_id: Set(13),
            name: Set("Turn off fan".to_string()),
            device_id: Set(14),
            command_id: Set(Some(31)),
            description: Set("Fan motor off action".to_string()),
            enable: Set(true),
        },
        ZoneCommandActionActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_command_id: Set(14),
            name: Set("Turn on fan".to_string()),
            device_id: Set(14),
            command_id: Set(Some(32)),
            description: Set("Fan motor restart action".to_string()),
            enable: Set(true),
        },
        // Steam Sauna zone command actions
        ZoneCommandActionActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_command_id: Set(15),
            name: Set("Turn on shir".to_string()),
            device_id: Set(17),
            command_id: Set(Some(37)),
            description: Set("Steam sauna shir on action".to_string()),
            enable: Set(true),
        },
        ZoneCommandActionActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_command_id: Set(16),
            name: Set("Turn off shir".to_string()),
            device_id: Set(17),
            command_id: Set(Some(38)),
            description: Set("Steam sauna shir off action".to_string()),
            enable: Set(true),
        },
        ZoneCommandActionActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_command_id: Set(17),
            name: Set("Turn on hitter".to_string()),
            device_id: Set(18),
            command_id: Set(Some(39)),
            description: Set("Steam sauna heater on action".to_string()),
            enable: Set(true),
        },
        ZoneCommandActionActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_command_id: Set(18),
            name: Set("Turn off hiter".to_string()),
            device_id: Set(18),
            command_id: Set(Some(40)),
            description: Set("Steam sauna heater off action 1".to_string()),
            enable: Set(true),
        },
        ZoneCommandActionActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_command_id: Set(19),
            name: Set("Turn off hiter".to_string()),
            device_id: Set(18),
            command_id: Set(Some(40)),
            description: Set("Steam sauna heater off action 2".to_string()),
            enable: Set(true),
        },
        ZoneCommandActionActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_command_id: Set(20),
            name: Set("Turn on hiter".to_string()),
            device_id: Set(18),
            command_id: Set(Some(39)),
            description: Set("Steam sauna heater restart action".to_string()),
            enable: Set(true),
        },
    ];

    println!("🚀 Starting to add {} sample zone command actions...", sample_zone_command_actions.len());
    
    for (index, zone_command_action) in sample_zone_command_actions.into_iter().enumerate() 
    {
        let action_name = match &zone_command_action.name 
        {
            Set(name) => name.clone(),
            _ => "Unknown".to_string(),
        };
        println!("📝 Adding zone command action {}: {}", index + 1, action_name);
        
        let result = zone_command_action_orm.add(db, zone_command_action).await;
        if result.success 
        {
            if let Some(added_action) = result.data 
            {
                println!("✅ Successfully added zone command action: {} (ID: {})", added_action.name, added_action.id);
            }
        } 
        else 
        {
            if let Some(error) = result.error 
            {
                println!("❌ Failed to add zone command action: {}", error);
            }
        }
    }
    
    println!("🎉 Finished adding sample zone command actions!");
    Ok(())
}

//--------------------------------------------------------------------------------- List All Zone Command Actions Logic
pub async fn list_all_zone_command_actions(db: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    let zone_command_action_orm = ZoneCommandActionORM::new(true, true);
    let filters = std::collections::HashMap::new();
    
    println!("📋 Fetching all zone command actions...");
    
    let result = zone_command_action_orm.items(db, filters).await;
    
    if result.success {
        if let Some(actions) = result.data {
            println!("⚡ Found {} zone command actions:", actions.len());
            println!("{:-<80}", "");
            for action in actions {
                let status = if action.enable { "✅ Enabled" } else { "❌ Disabled" };
                println!("ID: {} | Name: {} | Zone Command: {} | Device: {} | Command: {} | Status: {}", 
                    action.id, action.name, action.zone_command_id, 
                    action.device_id, action.command_id.unwrap_or(0), status);
            }
            println!("{:-<80}", "");
        } else {
            println!("📭 No zone command actions found.");
        }
    } else {
        if let Some(error) = result.error {
            println!("❌ Error fetching zone command actions: {}", error);
        }
    }
    
    Ok(())
}
