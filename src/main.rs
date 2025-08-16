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

    // Example: read zones without modifying data
    let _ = ZoneModel::select_all(&db).await?;

    Ok(())
}