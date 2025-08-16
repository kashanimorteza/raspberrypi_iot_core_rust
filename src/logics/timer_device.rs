//--------------------------------------------------------------------------------- Location
// src/logics/timer_device.rs

//--------------------------------------------------------------------------------- Description
// CRUD logic methods for the TimerDevice model

//--------------------------------------------------------------------------------- Import
use sea_orm::{ActiveModelTrait, DbConn, DbErr, EntityTrait};
use crate::models::timer_device::{ActiveModel as TimerDeviceActiveModel, Entity as TimerDeviceEntity, Model as TimerDeviceModel};

//--------------------------------------------------------------------------------- Methods
impl TimerDeviceModel 
{
	pub async fn select_all(db: &DbConn) -> Result<Vec<TimerDeviceModel>, DbErr> 
	{
		TimerDeviceEntity::find().all(db).await
	}

	pub async fn select_by_id(db: &DbConn, id: i32) -> Result<Option<TimerDeviceModel>, DbErr> 
	{
		TimerDeviceEntity::find_by_id(id).one(db).await
	}

	pub async fn insert(db: &DbConn, active: TimerDeviceActiveModel) -> Result<TimerDeviceModel, DbErr> 
	{
		active.insert(db).await
	}

	pub async fn update(db: &DbConn, active: TimerDeviceActiveModel) -> Result<TimerDeviceModel, DbErr> 
	{
		active.update(db).await
	}

	pub async fn delete(db: &DbConn, id: i32) -> Result<u64, DbErr> {
		let res = TimerDeviceEntity::delete_by_id(id).exec(db).await?;
		Ok(res.rows_affected)
	}
}


