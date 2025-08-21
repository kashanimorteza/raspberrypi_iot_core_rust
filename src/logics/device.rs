//--------------------------------------------------------------------------------- Location
// src/logics/device.rs

//--------------------------------------------------------------------------------- Description
// This file contains logic to add sample devices using the ORM

//--------------------------------------------------------------------------------- Import
use crate::orm::logics::device::DeviceORM;
use sea_orm::DatabaseConnection;
use crate::orm::models::device::ActiveModel as DeviceActiveModel;
use sea_orm::ActiveValue::Set;

//--------------------------------------------------------------------------------- Add Sample Devices Logic
pub async fn add_sample_devices(db: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> 
{
    let device_orm = DeviceORM::new(true, true);
    let sample_devices = vec![
        // Pool devices
        DeviceActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_id: Set(1),
            port_id: Set(20),
            power_id: Set(1),
            command_id: Set(1),
            value: Set(0),
            tune: Set(0),
            date: Set("2024-01-01".to_string()),
            address: Set("".to_string()),
            name: Set("Motor".to_string()),
            description: Set("Pool Motor".to_string()),
            enable: Set(true),
        },
        DeviceActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_id: Set(1),
            port_id: Set(21),
            power_id: Set(1),
            command_id: Set(1),
            value: Set(0),
            tune: Set(0),
            date: Set("2024-01-01".to_string()),
            address: Set("".to_string()),
            name: Set("Shir".to_string()),
            description: Set("Pool Shir".to_string()),
            enable: Set(true),
        },
        DeviceActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_id: Set(1),
            port_id: Set(22),
            power_id: Set(1),
            command_id: Set(1),
            value: Set(0),
            tune: Set(0),
            date: Set("2024-01-01".to_string()),
            address: Set("".to_string()),
            name: Set("Light".to_string()),
            description: Set("Pool Light".to_string()),
            enable: Set(true),
        },
        DeviceActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_id: Set(1),
            port_id: Set(19),
            power_id: Set(1),
            command_id: Set(1),
            value: Set(0),
            tune: Set(0),
            date: Set("2024-01-01".to_string()),
            address: Set("28-0922a03b54a4".to_string()),
            name: Set("Sensor".to_string()),
            description: Set("Pool Temperature Sensor".to_string()),
            enable: Set(true),
        },
        // Jacuzzi devices
        DeviceActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_id: Set(2),
            port_id: Set(23),
            power_id: Set(1),
            command_id: Set(1),
            value: Set(0),
            tune: Set(0),
            date: Set("2024-01-01".to_string()),
            address: Set("".to_string()),
            name: Set("Motor".to_string()),
            description: Set("Jacuzzi Motor".to_string()),
            enable: Set(true),
        },
        DeviceActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_id: Set(2),
            port_id: Set(24),
            power_id: Set(1),
            command_id: Set(1),
            value: Set(0),
            tune: Set(0),
            date: Set("2024-01-01".to_string()),
            address: Set("".to_string()),
            name: Set("Shir".to_string()),
            description: Set("Jacuzzi Shir".to_string()),
            enable: Set(true),
        },
        DeviceActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_id: Set(2),
            port_id: Set(25),
            power_id: Set(1),
            command_id: Set(1),
            value: Set(0),
            tune: Set(0),
            date: Set("2024-01-01".to_string()),
            address: Set("".to_string()),
            name: Set("Light".to_string()),
            description: Set("Jacuzzi Light".to_string()),
            enable: Set(true),
        },
        DeviceActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_id: Set(2),
            port_id: Set(26),
            power_id: Set(1),
            command_id: Set(1),
            value: Set(0),
            tune: Set(0),
            date: Set("2024-01-01".to_string()),
            address: Set("".to_string()),
            name: Set("Erjet-Fuse".to_string()),
            description: Set("Jacuzzi Erjet Fuse".to_string()),
            enable: Set(true),
        },
        DeviceActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_id: Set(2),
            port_id: Set(27),
            power_id: Set(1),
            command_id: Set(1),
            value: Set(0),
            tune: Set(0),
            date: Set("2024-01-01".to_string()),
            address: Set("".to_string()),
            name: Set("Erjet-Motor".to_string()),
            description: Set("Jacuzzi Erjet Motor".to_string()),
            enable: Set(true),
        },
        DeviceActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_id: Set(2),
            port_id: Set(19),
            power_id: Set(1),
            command_id: Set(1),
            value: Set(0),
            tune: Set(0),
            date: Set("2024-01-01".to_string()),
            address: Set("28-00000da95214".to_string()),
            name: Set("Sensor".to_string()),
            description: Set("Jacuzzi Temperature Sensor".to_string()),
            enable: Set(true),
        },
        // Sauna-Dry devices
        DeviceActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_id: Set(3),
            port_id: Set(28),
            power_id: Set(1),
            command_id: Set(1),
            value: Set(0),
            tune: Set(0),
            date: Set("2024-01-01".to_string()),
            address: Set("".to_string()),
            name: Set("Hiter".to_string()),
            description: Set("Sauna Dry Heater".to_string()),
            enable: Set(true),
        },
        DeviceActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_id: Set(3),
            port_id: Set(29),
            power_id: Set(1),
            command_id: Set(1),
            value: Set(0),
            tune: Set(0),
            date: Set("2024-01-01".to_string()),
            address: Set("".to_string()),
            name: Set("Light".to_string()),
            description: Set("Sauna Dry Light".to_string()),
            enable: Set(true),
        },
        DeviceActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_id: Set(3),
            port_id: Set(19),
            power_id: Set(1),
            command_id: Set(1),
            value: Set(0),
            tune: Set(0),
            date: Set("2024-01-01".to_string()),
            address: Set("".to_string()),
            name: Set("Sensor".to_string()),
            description: Set("Sauna Dry Temperature Sensor".to_string()),
            enable: Set(true),
        },
        // Fan devices
        DeviceActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_id: Set(4),
            port_id: Set(30),
            power_id: Set(1),
            command_id: Set(1),
            value: Set(0),
            tune: Set(0),
            date: Set("2024-01-01".to_string()),
            address: Set("".to_string()),
            name: Set("Motor".to_string()),
            description: Set("Fan Motor".to_string()),
            enable: Set(true),
        },
        DeviceActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_id: Set(4),
            port_id: Set(19),
            power_id: Set(1),
            command_id: Set(1),
            value: Set(0),
            tune: Set(0),
            date: Set("2024-01-01".to_string()),
            address: Set("".to_string()),
            name: Set("Sensor".to_string()),
            description: Set("Fan Temperature Sensor".to_string()),
            enable: Set(true),
        },
        // Cold devices
        DeviceActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_id: Set(5),
            port_id: Set(31),
            power_id: Set(1),
            command_id: Set(1),
            value: Set(0),
            tune: Set(0),
            date: Set("2024-01-01".to_string()),
            address: Set("".to_string()),
            name: Set("Motor".to_string()),
            description: Set("Cold Motor".to_string()),
            enable: Set(true),
        },
        DeviceActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_id: Set(5),
            port_id: Set(39),
            power_id: Set(1),
            command_id: Set(1),
            value: Set(0),
            tune: Set(0),
            date: Set("2024-01-01".to_string()),
            address: Set("".to_string()),
            name: Set("Light".to_string()),
            description: Set("Cold Light".to_string()),
            enable: Set(true),
        },
        // Steam Sauna devices
        DeviceActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_id: Set(6),
            port_id: Set(32),
            power_id: Set(1),
            command_id: Set(1),
            value: Set(0),
            tune: Set(0),
            date: Set("2024-01-01".to_string()),
            address: Set("".to_string()),
            name: Set("Shir".to_string()),
            description: Set("Steam Sauna Shir".to_string()),
            enable: Set(true),
        },
        DeviceActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_id: Set(6),
            port_id: Set(33),
            power_id: Set(1),
            command_id: Set(1),
            value: Set(0),
            tune: Set(0),
            date: Set("2024-01-01".to_string()),
            address: Set("".to_string()),
            name: Set("Hiter".to_string()),
            description: Set("Steam Sauna Heater".to_string()),
            enable: Set(true),
        },
        DeviceActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_id: Set(6),
            port_id: Set(34),
            power_id: Set(1),
            command_id: Set(1),
            value: Set(0),
            tune: Set(0),
            date: Set("2024-01-01".to_string()),
            address: Set("".to_string()),
            name: Set("Light".to_string()),
            description: Set("Steam Sauna Light".to_string()),
            enable: Set(true),
        },
        DeviceActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_id: Set(6),
            port_id: Set(35),
            power_id: Set(1),
            command_id: Set(1),
            value: Set(0),
            tune: Set(0),
            date: Set("2024-01-01".to_string()),
            address: Set("".to_string()),
            name: Set("Sensor-WH".to_string()),
            description: Set("Steam Sauna Water High Sensor".to_string()),
            enable: Set(true),
        },
        DeviceActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_id: Set(6),
            port_id: Set(36),
            power_id: Set(1),
            command_id: Set(1),
            value: Set(0),
            tune: Set(0),
            date: Set("2024-01-01".to_string()),
            address: Set("".to_string()),
            name: Set("Sensor-WL".to_string()),
            description: Set("Steam Sauna Water Low Sensor".to_string()),
            enable: Set(true),
        },
        DeviceActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_id: Set(6),
            port_id: Set(19),
            power_id: Set(1),
            command_id: Set(1),
            value: Set(0),
            tune: Set(0),
            date: Set("2024-01-01".to_string()),
            address: Set("28-0822a00f05d2".to_string()),
            name: Set("Sensor".to_string()),
            description: Set("Steam Sauna Temperature Sensor".to_string()),
            enable: Set(true),
        },
        DeviceActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_id: Set(6),
            port_id: Set(37),
            power_id: Set(1),
            command_id: Set(1),
            value: Set(0),
            tune: Set(0),
            date: Set("2024-01-01".to_string()),
            address: Set("".to_string()),
            name: Set("Esans".to_string()),
            description: Set("Steam Sauna Esans".to_string()),
            enable: Set(true),
        },
        DeviceActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_id: Set(6),
            port_id: Set(38),
            power_id: Set(1),
            command_id: Set(1),
            value: Set(0),
            tune: Set(0),
            date: Set("2024-01-01".to_string()),
            address: Set("".to_string()),
            name: Set("Takhlie".to_string()),
            description: Set("Steam Sauna Takhlie".to_string()),
            enable: Set(true),
        },
        // Abnama devices
        DeviceActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            zone_id: Set(7),
            port_id: Set(40),
            power_id: Set(1),
            command_id: Set(1),
            value: Set(0),
            tune: Set(0),
            date: Set("2024-01-01".to_string()),
            address: Set("".to_string()),
            name: Set("Motor".to_string()),
            description: Set("Abnama Motor".to_string()),
            enable: Set(true),
        },
    ];

    println!("🚀 Starting to add {} sample devices...", sample_devices.len());
    
    for (index, device) in sample_devices.into_iter().enumerate() 
    {
        let device_name = match &device.name 
        {
            Set(name) => name.clone(),
            _ => "Unknown".to_string(),
        };
        println!("📝 Adding device {}: {}", index + 1, device_name);
        
        let result = device_orm.add(db, device).await;
        if result.status 
        {
            if let Some(added_device) = result.data 
            {
                println!("✅ Successfully added device: {} (ID: {})", added_device.name, added_device.id);
            }
        } 
        else 
        {
            if let Some(error) = result.error 
            {
                println!("❌ Failed to add device: {}", error);
            }
        }
    }
    
    println!("🎉 Finished adding sample devices!");
    Ok(())
}

//--------------------------------------------------------------------------------- List All Devices Logic
pub async fn list_all_devices(db: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    let device_orm = DeviceORM::new(true, true);
    let filters = std::collections::HashMap::new();
    
    println!("📋 Fetching all devices...");
    
    let result = device_orm.items(db, filters).await;
    
    if result.status {
        if let Some(devices) = result.data {
            println!("🔌 Found {} devices:", devices.len());
            println!("{:-<80}", "");
            for device in devices {
                let status = if device.enable { "✅ Enabled" } else { "❌ Disabled" };
                println!("ID: {} | Name: {} | Address: {} | Zone: {} | Status: {}", 
                    device.id, device.name, device.address, device.zone_id, status);
            }
            println!("{:-<80}", "");
        } else {
            println!("📭 No devices found.");
        }
    } else {
        if let Some(error) = result.error {
            println!("❌ Error fetching devices: {}", error);
        }
    }
    
    Ok(())
}
