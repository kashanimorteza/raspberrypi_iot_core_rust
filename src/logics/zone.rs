//--------------------------------------------------------------------------------- Location
// src/logics/zone.rs

//--------------------------------------------------------------------------------- Description
// CRUD logic methods for the Zone model

//--------------------------------------------------------------------------------- Import
use sea_orm::{ActiveModelTrait, DbConn, DbErr, EntityTrait};
use crate::models::zone::{ActiveModel as ZoneActiveModel, Entity as ZoneEntity, Model as ZoneModel};

//--------------------------------------------------------------------------------- Methods
impl ZoneModel 
{
	pub async fn select_all(db: &DbConn) -> Result<Vec<ZoneModel>, DbErr> 
	{
		ZoneEntity::find().all(db).await
	}

	pub async fn select_by_id(db: &DbConn, id: i32) -> Result<Option<ZoneModel>, DbErr> 
	{
		ZoneEntity::find_by_id(id).one(db).await
	}

	pub async fn insert(db: &DbConn, active: ZoneActiveModel) -> Result<ZoneModel, DbErr> 
	{
		active.insert(db).await
	}

	pub async fn update(db: &DbConn, active: ZoneActiveModel) -> Result<ZoneModel, DbErr> 
	{
		active.update(db).await
	}

	pub async fn delete(db: &DbConn, id: i32) -> Result<u64, DbErr> 
	{
		let res = ZoneEntity::delete_by_id(id).exec(db).await?;
		Ok(res.rows_affected)
	}
}


