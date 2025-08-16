//--------------------------------------------------------------------------------- Location
// src/logics/timer_limit.rs

//--------------------------------------------------------------------------------- Description
// CRUD logic methods for the TimerLimit model

//--------------------------------------------------------------------------------- Import
use sea_orm::{ActiveModelTrait, DbConn, DbErr, EntityTrait};
use crate::models::timer_limit::{ActiveModel as TimerLimitActiveModel, Entity as TimerLimitEntity, Model as TimerLimitModel};

//--------------------------------------------------------------------------------- Methods
impl TimerLimitModel 
{
	pub async fn select_all(db: &DbConn) -> Result<Vec<TimerLimitModel>, DbErr> 
	{
		TimerLimitEntity::find().all(db).await
	}

	pub async fn select_by_id(db: &DbConn, id: i32) -> Result<Option<TimerLimitModel>, DbErr> 
	{
		TimerLimitEntity::find_by_id(id).one(db).await
	}

	pub async fn insert(db: &DbConn, active: TimerLimitActiveModel) -> Result<TimerLimitModel, DbErr> 
	{
		active.insert(db).await
	}

	pub async fn update(db: &DbConn, active: TimerLimitActiveModel) -> Result<TimerLimitModel, DbErr> 
	{
		active.update(db).await
	}

	pub async fn delete(db: &DbConn, id: i32) -> Result<u64, DbErr> {
		let res = TimerLimitEntity::delete_by_id(id).exec(db).await?;
		Ok(res.rows_affected)
	}
}


