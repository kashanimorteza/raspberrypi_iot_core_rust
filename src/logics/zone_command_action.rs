//--------------------------------------------------------------------------------- Location
// src/logics/zone_command_action.rs

//--------------------------------------------------------------------------------- Description
// CRUD logic methods for the ZoneCommandAction model

//--------------------------------------------------------------------------------- Import
use sea_orm::{ActiveModelTrait, DbConn, DbErr, EntityTrait};
use crate::models::zone_command_action::{ActiveModel as ZoneCommandActionActiveModel, Entity as ZoneCommandActionEntity, Model as ZoneCommandActionModel};

//--------------------------------------------------------------------------------- Methods
impl ZoneCommandActionModel 
{
	pub async fn select_all(db: &DbConn) -> Result<Vec<ZoneCommandActionModel>, DbErr> 
	{
		ZoneCommandActionEntity::find().all(db).await
	}

	pub async fn select_by_id(db: &DbConn, id: i32) -> Result<Option<ZoneCommandActionModel>, DbErr> 
	{
		ZoneCommandActionEntity::find_by_id(id).one(db).await
	}

	pub async fn insert(db: &DbConn, active: ZoneCommandActionActiveModel) -> Result<ZoneCommandActionModel, DbErr> 
	{
		active.insert(db).await
	}

	pub async fn update(db: &DbConn, active: ZoneCommandActionActiveModel) -> Result<ZoneCommandActionModel, DbErr> 
	{
		active.update(db).await
	}

	pub async fn delete(db: &DbConn, id: i32) -> Result<u64, DbErr> {
		let res = ZoneCommandActionEntity::delete_by_id(id).exec(db).await?;
		Ok(res.rows_affected)
	}
}


