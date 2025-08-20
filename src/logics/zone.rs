//--------------------------------------------------------------------------------- Location
// src/logics/zone.rs

//--------------------------------------------------------------------------------- Description
// This file contains logic to add sample zones using the ORM

//--------------------------------------------------------------------------------- Import
use crate::orm::logics::zone::ZoneORM;
use sea_orm::DatabaseConnection;
use crate::orm::models::zone::ActiveModel as ZoneActiveModel;
use sea_orm::ActiveValue::Set;

//--------------------------------------------------------------------------------- Add Sample Zones Logic
pub async fn add_sample_zones(db: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> 
{
    let zone_orm = ZoneORM::new(true, true);
    let sample_zones = vec![
        ZoneActiveModel 
        {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("Living Room".to_string()),
            description: Set("Main living area with smart devices".to_string()),
            enable: Set(true),
        },
        ZoneActiveModel 
        {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("Kitchen".to_string()),
            description: Set("Kitchen area with appliances".to_string()),
            enable: Set(true),
        },
        ZoneActiveModel 
        {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("Bedroom".to_string()),
            description: Set("Master bedroom with climate control".to_string()),
            enable: Set(true),
        },
    ];

    println!("🚀 Starting to add {} sample zones...", sample_zones.len());
    
    for (index, zone) in sample_zones.into_iter().enumerate() 
    {
        let zone_name = match &zone.name 
        {
            Set(name) => name.clone(),
            _ => "Unknown".to_string(),
        };
        println!("📝 Adding zone {}: {}", index + 1, zone_name);
        
        let result = zone_orm.add(db, zone).await;
        if result.success 
        {
            if let Some(added_zone) = result.data 
            {
                println!("✅ Successfully added zone: {} (ID: {})", added_zone.name, added_zone.id);
            }
        } 
        else 
        {
            if let Some(error) = result.error 
            {
                println!("❌ Failed to add zone: {}", error);
            }
        }
    }
    
    println!("🎉 Finished adding sample zones!");
    Ok(())
}

//--------------------------------------------------------------------------------- List All Zones Logic
pub async fn list_all_zones(db: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    let zone_orm = ZoneORM::new(true, true);
    let filters = std::collections::HashMap::new();
    
    println!("📋 Fetching all zones...");
    
    let result = zone_orm.items(db, filters).await;
    
    if result.success {
        if let Some(zones) = result.data {
            println!("🏠 Found {} zones:", zones.len());
            println!("{:-<80}", "");
            for zone in zones {
                let status = if zone.enable { "✅ Enabled" } else { "❌ Disabled" };
                println!("ID: {} | Name: {} | User: {} | Description: {} | Status: {}", 
                    zone.id, zone.name, zone.user_id, zone.description, status);
            }
            println!("{:-<80}", "");
        } else {
            println!("📭 No zones found.");
        }
    } else {
        if let Some(error) = result.error {
            println!("❌ Error fetching zones: {}", error);
        }
    }
    
    Ok(())
}
