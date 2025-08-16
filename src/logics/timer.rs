//--------------------------------------------------------------------------------- Location
// src/logics/timer.rs

//--------------------------------------------------------------------------------- Description
// CRUD logic methods for the Timer model

//--------------------------------------------------------------------------------- Import
use sea_orm::{ActiveModelTrait, DbConn, DbErr, EntityTrait};
use crate::models::timer::{ActiveModel as TimerActiveModel, Entity as TimerEntity, Model as TimerModel};

//--------------------------------------------------------------------------------- Methods
impl TimerModel 
{
	pub async fn select_all(db: &DbConn) -> Result<Vec<TimerModel>, DbErr> 
	{
		TimerEntity::find().all(db).await
	}

	pub async fn select_by_id(db: &DbConn, id: i32) -> Result<Option<TimerModel>, DbErr> 
	{
		TimerEntity::find_by_id(id).one(db).await
	}

	pub async fn insert(db: &DbConn, active: TimerActiveModel) -> Result<TimerModel, DbErr> 
	{
		active.insert(db).await
	}

	pub async fn update(db: &DbConn, active: TimerActiveModel) -> Result<TimerModel, DbErr> 
	{
		active.update(db).await
	}

	pub async fn delete(db: &DbConn, id: i32) -> Result<u64, DbErr> {
		let res = TimerEntity::delete_by_id(id).exec(db).await?;
		Ok(res.rows_affected)
	}
}


