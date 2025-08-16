//--------------------------------------------------------------------------------- Location
// src/logics/port.rs

//--------------------------------------------------------------------------------- Description
// CRUD logic methods for the Port model

//--------------------------------------------------------------------------------- Import
use sea_orm::{ActiveModelTrait, DbConn, DbErr, EntityTrait};
use crate::models::port::{ActiveModel as PortActiveModel, Entity as PortEntity, Model as PortModel};

//--------------------------------------------------------------------------------- Methods
impl PortModel 
{
	pub async fn select_all(db: &DbConn) -> Result<Vec<PortModel>, DbErr> 
	{
		PortEntity::find().all(db).await
	}

	pub async fn select_by_id(db: &DbConn, id: i32) -> Result<Option<PortModel>, DbErr> 
	{
		PortEntity::find_by_id(id).one(db).await
	}

	pub async fn insert(db: &DbConn, active: PortActiveModel) -> Result<PortModel, DbErr> 
	{
		active.insert(db).await
	}

	pub async fn update(db: &DbConn, active: PortActiveModel) -> Result<PortModel, DbErr> 
	{
		active.update(db).await
	}

	pub async fn delete(db: &DbConn, id: i32) -> Result<u64, DbErr> {
		let res = PortEntity::delete_by_id(id).exec(db).await?;
		Ok(res.rows_affected)
	}
}


