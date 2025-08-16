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
pub use models::zone::{Entity as ZoneEntity, Model as ZoneModel, ActiveModel as ZoneActiveModel};
use sea_orm::ActiveValue::Set;

//--------------------------------------------------------------------------------- Class
#[tokio::main]
async fn main() -> Result<(), DbErr> 
{
    let db = Database::connect("postgres://postgres:123456@192.168.64.9:5432/raspberrypi").await?;

    // Insert
    let created = ZoneModel::insert(
        &db,
        ZoneActiveModel {
            user_id: Set(1),
            name: Set("Zone A".to_owned()),
            description: Set("Initial description".to_owned()),
            enable: Set(true),
            ..Default::default()
        },
    ).await?;
    println!("Inserted: {:?}", created);

    // Update
    let mut to_update: ZoneActiveModel = created.clone().into();
    to_update.name = Set("Zone A+".to_owned());
    to_update.description = Set("Updated description".to_owned());
    let updated = ZoneModel::update(&db, to_update).await?;
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
    // let rows_affected = ZoneModel::delete(&db, updated.id).await?;
    // println!("Deleted rows: {}", rows_affected);

    Ok(())
}