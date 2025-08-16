//--------------------------------------------------------------------------------- Location
// src/logics/timer_item.rs

//--------------------------------------------------------------------------------- Description
// CRUD logic methods for the TimerItem model

//--------------------------------------------------------------------------------- Import
use sea_orm::{ActiveModelTrait, DbConn, DbErr, EntityTrait};
use crate::models::timer_item::{ActiveModel as TimerItemActiveModel, Entity as TimerItemEntity, Model as TimerItemModel};

//--------------------------------------------------------------------------------- Methods
impl TimerItemModel 
{
	pub async fn select_all(db: &DbConn) -> Result<Vec<TimerItemModel>, DbErr> 
	{
		TimerItemEntity::find().all(db).await
	}

	pub async fn select_by_id(db: &DbConn, id: i32) -> Result<Option<TimerItemModel>, DbErr> 
	{
		TimerItemEntity::find_by_id(id).one(db).await
	}

	pub async fn insert(db: &DbConn, active: TimerItemActiveModel) -> Result<TimerItemModel, DbErr> 
	{
		active.insert(db).await
	}

	pub async fn update(db: &DbConn, active: TimerItemActiveModel) -> Result<TimerItemModel, DbErr> 
	{
		active.update(db).await
	}

	pub async fn delete(db: &DbConn, id: i32) -> Result<u64, DbErr> {
		let res = TimerItemEntity::delete_by_id(id).exec(db).await?;
		Ok(res.rows_affected)
	}
}


