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
        // None port
        PortActiveModel {
            id: Set(0),
            user_id: Set(1),
            name: Set("None".to_string()),
            pin: Set(Some(0)),
            port: Set(None),
            value: Set(Some(0)),
            description: Set("None port".to_string()),
            enable: Set(true),
            protocol: Set("PWR".to_string()),
            r#type: Set("PWR".to_string()),
        },
        // PWR ports
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("PWR-1".to_string()),
            pin: Set(Some(1)),
            port: Set(None),
            value: Set(Some(3)),
            description: Set("Power pin 1".to_string()),
            enable: Set(true),
            protocol: Set("PWR".to_string()),
            r#type: Set("PWR".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("PWR-2".to_string()),
            pin: Set(Some(2)),
            port: Set(None),
            value: Set(Some(5)),
            description: Set("Power pin 2".to_string()),
            enable: Set(true),
            protocol: Set("PWR".to_string()),
            r#type: Set("PWR".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("PWR-3".to_string()),
            pin: Set(Some(4)),
            port: Set(None),
            value: Set(Some(5)),
            description: Set("Power pin 3".to_string()),
            enable: Set(true),
            protocol: Set("PWR".to_string()),
            r#type: Set("PWR".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("PWR-4".to_string()),
            pin: Set(Some(17)),
            port: Set(None),
            value: Set(Some(3)),
            description: Set("Power pin 4".to_string()),
            enable: Set(true),
            protocol: Set("PWR".to_string()),
            r#type: Set("PWR".to_string()),
        },
        // GND ports
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("GND-1".to_string()),
            pin: Set(Some(6)),
            port: Set(None),
            value: Set(Some(0)),
            description: Set("Ground pin 1".to_string()),
            enable: Set(true),
            protocol: Set("GND".to_string()),
            r#type: Set("GND".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("GND-2".to_string()),
            pin: Set(Some(9)),
            port: Set(None),
            value: Set(Some(0)),
            description: Set("Ground pin 2".to_string()),
            enable: Set(true),
            protocol: Set("GND".to_string()),
            r#type: Set("GND".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("GND-3".to_string()),
            pin: Set(Some(14)),
            port: Set(None),
            value: Set(Some(0)),
            description: Set("Ground pin 3".to_string()),
            enable: Set(true),
            protocol: Set("GND".to_string()),
            r#type: Set("GND".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("GND-4".to_string()),
            pin: Set(Some(20)),
            port: Set(None),
            value: Set(Some(0)),
            description: Set("Ground pin 4".to_string()),
            enable: Set(true),
            protocol: Set("GND".to_string()),
            r#type: Set("GND".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("GND-5".to_string()),
            pin: Set(Some(25)),
            port: Set(None),
            value: Set(Some(0)),
            description: Set("Ground pin 5".to_string()),
            enable: Set(true),
            protocol: Set("GND".to_string()),
            r#type: Set("GND".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("GND-6".to_string()),
            pin: Set(Some(30)),
            port: Set(None),
            value: Set(Some(0)),
            description: Set("Ground pin 6".to_string()),
            enable: Set(true),
            protocol: Set("GND".to_string()),
            r#type: Set("GND".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("GND-7".to_string()),
            pin: Set(Some(34)),
            port: Set(None),
            value: Set(Some(0)),
            description: Set("Ground pin 7".to_string()),
            enable: Set(true),
            protocol: Set("GND".to_string()),
            r#type: Set("GND".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("GND-8".to_string()),
            pin: Set(Some(39)),
            port: Set(None),
            value: Set(Some(0)),
            description: Set("Ground pin 8".to_string()),
            enable: Set(true),
            protocol: Set("GND".to_string()),
            r#type: Set("GND".to_string()),
        },
        // Reserved ports
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("Reserved".to_string()),
            pin: Set(Some(27)),
            port: Set(Some(0)),
            value: Set(Some(0)),
            description: Set("Reserved pin 27".to_string()),
            enable: Set(true),
            protocol: Set("RESERVED".to_string()),
            r#type: Set("IN".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("Reserved".to_string()),
            pin: Set(Some(28)),
            port: Set(Some(0)),
            value: Set(Some(0)),
            description: Set("Reserved pin 28".to_string()),
            enable: Set(true),
            protocol: Set("RESERVED".to_string()),
            r#type: Set("IN".to_string()),
        },
        // I2C ports
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("I2C-SDA".to_string()),
            pin: Set(Some(3)),
            port: Set(Some(2)),
            value: Set(Some(0)),
            description: Set("I2C SDA pin".to_string()),
            enable: Set(true),
            protocol: Set("I2C".to_string()),
            r#type: Set("SDA".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("I2C-SCL".to_string()),
            pin: Set(Some(5)),
            port: Set(Some(3)),
            value: Set(Some(0)),
            description: Set("I2C SCL pin".to_string()),
            enable: Set(true),
            protocol: Set("I2C".to_string()),
            r#type: Set("SCL".to_string()),
        },
        // UART ports
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("UART-TX".to_string()),
            pin: Set(Some(8)),
            port: Set(Some(0)),
            value: Set(Some(0)),
            description: Set("UART TX pin".to_string()),
            enable: Set(true),
            protocol: Set("UART".to_string()),
            r#type: Set("TX".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("UART-RX".to_string()),
            pin: Set(Some(10)),
            port: Set(Some(0)),
            value: Set(Some(0)),
            description: Set("UART RX pin".to_string()),
            enable: Set(true),
            protocol: Set("UART".to_string()),
            r#type: Set("RX".to_string()),
        },
        // GPIO ports
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("Data-1".to_string()),
            pin: Set(Some(7)),
            port: Set(Some(4)),
            value: Set(Some(0)),
            description: Set("Data pin 1".to_string()),
            enable: Set(true),
            protocol: Set("FILE".to_string()),
            r#type: Set("IN".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("P-Motor".to_string()),
            pin: Set(Some(11)),
            port: Set(Some(17)),
            value: Set(Some(0)),
            description: Set("Pool Motor".to_string()),
            enable: Set(true),
            protocol: Set("GPIO".to_string()),
            r#type: Set("OUT".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("P-Shir".to_string()),
            pin: Set(Some(12)),
            port: Set(Some(18)),
            value: Set(Some(0)),
            description: Set("Pool Shir".to_string()),
            enable: Set(true),
            protocol: Set("GPIO".to_string()),
            r#type: Set("OUT".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("P-Light".to_string()),
            pin: Set(Some(13)),
            port: Set(Some(27)),
            value: Set(Some(0)),
            description: Set("Pool Light".to_string()),
            enable: Set(true),
            protocol: Set("GPIO".to_string()),
            r#type: Set("OUT".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("J-Motor".to_string()),
            pin: Set(Some(15)),
            port: Set(Some(22)),
            value: Set(Some(0)),
            description: Set("Jacuzzi Motor".to_string()),
            enable: Set(true),
            protocol: Set("GPIO".to_string()),
            r#type: Set("OUT".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("J-Shir".to_string()),
            pin: Set(Some(16)),
            port: Set(Some(23)),
            value: Set(Some(0)),
            description: Set("Jacuzzi Shir".to_string()),
            enable: Set(true),
            protocol: Set("GPIO".to_string()),
            r#type: Set("OUT".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("J-Light".to_string()),
            pin: Set(Some(18)),
            port: Set(Some(24)),
            value: Set(Some(0)),
            description: Set("Jacuzzi Light".to_string()),
            enable: Set(true),
            protocol: Set("GPIO".to_string()),
            r#type: Set("OUT".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("J-Fuse".to_string()),
            pin: Set(Some(19)),
            port: Set(Some(10)),
            value: Set(Some(0)),
            description: Set("Jacuzzi Fuse".to_string()),
            enable: Set(true),
            protocol: Set("GPIO".to_string()),
            r#type: Set("IN".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("J-Erjet".to_string()),
            pin: Set(Some(21)),
            port: Set(Some(9)),
            value: Set(Some(0)),
            description: Set("Jacuzzi Erjet".to_string()),
            enable: Set(true),
            protocol: Set("GPIO".to_string()),
            r#type: Set("OUT".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("SD-Hiter".to_string()),
            pin: Set(Some(22)),
            port: Set(Some(25)),
            value: Set(Some(0)),
            description: Set("Sauna Dry Heater".to_string()),
            enable: Set(true),
            protocol: Set("GPIO".to_string()),
            r#type: Set("OUT".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("SD-Light".to_string()),
            pin: Set(Some(23)),
            port: Set(Some(11)),
            value: Set(Some(0)),
            description: Set("Sauna Dry Light".to_string()),
            enable: Set(true),
            protocol: Set("GPIO".to_string()),
            r#type: Set("OUT".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("F-Motor".to_string()),
            pin: Set(Some(24)),
            port: Set(Some(8)),
            value: Set(Some(0)),
            description: Set("Fan Motor".to_string()),
            enable: Set(true),
            protocol: Set("GPIO".to_string()),
            r#type: Set("OUT".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("C-Motor".to_string()),
            pin: Set(Some(26)),
            port: Set(Some(7)),
            value: Set(Some(0)),
            description: Set("Cold Motor".to_string()),
            enable: Set(true),
            protocol: Set("GPIO".to_string()),
            r#type: Set("OUT".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("SS-Shir".to_string()),
            pin: Set(Some(29)),
            port: Set(Some(5)),
            value: Set(Some(0)),
            description: Set("Steam Sauna Shir".to_string()),
            enable: Set(true),
            protocol: Set("GPIO".to_string()),
            r#type: Set("OUT".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("SS-Hiter".to_string()),
            pin: Set(Some(31)),
            port: Set(Some(6)),
            value: Set(Some(0)),
            description: Set("Steam Sauna Heater".to_string()),
            enable: Set(true),
            protocol: Set("GPIO".to_string()),
            r#type: Set("OUT".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("SS-Light".to_string()),
            pin: Set(Some(32)),
            port: Set(Some(12)),
            value: Set(Some(0)),
            description: Set("Steam Sauna Light".to_string()),
            enable: Set(true),
            protocol: Set("GPIO".to_string()),
            r#type: Set("OUT".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("SS-WH".to_string()),
            pin: Set(Some(33)),
            port: Set(Some(13)),
            value: Set(Some(0)),
            description: Set("Steam Sauna Water High".to_string()),
            enable: Set(true),
            protocol: Set("GPIO".to_string()),
            r#type: Set("IN".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("SS-WL".to_string()),
            pin: Set(Some(35)),
            port: Set(Some(19)),
            value: Set(Some(0)),
            description: Set("Steam Sauna Water Low".to_string()),
            enable: Set(true),
            protocol: Set("GPIO".to_string()),
            r#type: Set("IN".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("SS-Esans".to_string()),
            pin: Set(Some(36)),
            port: Set(Some(16)),
            value: Set(Some(0)),
            description: Set("Steam Sauna Esans".to_string()),
            enable: Set(true),
            protocol: Set("GPIO".to_string()),
            r#type: Set("OUT".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("SS-Takhlie".to_string()),
            pin: Set(Some(37)),
            port: Set(Some(26)),
            value: Set(Some(0)),
            description: Set("Steam Sauna Takhlie".to_string()),
            enable: Set(true),
            protocol: Set("GPIO".to_string()),
            r#type: Set("OUT".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("C-Light".to_string()),
            pin: Set(Some(38)),
            port: Set(Some(20)),
            value: Set(Some(0)),
            description: Set("Cold Light".to_string()),
            enable: Set(true),
            protocol: Set("GPIO".to_string()),
            r#type: Set("OUT".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("Abnama".to_string()),
            pin: Set(Some(40)),
            port: Set(Some(21)),
            value: Set(Some(0)),
            description: Set("Abnama".to_string()),
            enable: Set(true),
            protocol: Set("GPIO".to_string()),
            r#type: Set("OUT".to_string()),
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
