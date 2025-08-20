//--------------------------------------------------------------------------------- Location
// src/logics/port.rs

//--------------------------------------------------------------------------------- Description
// This file contains logic to add sample ports using the ORM

//--------------------------------------------------------------------------------- Import
use crate::orm::logics::port::PortORM;
use sea_orm::DatabaseConnection;
use crate::orm::models::port::ActiveModel as PortActiveModel;
use sea_orm::ActiveValue::Set;

//--------------------------------------------------------------------------------- Add Sample Ports Logic
pub async fn add_sample_ports(db: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> 
{
    let port_orm = PortORM::new(true, true);
    let sample_ports = vec![
        PortActiveModel 
        {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("GPIO Pin 18".to_string()),
            pin: Set(Some(18)),
            port: Set(Some(1)),
            value: Set(Some(0)),
            description: Set("Digital output pin for LED control".to_string()),
            enable: Set(true),
            protocol: Set("GPIO".to_string()),
            r#type: Set("Digital".to_string()),
        },
        PortActiveModel 
        {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("I2C Bus 1".to_string()),
            pin: Set(Some(2)),
            port: Set(Some(2)),
            value: Set(Some(0)),
            description: Set("I2C communication bus for sensors".to_string()),
            enable: Set(true),
            protocol: Set("I2C".to_string()),
            r#type: Set("Communication".to_string()),
        },
    ];

    println!("🚀 Starting to add {} sample ports...", sample_ports.len());
    
    for (index, port) in sample_ports.into_iter().enumerate() 
    {
        let port_name = match &port.name 
        {
            Set(name) => name.clone(),
            _ => "Unknown".to_string(),
        };
        println!("📝 Adding port {}: {}", index + 1, port_name);
        
        let result = port_orm.add(db, port).await;
        if result.success 
        {
            if let Some(added_port) = result.data 
            {
                println!("✅ Successfully added port: {} (ID: {})", added_port.name, added_port.id);
            }
        } 
        else 
        {
            if let Some(error) = result.error 
            {
                println!("❌ Failed to add port: {}", error);
            }
        }
    }
    
    println!("🎉 Finished adding sample ports!");
    Ok(())
}

//--------------------------------------------------------------------------------- List All Ports Logic
pub async fn list_all_ports(db: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    let port_orm = PortORM::new(true, true);
    let filters = std::collections::HashMap::new();
    
    println!("📋 Fetching all ports...");
    
    let result = port_orm.items(db, filters).await;
    
    if result.success {
        if let Some(ports) = result.data {
            println!("🔌 Found {} ports:", ports.len());
            println!("{:-<80}", "");
            for port in ports {
                let status = if port.enable { "✅ Enabled" } else { "❌ Disabled" };
                println!("ID: {} | Name: {} | Pin: {} | Protocol: {} | Type: {} | Status: {}", 
                    port.id, port.name, port.pin.unwrap_or(0), port.protocol, port.r#type, status);
            }
            println!("{:-<80}", "");
        } else {
            println!("📭 No ports found.");
        }
    } else {
        if let Some(error) = result.error {
            println!("❌ Error fetching ports: {}", error);
        }
    }
    
    Ok(())
}
