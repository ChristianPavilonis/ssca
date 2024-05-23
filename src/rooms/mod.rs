use std::error::Error;

use crate::Db;

pub mod actions;
mod views;

async fn create_room(db: &Db, room: String) -> Result<(), Box<dyn Error>> {
    sqlx::query("insert into rooms (name) values (?1)")
        .bind(room)
        .execute(db)
        .await?;

    Ok(())
}
