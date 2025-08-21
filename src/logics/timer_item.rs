//--------------------------------------------------------------------------------- Location
// src/logics/timer_item.rs

//--------------------------------------------------------------------------------- Description
// This file contains logic to add sample timer items using the ORM

//--------------------------------------------------------------------------------- Import
use crate::orm::logics::timer_item::TimerItemORM;
use sea_orm::DatabaseConnection;
use crate::orm::models::timer_item::ActiveModel as TimerItemActiveModel;
use sea_orm::ActiveValue::Set;

//--------------------------------------------------------------------------------- Add Sample Timer Items Logic
pub async fn add_sample_timer_items(db: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> 
{
    let timer_item_orm = TimerItemORM::new(true, true);
    let sample_timer_items = vec![
        // Timer-On items (timer_id=1)
        TimerItemActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            timer_id: Set(1),
            name: Set("0-1".to_string()),
            value_from: Set("00:00".to_string()),
            value_to: Set("01:00".to_string()),
            description: Set("Timer on period 0-1".to_string()),
            enable: Set(true),
        },
        TimerItemActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            timer_id: Set(1),
            name: Set("1-2".to_string()),
            value_from: Set("01:00".to_string()),
            value_to: Set("02:00".to_string()),
            description: Set("Timer on period 1-2".to_string()),
            enable: Set(true),
        },
        TimerItemActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            timer_id: Set(1),
            name: Set("6-7".to_string()),
            value_from: Set("06:00".to_string()),
            value_to: Set("07:00".to_string()),
            description: Set("Timer on period 6-7".to_string()),
            enable: Set(true),
        },
        TimerItemActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            timer_id: Set(1),
            name: Set("7-8".to_string()),
            value_from: Set("07:00".to_string()),
            value_to: Set("08:00".to_string()),
            description: Set("Timer on period 7-8".to_string()),
            enable: Set(true),
        },
        TimerItemActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            timer_id: Set(1),
            name: Set("12-13".to_string()),
            value_from: Set("12:00".to_string()),
            value_to: Set("13:00".to_string()),
            description: Set("Timer on period 12-13".to_string()),
            enable: Set(true),
        },
        TimerItemActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            timer_id: Set(1),
            name: Set("13-14".to_string()),
            value_from: Set("13:00".to_string()),
            value_to: Set("14:00".to_string()),
            description: Set("Timer on period 13-14".to_string()),
            enable: Set(true),
        },
        TimerItemActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            timer_id: Set(1),
            name: Set("22-23".to_string()),
            value_from: Set("22:00".to_string()),
            value_to: Set("23:00".to_string()),
            description: Set("Timer on period 22-23".to_string()),
            enable: Set(true),
        },
        TimerItemActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            timer_id: Set(1),
            name: Set("23-0".to_string()),
            value_from: Set("23:00".to_string()),
            value_to: Set("00:00".to_string()),
            description: Set("Timer on period 23-0".to_string()),
            enable: Set(true),
        },
        // Timer-Off items (timer_id=2)
        TimerItemActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            timer_id: Set(2),
            name: Set("2-3".to_string()),
            value_from: Set("02:00".to_string()),
            value_to: Set("03:00".to_string()),
            description: Set("Timer off period 2-3".to_string()),
            enable: Set(true),
        },
        TimerItemActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            timer_id: Set(2),
            name: Set("3-4".to_string()),
            value_from: Set("03:00".to_string()),
            value_to: Set("04:00".to_string()),
            description: Set("Timer off period 3-4".to_string()),
            enable: Set(true),
        },
        TimerItemActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            timer_id: Set(2),
            name: Set("4-5".to_string()),
            value_from: Set("04:00".to_string()),
            value_to: Set("05:00".to_string()),
            description: Set("Timer off period 4-5".to_string()),
            enable: Set(true),
        },
        TimerItemActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            timer_id: Set(2),
            name: Set("5-6".to_string()),
            value_from: Set("05:00".to_string()),
            value_to: Set("06:00".to_string()),
            description: Set("Timer off period 5-6".to_string()),
            enable: Set(true),
        },
        TimerItemActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            timer_id: Set(2),
            name: Set("8-9".to_string()),
            value_from: Set("08:00".to_string()),
            value_to: Set("09:00".to_string()),
            description: Set("Timer off period 8-9".to_string()),
            enable: Set(true),
        },
        TimerItemActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            timer_id: Set(2),
            name: Set("9-10".to_string()),
            value_from: Set("09:00".to_string()),
            value_to: Set("10:00".to_string()),
            description: Set("Timer off period 9-10".to_string()),
            enable: Set(true),
        },
        TimerItemActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            timer_id: Set(2),
            name: Set("10-11".to_string()),
            value_from: Set("10:00".to_string()),
            value_to: Set("11:00".to_string()),
            description: Set("Timer off period 10-11".to_string()),
            enable: Set(true),
        },
        TimerItemActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            timer_id: Set(2),
            name: Set("11-12".to_string()),
            value_from: Set("11:00".to_string()),
            value_to: Set("12:00".to_string()),
            description: Set("Timer off period 11-12".to_string()),
            enable: Set(true),
        },
        TimerItemActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            timer_id: Set(2),
            name: Set("14-15".to_string()),
            value_from: Set("14:00".to_string()),
            value_to: Set("15:00".to_string()),
            description: Set("Timer off period 14-15".to_string()),
            enable: Set(true),
        },
        TimerItemActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            timer_id: Set(2),
            name: Set("15-16".to_string()),
            value_from: Set("15:00".to_string()),
            value_to: Set("16:00".to_string()),
            description: Set("Timer off period 15-16".to_string()),
            enable: Set(true),
        },
        TimerItemActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            timer_id: Set(2),
            name: Set("16-17".to_string()),
            value_from: Set("16:00".to_string()),
            value_to: Set("17:00".to_string()),
            description: Set("Timer off period 16-17".to_string()),
            enable: Set(true),
        },
        TimerItemActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            timer_id: Set(2),
            name: Set("17-18".to_string()),
            value_from: Set("17:00".to_string()),
            value_to: Set("18:00".to_string()),
            description: Set("Timer off period 17-18".to_string()),
            enable: Set(true),
        },
        TimerItemActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            timer_id: Set(2),
            name: Set("18-19".to_string()),
            value_from: Set("18:00".to_string()),
            value_to: Set("19:00".to_string()),
            description: Set("Timer off period 18-19".to_string()),
            enable: Set(true),
        },
        TimerItemActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            timer_id: Set(2),
            name: Set("19-20".to_string()),
            value_from: Set("19:00".to_string()),
            value_to: Set("20:00".to_string()),
            description: Set("Timer off period 19-20".to_string()),
            enable: Set(true),
        },
        TimerItemActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            timer_id: Set(2),
            name: Set("20-21".to_string()),
            value_from: Set("20:00".to_string()),
            value_to: Set("21:00".to_string()),
            description: Set("Timer off period 20-21".to_string()),
            enable: Set(true),
        },
        TimerItemActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            timer_id: Set(2),
            name: Set("21-22".to_string()),
            value_from: Set("21:00".to_string()),
            value_to: Set("22:00".to_string()),
            description: Set("Timer off period 21-22".to_string()),
            enable: Set(true),
        },
        // Additional timer item
        TimerItemActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            timer_id: Set(2),
            name: Set("part-1".to_string()),
            value_from: Set("15:00".to_string()),
            value_to: Set("16:00".to_string()),
            description: Set("Timer off part 1".to_string()),
            enable: Set(true),
        },
    ];

    println!("🚀 Starting to add {} sample timer items...", sample_timer_items.len());
    
    for (index, timer_item) in sample_timer_items.into_iter().enumerate() 
    {
        let item_name = match &timer_item.name 
        {
            Set(name) => name.clone(),
            _ => "Unknown".to_string(),
        };
        println!("📝 Adding timer item {}: {}", index + 1, item_name);
        
        let result = timer_item_orm.add(db, timer_item).await;
        if result.success 
        {
            if let Some(added_item) = result.data 
            {
                println!("✅ Successfully added timer item: {} (ID: {})", added_item.name, added_item.id);
            }
        } 
        else 
        {
            if let Some(error) = result.error 
            {
                println!("❌ Failed to add timer item: {}", error);
            }
        }
    }
    
    println!("🎉 Finished adding sample timer items!");
    Ok(())
}

//--------------------------------------------------------------------------------- List All Timer Items Logic
pub async fn list_all_timer_items(db: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    let timer_item_orm = TimerItemORM::new(true, true);
    let filters = std::collections::HashMap::new();
    
    println!("📋 Fetching all timer items...");
    
    let result = timer_item_orm.items(db, filters).await;
    
    if result.success {
        if let Some(items) = result.data {
            println!("⏰ Found {} timer items:", items.len());
            println!("{:-<80}", "");
            for item in items {
                let status = if item.enable { "✅ Enabled" } else { "❌ Disabled" };
                println!("ID: {} | Name: {} | Timer: {} | From: {} | To: {} | Status: {}", 
                    item.id, item.name, item.timer_id, item.value_from, item.value_to, status);
            }
            println!("{:-<80}", "");
        } else {
            println!("📭 No timer items found.");
        }
    } else {
        if let Some(error) = result.error {
            println!("❌ Error fetching timer items: {}", error);
        }
    }
    
    Ok(())
}
