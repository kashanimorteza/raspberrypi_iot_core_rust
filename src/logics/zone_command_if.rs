//--------------------------------------------------------------------------------- Location
// src/logics/zone_command_if.rs

//--------------------------------------------------------------------------------- Description
// CRUD logic methods for the ZoneCommandIf model

//--------------------------------------------------------------------------------- Import
use sea_orm::{ActiveModelTrait, DbConn, DbErr, EntityTrait};
use crate::models::zone_command_if::{ActiveModel as ZoneCommandIfActiveModel, Entity as ZoneCommandIfEntity, Model as ZoneCommandIfModel};

//--------------------------------------------------------------------------------- Methods
impl ZoneCommandIfModel 
{
	pub async fn select_all(db: &DbConn) -> Result<Vec<ZoneCommandIfModel>, DbErr> 
	{
		ZoneCommandIfEntity::find().all(db).await
	}

	pub async fn select_by_id(db: &DbConn, id: i32) -> Result<Option<ZoneCommandIfModel>, DbErr> 
	{
		ZoneCommandIfEntity::find_by_id(id).one(db).await
	}

	pub async fn insert(db: &DbConn, active: ZoneCommandIfActiveModel) -> Result<ZoneCommandIfModel, DbErr> 
	{
		active.insert(db).await
	}

	pub async fn update(db: &DbConn, active: ZoneCommandIfActiveModel) -> Result<ZoneCommandIfModel, DbErr> 
	{
		active.update(db).await
	}

	pub async fn delete(db: &DbConn, id: i32) -> Result<u64, DbErr> {
		let res = ZoneCommandIfEntity::delete_by_id(id).exec(db).await?;
		Ok(res.rows_affected)
	}
}


