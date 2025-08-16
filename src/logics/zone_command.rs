//--------------------------------------------------------------------------------- Location
// src/logics/zone_command.rs

//--------------------------------------------------------------------------------- Description
// CRUD logic methods for the ZoneCommand model

//--------------------------------------------------------------------------------- Import
use sea_orm::{ActiveModelTrait, DbConn, DbErr, EntityTrait};
use crate::models::zone_command::{ActiveModel as ZoneCommandActiveModel, Entity as ZoneCommandEntity, Model as ZoneCommandModel};

//--------------------------------------------------------------------------------- Methods
impl ZoneCommandModel 
{
	pub async fn select_all(db: &DbConn) -> Result<Vec<ZoneCommandModel>, DbErr> 
	{
		ZoneCommandEntity::find().all(db).await
	}

	pub async fn select_by_id(db: &DbConn, id: i32) -> Result<Option<ZoneCommandModel>, DbErr> 
	{
		ZoneCommandEntity::find_by_id(id).one(db).await
	}

	pub async fn insert(db: &DbConn, active: ZoneCommandActiveModel) -> Result<ZoneCommandModel, DbErr> 
	{
		active.insert(db).await
	}

	pub async fn update(db: &DbConn, active: ZoneCommandActiveModel) -> Result<ZoneCommandModel, DbErr> 
	{
		active.update(db).await
	}

	pub async fn delete(db: &DbConn, id: i32) -> Result<u64, DbErr> {
		let res = ZoneCommandEntity::delete_by_id(id).exec(db).await?;
		Ok(res.rows_affected)
	}
}


