//--------------------------------------------------------------------------------- Location
// src/logics/admin.rs

//--------------------------------------------------------------------------------- Description
// This file contains admin logic for managing the system



//--------------------------------------------------------------------------------- Import
use crate::logics::{user, config, device, device_command, zone, port, timer, log};
use sea_orm::DatabaseConnection;



//--------------------------------------------------------------------------------- Admin Logic
pub struct Admin;

impl Admin 
{
    pub fn new() -> Self 
    {
        Self
    }

    //-------------------------- [Add Samples]
    pub async fn add_samples(&self, db: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔧 Admin: Starting to add sample data for all models...");
        println!("{:=<80}", "");
        
        // Add sample data for all models in logical order
        println!("📋 Step 1: Adding sample users...");
        user::add_sample_users(db).await?;
        
        println!("\n📋 Step 2: Adding sample configurations...");
        config::add_sample_configs(db).await?;
        
        println!("\n📋 Step 3: Adding sample zones...");
        zone::add_sample_zones(db).await?;
        
        println!("\n📋 Step 4: Adding sample ports...");
        port::add_sample_ports(db).await?;
        
        println!("\n📋 Step 5: Adding sample devices...");
        device::add_sample_devices(db).await?;
        
        println!("\n📋 Step 6: Adding sample device commands...");
        device_command::add_sample_device_commands(db).await?;
        
        println!("\n📋 Step 7: Adding sample timers...");
        timer::add_sample_timers(db).await?;
        
        println!("\n📋 Step 8: Adding sample logs...");
        log::add_sample_logs(db).await?;
        
        println!("\n{:=<80}", "");
        println!("✅ Admin: All sample data added successfully!");
        println!("🎉 Database is now populated with sample data for all models!");
        Ok(())
    }

    //-------------------------- [List All Data]
    pub async fn list_all_data(&self, db: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔧 Admin: Listing all data from all models...");
        println!("{:=<80}", "");
        
        // List data from all models
        println!("📋 Users:");
        user::list_all_users(db).await?;
        
        println!("\n📋 Configurations:");
        config::list_all_configs(db).await?;
        
        println!("\n📋 Zones:");
        zone::list_all_zones(db).await?;
        
        println!("\n📋 Ports:");
        port::list_all_ports(db).await?;
        
        println!("\n📋 Devices:");
        device::list_all_devices(db).await?;
        
        println!("\n📋 Device Commands:");
        device_command::list_all_device_commands(db).await?;
        
        println!("\n📋 Timers:");
        timer::list_all_timers(db).await?;
        
        println!("\n📋 Logs:");
        log::list_all_logs(db).await?;
        
        println!("\n{:=<80}", "");
        println!("✅ Admin: All data listed successfully!");
        Ok(())
    }
}
