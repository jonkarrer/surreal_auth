#![allow(dead_code)]
use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: Thing,
    username: String,
    password: String,
    email: String,
    created_at: String,
}

impl User {
    pub fn new(username: &str, password: &str, email: &str) -> Self {
        Self {
            id: Thing::from(("user", username)),
            username: username.to_string(),
            password: password.to_string(),
            email: email.to_string(),
            created_at: String::from(""),
        }
    }
}

pub async fn create_user_table(database: Surreal<Client>) -> surrealdb::Result<()> {
    let sql = "
        DEFINE TABLE user SCHEMAFULL;
        DEFINE FIELD username ON TABLE user TYPE string;
        DEFINE FIELD password ON TABLE user TYPE string;
        DEFINE FIELD email ON TABLE user TYPE string;
        DEFINE FIELD created_at ON TABLE user TYPE datetime;
    ";

    database.query(sql).await?;

    Ok(())
}

pub async fn create_user(database: Surreal<Client>, user: User) -> surrealdb::Result<()> {
    let sql = "
    CREATE user CONTENT {
        id: $id,
        username: $username,
        password: $password,
        email: $email,
        created_at: time::now()
    };
    ";

    database
        .query(sql)
        .bind(("id", user.id))
        .bind(("username", user.username))
        .bind(("password", user.password))
        .bind(("email", user.email))
        .await?;

    Ok(())
}

pub async fn read_user(database: Surreal<Client>, username: &str) -> Option<User> {
    let user: Option<User> = match database.select(("user", username)).await {
        Ok(user) => user,
        Err(e) => {
            println!("read_user:{}", e);
            None
        }
    };
    user
}
