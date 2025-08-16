//--------------------------------------------------------------------------------- Location
// src/main.rs

//--------------------------------------------------------------------------------- Description
// This is main

//--------------------------------------------------------------------------------- Import
pub use sea_orm::entity::prelude::*;
pub use sea_orm::Database;
pub mod models;
pub mod logics;
pub use sea_orm::{EntityTrait, DbConn, DbErr};
pub use models::zone::{Entity as ZoneEntity, Model as ZoneModel};

//--------------------------------------------------------------------------------- Class
#[tokio::main]
async fn main() -> Result<(), DbErr> 
{
    let db = Database::connect("postgres://postgres:123456@192.168.64.9:5432/raspberrypi").await?;

    // Insert
    let created = ZoneModel::insert(&db, 1, "Zone A".to_owned(), "Initial description".to_owned(), true).await?;
    println!("Inserted: {:?}", created);

    // Update
    let updated = ZoneModel::update(
        &db,
        created.id,
        None, // keep user_id
        Some("Zone A+".to_owned()),
        Some("Updated description".to_owned()),
        None, // keep enable
    ).await?;
    println!("Updated: {:?}", updated);

    // Select by id
    let fetched = ZoneModel::select_by_id(&db, updated.id).await?;
    println!("Selected by id: {:?}", fetched);

    // Select all
    let zones = ZoneModel::select_all(&db).await?;
    println!("Select all ({} rows)", zones.len());
    for z in zones {
        println!("{:?}", z);
    }

    // Delete
    let rows_affected = ZoneModel::delete(&db, updated.id).await?;
    println!("Deleted rows: {}", rows_affected);

    Ok(())
}