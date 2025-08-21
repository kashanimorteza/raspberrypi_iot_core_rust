//--------------------------------------------------------------------------------- Location
// src/logics/admin.rs

//--------------------------------------------------------------------------------- Description
// This file contains admin logic for managing the system



//--------------------------------------------------------------------------------- Import
use crate::logics::{
    user, config, device, device_command, zone, zone_command, zone_command_if, zone_command_action,
    port, timer, timer_item, timer_device, timer_limit, log
};
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
        
        println!("\n📋 Step 7: Adding sample zone commands...");
        zone_command::add_sample_zone_commands(db).await?;
        
        println!("\n📋 Step 8: Adding sample zone command ifs...");
        zone_command_if::add_sample_zone_command_ifs(db).await?;
        
        println!("\n📋 Step 9: Adding sample zone command actions...");
        zone_command_action::add_sample_zone_command_actions(db).await?;
        
        println!("\n📋 Step 10: Adding sample timers...");
        timer::add_sample_timers(db).await?;
        
        println!("\n📋 Step 11: Adding sample timer items...");
        timer_item::add_sample_timer_items(db).await?;
        
        println!("\n📋 Step 12: Adding sample timer devices...");
        timer_device::add_sample_timer_devices(db).await?;
        
        println!("\n📋 Step 13: Adding sample timer limits...");
        timer_limit::add_sample_timer_limits(db).await?;
        
        println!("\n📋 Step 14: Adding sample logs...");
        log::add_sample_logs(db).await?;
        
        println!("\n{:=<80}", "");
        println!("✅ Admin: All sample data added successfully!");
        println!("🎉 Database is now populated with sample data for all {} models!", 14);
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
        
        println!("\n📋 Zone Commands:");
        zone_command::list_all_zone_commands(db).await?;
        
        println!("\n📋 Zone Command Ifs:");
        zone_command_if::list_all_zone_command_ifs(db).await?;
        
        println!("\n📋 Zone Command Actions:");
        zone_command_action::list_all_zone_command_actions(db).await?;
        
        println!("\n📋 Timers:");
        timer::list_all_timers(db).await?;
        
        println!("\n📋 Timer Items:");
        timer_item::list_all_timer_items(db).await?;
        
        println!("\n📋 Timer Devices:");
        timer_device::list_all_timer_devices(db).await?;
        
        println!("\n📋 Timer Limits:");
        timer_limit::list_all_timer_limits(db).await?;
        
        println!("\n📋 Logs:");
        log::list_all_logs(db).await?;
        
        println!("\n{:=<80}", "");
        println!("✅ Admin: All data listed successfully!");
        println!("📊 Listed data from all {} models!", 14);
        Ok(())
    }
}
