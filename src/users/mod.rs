use serde::Deserialize;
use sqlx::prelude::FromRow;

use crate::Db;

pub mod actions;
pub mod auth;
pub mod views;
pub mod extractors;

#[derive(Deserialize, Debug, FromRow, PartialEq, Default, Clone)]
pub struct User {
    pub id: Option<u32>,
    pub name: String,
    pub password: Option<String>,
}

pub async fn find_user_by_name(db: &Db, name: String) -> Option<User> {
    sqlx::query_as::<_, User>("select * from users where name = ?")
        .bind(name)
        .fetch_one(db)
        .await
        .ok()
}

pub async fn find_user_by_id(db: &Db, id: u32) -> Option<User> {
    sqlx::query_as::<_, User>("select * from users where id = ?")
        .bind(id)
        .fetch_one(db)
        .await
        .ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_db;
    use crate::users::auth::register_user;

    #[tokio::test]
    async fn it_can_find_a_user_by_name() {
        let db = test_db().await;
        register_user("Kramer".to_string(), "secret", &db)
            .await
            .unwrap();
        let user = find_user_by_name(&db, "Kramer".to_string()).await.unwrap();

        assert_eq!(user.name, "Kramer".to_string());
        assert_eq!(user.id, Some(1));
    }

    #[tokio::test]
    async fn it_can_find_a_user_by_id() {
        let db = test_db().await;
        register_user("Kramer".to_string(), "secret", &db)
            .await
            .unwrap();
        let user = find_user_by_id(&db, 1).await.unwrap();

        assert_eq!(user.name, "Kramer".to_string());
        assert_eq!(user.id, Some(1));
    }
}
