use crate::users::User;
use crate::Db;
use anyhow::Result;
use bcrypt::DEFAULT_COST;

pub async fn register_user<P>(name: String, password: P, db: &Db) -> Result<User>
where
    P: AsRef<[u8]>,
{
    let hash = bcrypt::hash(password, DEFAULT_COST)?;

    sqlx::query("insert into users (name, password) values (?1, ?2)")
        .bind(&name)
        .bind(hash)
        .execute(db)
        .await?;

    Ok(User {
        name,
        ..Default::default()
    })
}

pub fn check_password(user: &User, password: String) -> bool {
    match &user.password {
        None => false,
        Some(hash) => bcrypt::verify(password, hash.as_str())
            .ok()
            .unwrap_or(false),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_db;

    #[tokio::test]
    async fn it_can_register_a_user() {
        let db = test_db().await;

        let user = register_user("Kramer".to_string(), "secret", &db)
            .await
            .unwrap();

        let result = sqlx::query_as::<_, User>("select * from users where id = ?1")
            .bind(1)
            .fetch_one(&db)
            .await
            .unwrap();

        assert_eq!(user.name, result.name);
    }

    #[test]
    fn it_can_check_credentials() {
        let user = User {
            name: "Kramer".to_string(),
            password: Some(
                "$2b$12$FgzcneSFxrV36/CI6iMSAeNruqOO1M7h3TiUWhbj1cKWuCfud57Fu".to_string(),
            ),
            ..Default::default()
        };

        assert!(!check_password(&user, "poop".to_string()));
        assert!(check_password(&user, "secret".to_string()));
    }
}
