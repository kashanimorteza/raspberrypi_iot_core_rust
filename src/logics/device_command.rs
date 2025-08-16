//--------------------------------------------------------------------------------- Location
// src/logics/device_command.rs

//--------------------------------------------------------------------------------- Description
// CRUD logic methods for the DeviceCommand model

//--------------------------------------------------------------------------------- Import
use sea_orm::{ActiveModelTrait, DbConn, DbErr, EntityTrait};
use crate::models::device_command::{ActiveModel as DeviceCommandActiveModel, Entity as DeviceCommandEntity, Model as DeviceCommandModel};

//--------------------------------------------------------------------------------- Methods
impl DeviceCommandModel 
{
	pub async fn select_all(db: &DbConn) -> Result<Vec<DeviceCommandModel>, DbErr> 
	{
		DeviceCommandEntity::find().all(db).await
	}

	pub async fn select_by_id(db: &DbConn, id: i32) -> Result<Option<DeviceCommandModel>, DbErr> 
	{
		DeviceCommandEntity::find_by_id(id).one(db).await
	}

	pub async fn insert(db: &DbConn, active: DeviceCommandActiveModel) -> Result<DeviceCommandModel, DbErr> 
	{
		active.insert(db).await
	}

	pub async fn update(db: &DbConn, active: DeviceCommandActiveModel) -> Result<DeviceCommandModel, DbErr> 
	{
		active.update(db).await
	}

	pub async fn delete(db: &DbConn, id: i32) -> Result<u64, DbErr> {
		let res = DeviceCommandEntity::delete_by_id(id).exec(db).await?;
		Ok(res.rows_affected)
	}
}


