use anyhow::Result;
use sqlx::prelude::FromRow;

use crate::Db;

pub mod actions;
mod views;

#[derive(FromRow, Debug)]
pub struct Room {
    pub id: Option<u32>,
    pub name: String,
}

pub async fn create_room(db: &Db, room: String) -> Result<()> {
    sqlx::query("insert into rooms (name) values (?1)")
        .bind(room.to_lowercase())
        .execute(db)
        .await?;

    Ok(())
}

pub async fn get_room_by_id(db: &Db, id: u32) -> Result<Room> {
    let room = sqlx::query_as::<_, Room>("select * from rooms where id = ?")
        .bind(id)
        .fetch_one(db)
        .await?;

    Ok(room)
}

pub async fn get_room_by_name(db: &Db, name: &String) -> Result<Room> {
    let room = sqlx::query_as::<_, Room>("select * from rooms where name = ?")
        .bind(name)
        .fetch_one(db)
        .await?;

    Ok(room)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_db;

    #[tokio::test]
    async fn it_can_create_rooms() {
        let db = test_db().await;

        // it lowercases the name
        create_room(&db, "General".to_string())
            .await
            .expect("error creating room");

        let room = sqlx::query_as::<_, Room>("select * from rooms")
            .fetch_one(&db)
            .await
            .expect("failed to create room");

        assert_eq!(room.name, "general".to_string());
        assert_eq!(room.id, Some(1));
    }

    #[tokio::test]
    async fn it_can_get_a_room_by_id() {
        let db = test_db().await;

        create_room(&db, "general".to_string())
            .await
            .expect("error creating room");
        create_room(&db, "another".to_string())
            .await
            .expect("error creating room");

        let room = get_room_by_id(&db, 2).await.expect("geez get a room!");

        assert_eq!(room.name, "another".to_string());
        assert_eq!(room.id, Some(2));
    }

    #[tokio::test]
    async fn it_can_get_a_room_by_name() {
        let db = test_db().await;

        create_room(&db, "general".to_string())
            .await
            .expect("error creating room");
        create_room(&db, "another".to_string())
            .await
            .expect("error creating room");

        let room = get_room_by_name(&db, &"another".to_string())
            .await
            .expect("geez get a room!");

        assert_eq!(room.name, "another".to_string());
        assert_eq!(room.id, Some(2));
    }
}
