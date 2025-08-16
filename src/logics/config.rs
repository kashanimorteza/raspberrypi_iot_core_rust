//--------------------------------------------------------------------------------- Location
// src/logics/config.rs

//--------------------------------------------------------------------------------- Description
// CRUD logic methods for the Config model

//--------------------------------------------------------------------------------- Import
use sea_orm::{ActiveModelTrait, DbConn, DbErr, EntityTrait};
use crate::models::config::{ActiveModel as ConfigActiveModel, Entity as ConfigEntity, Model as ConfigModel};

//--------------------------------------------------------------------------------- Methods
impl ConfigModel 
{
	pub async fn select_all(db: &DbConn) -> Result<Vec<ConfigModel>, DbErr> 
	{
		ConfigEntity::find().all(db).await
	}

	pub async fn select_by_id(db: &DbConn, id: i32) -> Result<Option<ConfigModel>, DbErr> 
	{
		ConfigEntity::find_by_id(id).one(db).await
	}

	pub async fn insert(db: &DbConn, active: ConfigActiveModel) -> Result<ConfigModel, DbErr> 
	{
		active.insert(db).await
	}

	pub async fn update(db: &DbConn, active: ConfigActiveModel) -> Result<ConfigModel, DbErr> 
	{
		active.update(db).await
	}

	pub async fn delete(db: &DbConn, id: i32) -> Result<u64, DbErr> {
		let res = ConfigEntity::delete_by_id(id).exec(db).await?;
		Ok(res.rows_affected)
	}
}


