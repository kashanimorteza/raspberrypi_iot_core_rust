//--------------------------------------------------------------------------------- Location
// src/logics/log.rs

//--------------------------------------------------------------------------------- Description
// CRUD logic methods for the Log model

//--------------------------------------------------------------------------------- Import
use sea_orm::{ActiveModelTrait, DbConn, DbErr, EntityTrait};
use crate::models::log::{ActiveModel as LogActiveModel, Entity as LogEntity, Model as LogModel};

//--------------------------------------------------------------------------------- Methods
impl LogModel 
{
	pub async fn select_all(db: &DbConn) -> Result<Vec<LogModel>, DbErr> 
	{
		LogEntity::find().all(db).await
	}

	pub async fn select_by_id(db: &DbConn, id: i32) -> Result<Option<LogModel>, DbErr> 
	{
		LogEntity::find_by_id(id).one(db).await
	}

	pub async fn insert(db: &DbConn, active: LogActiveModel) -> Result<LogModel, DbErr> 
	{
		active.insert(db).await
	}

	pub async fn update(db: &DbConn, active: LogActiveModel) -> Result<LogModel, DbErr> 
	{
		active.update(db).await
	}

	pub async fn delete(db: &DbConn, id: i32) -> Result<u64, DbErr> {
		let res = LogEntity::delete_by_id(id).exec(db).await?;
		Ok(res.rows_affected)
	}
}


