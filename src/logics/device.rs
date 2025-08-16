//--------------------------------------------------------------------------------- Location
// src/logics/device.rs

//--------------------------------------------------------------------------------- Description
// CRUD logic methods for the Device model

//--------------------------------------------------------------------------------- Import
use sea_orm::{ActiveModelTrait, DbConn, DbErr, EntityTrait};
use crate::models::device::{ActiveModel as DeviceActiveModel, Entity as DeviceEntity, Model as DeviceModel};

//--------------------------------------------------------------------------------- Methods
impl DeviceModel 
{
	pub async fn select_all(db: &DbConn) -> Result<Vec<DeviceModel>, DbErr> 
	{
		DeviceEntity::find().all(db).await
	}

	pub async fn select_by_id(db: &DbConn, id: i32) -> Result<Option<DeviceModel>, DbErr> 
	{
		DeviceEntity::find_by_id(id).one(db).await
	}

	pub async fn insert(db: &DbConn, active: DeviceActiveModel) -> Result<DeviceModel, DbErr> 
	{
		active.insert(db).await
	}

	pub async fn update(db: &DbConn, active: DeviceActiveModel) -> Result<DeviceModel, DbErr> 
	{
		active.update(db).await
	}

	pub async fn delete(db: &DbConn, id: i32) -> Result<u64, DbErr> {
		let res = DeviceEntity::delete_by_id(id).exec(db).await?;
		Ok(res.rows_affected)
	}
}


