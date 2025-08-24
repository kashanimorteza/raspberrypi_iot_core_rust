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
            pin: Set(0),
            port: Set(0),
            value: Set(0),
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
            pin: Set(1),
            port: Set(0),
            value: Set(3),
            description: Set("Power pin 1".to_string()),
            enable: Set(true),
            protocol: Set("PWR".to_string()),
            r#type: Set("PWR".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("PWR-2".to_string()),
            pin: Set(2),
            port: Set(0),
            value: Set(5),
            description: Set("Power pin 2".to_string()),
            enable: Set(true),
            protocol: Set("PWR".to_string()),
            r#type: Set("PWR".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("PWR-3".to_string()),
            pin: Set(4),
            port: Set(0),
            value: Set(5),
            description: Set("Power pin 3".to_string()),
            enable: Set(true),
            protocol: Set("PWR".to_string()),
            r#type: Set("PWR".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("PWR-4".to_string()),
            pin: Set(17),
            port: Set(0),
            value: Set(3),
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
            pin: Set(6),
            port: Set(0),
            value: Set(0),
            description: Set("Ground pin 1".to_string()),
            enable: Set(true),
            protocol: Set("GND".to_string()),
            r#type: Set("GND".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("GND-2".to_string()),
            pin: Set(9),
            port: Set(0),
            value: Set(0),
            description: Set("Ground pin 2".to_string()),
            enable: Set(true),
            protocol: Set("GND".to_string()),
            r#type: Set("GND".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("GND-3".to_string()),
            pin: Set(14),
            port: Set(0),
            value: Set(0),
            description: Set("Ground pin 3".to_string()),
            enable: Set(true),
            protocol: Set("GND".to_string()),
            r#type: Set("GND".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("GND-4".to_string()),
            pin: Set(20),
            port: Set(0),
            value: Set(0),
            description: Set("Ground pin 4".to_string()),
            enable: Set(true),
            protocol: Set("GND".to_string()),
            r#type: Set("GND".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("GND-5".to_string()),
            pin: Set(25),
            port: Set(0),
            value: Set(0),
            description: Set("Ground pin 5".to_string()),
            enable: Set(true),
            protocol: Set("GND".to_string()),
            r#type: Set("GND".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("GND-6".to_string()),
            pin: Set(30),
            port: Set(0),
            value: Set(0),
            description: Set("Ground pin 6".to_string()),
            enable: Set(true),
            protocol: Set("GND".to_string()),
            r#type: Set("GND".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("GND-7".to_string()),
            pin: Set(34),
            port: Set(0),
            value: Set(0),
            description: Set("Ground pin 7".to_string()),
            enable: Set(true),
            protocol: Set("GND".to_string()),
            r#type: Set("GND".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("GND-8".to_string()),
            pin: Set(39),
            port: Set(0),
            value: Set(0),
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
            pin: Set(27),
            port: Set(0),
            value: Set(0),
            description: Set("Reserved pin 27".to_string()),
            enable: Set(true),
            protocol: Set("RESERVED".to_string()),
            r#type: Set("IN".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("Reserved".to_string()),
            pin: Set(28),
            port: Set(0),
            value: Set(0),
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
            pin: Set(3),
            port: Set(2),
            value: Set(0),
            description: Set("I2C SDA pin".to_string()),
            enable: Set(true),
            protocol: Set("I2C".to_string()),
            r#type: Set("SDA".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("I2C-SCL".to_string()),
            pin: Set(5),
            port: Set(3),
            value: Set(0),
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
            pin: Set(8),
            port: Set(0),
            value: Set(0),
            description: Set("UART TX pin".to_string()),
            enable: Set(true),
            protocol: Set("UART".to_string()),
            r#type: Set("TX".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("UART-RX".to_string()),
            pin: Set(10),
            port: Set(0),
            value: Set(0),
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
            pin: Set(7),
            port: Set(4),
            value: Set(0),
            description: Set("Data pin 1".to_string()),
            enable: Set(true),
            protocol: Set("FILE".to_string()),
            r#type: Set("IN".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("P-Motor".to_string()),
            pin: Set(11),
            port: Set(17),
            value: Set(0),
            description: Set("Pool Motor".to_string()),
            enable: Set(true),
            protocol: Set("GPIO".to_string()),
            r#type: Set("OUT".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("P-Shir".to_string()),
            pin: Set(12),
            port: Set(18),
            value: Set(0),
            description: Set("Pool Shir".to_string()),
            enable: Set(true),
            protocol: Set("GPIO".to_string()),
            r#type: Set("OUT".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("P-Light".to_string()),
            pin: Set(13),
            port: Set(27),
            value: Set(0),
            description: Set("Pool Light".to_string()),
            enable: Set(true),
            protocol: Set("GPIO".to_string()),
            r#type: Set("OUT".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("J-Motor".to_string()),
            pin: Set(15),
            port: Set(22),
            value: Set(0),
            description: Set("Jacuzzi Motor".to_string()),
            enable: Set(true),
            protocol: Set("GPIO".to_string()),
            r#type: Set("OUT".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("J-Shir".to_string()),
            pin: Set(16),
            port: Set(23),
            value: Set(0),
            description: Set("Jacuzzi Shir".to_string()),
            enable: Set(true),
            protocol: Set("GPIO".to_string()),
            r#type: Set("OUT".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("J-Light".to_string()),
            pin: Set(18),
            port: Set(24),
            value: Set(0),
            description: Set("Jacuzzi Light".to_string()),
            enable: Set(true),
            protocol: Set("GPIO".to_string()),
            r#type: Set("OUT".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("J-Fuse".to_string()),
            pin: Set(19),
            port: Set(10),
            value: Set(0),
            description: Set("Jacuzzi Fuse".to_string()),
            enable: Set(true),
            protocol: Set("GPIO".to_string()),
            r#type: Set("IN".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("J-Erjet".to_string()),
            pin: Set(21),
            port: Set(9),
            value: Set(0),
            description: Set("Jacuzzi Erjet".to_string()),
            enable: Set(true),
            protocol: Set("GPIO".to_string()),
            r#type: Set("OUT".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("SD-Hiter".to_string()),
            pin: Set(22),
            port: Set(25),
            value: Set(0),
            description: Set("Sauna Dry Heater".to_string()),
            enable: Set(true),
            protocol: Set("GPIO".to_string()),
            r#type: Set("OUT".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("SD-Light".to_string()),
            pin: Set(23),
            port: Set(11),
            value: Set(0),
            description: Set("Sauna Dry Light".to_string()),
            enable: Set(true),
            protocol: Set("GPIO".to_string()),
            r#type: Set("OUT".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("F-Motor".to_string()),
            pin: Set(24),
            port: Set(8),
            value: Set(0),
            description: Set("Fan Motor".to_string()),
            enable: Set(true),
            protocol: Set("GPIO".to_string()),
            r#type: Set("OUT".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("C-Motor".to_string()),
            pin: Set(26),
            port: Set(7),
            value: Set(0),
            description: Set("Cold Motor".to_string()),
            enable: Set(true),
            protocol: Set("GPIO".to_string()),
            r#type: Set("OUT".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("SS-Shir".to_string()),
            pin: Set(29),
            port: Set(5),
            value: Set(0),
            description: Set("Steam Sauna Shir".to_string()),
            enable: Set(true),
            protocol: Set("GPIO".to_string()),
            r#type: Set("OUT".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("SS-Hiter".to_string()),
            pin: Set(31),
            port: Set(6),
            value: Set(0),
            description: Set("Steam Sauna Heater".to_string()),
            enable: Set(true),
            protocol: Set("GPIO".to_string()),
            r#type: Set("OUT".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("SS-Light".to_string()),
            pin: Set(32),
            port: Set(12),
            value: Set(0),
            description: Set("Steam Sauna Light".to_string()),
            enable: Set(true),
            protocol: Set("GPIO".to_string()),
            r#type: Set("OUT".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("SS-WH".to_string()),
            pin: Set(33),
            port: Set(13),
            value: Set(0),
            description: Set("Steam Sauna Water High".to_string()),
            enable: Set(true),
            protocol: Set("GPIO".to_string()),
            r#type: Set("IN".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("SS-WL".to_string()),
            pin: Set(35),
            port: Set(19),
            value: Set(0),
            description: Set("Steam Sauna Water Low".to_string()),
            enable: Set(true),
            protocol: Set("GPIO".to_string()),
            r#type: Set("IN".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("SS-Esans".to_string()),
            pin: Set(36),
            port: Set(16),
            value: Set(0),
            description: Set("Steam Sauna Esans".to_string()),
            enable: Set(true),
            protocol: Set("GPIO".to_string()),
            r#type: Set("OUT".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("SS-Takhlie".to_string()),
            pin: Set(37),
            port: Set(26),
            value: Set(0),
            description: Set("Steam Sauna Takhlie".to_string()),
            enable: Set(true),
            protocol: Set("GPIO".to_string()),
            r#type: Set("OUT".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("C-Light".to_string()),
            pin: Set(38),
            port: Set(20),
            value: Set(0),
            description: Set("Cold Light".to_string()),
            enable: Set(true),
            protocol: Set("GPIO".to_string()),
            r#type: Set("OUT".to_string()),
        },
        PortActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(1),
            name: Set("Abnama".to_string()),
            pin: Set(40),
            port: Set(21),
            value: Set(0),
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
        if result.status 
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
    
    if result.status {
        if let Some(ports) = result.data {
            println!("🔌 Found {} ports:", ports.len());
            println!("{:-<80}", "");
            for port in ports {
                let status = if port.enable { "✅ Enabled" } else { "❌ Disabled" };
                println!("ID: {} | Name: {} | Pin: {} | Protocol: {} | Type: {} | Status: {}", 
                    port.id, port.name, port.pin, port.protocol, port.r#type, status);
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
