use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize, FromRow)]
pub struct NewUser {
    #[sqlx(rename = "names")]
    pub name: String,

    #[sqlx(rename = "emails")]
    pub email: String,

    #[sqlx(rename = "passwords")]
    pub password: String,

    #[sqlx(rename = "roles")]
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,

    #[sqlx(rename = "names")]
    pub name: String,

    #[sqlx(rename = "emails")]
    pub email: String,

    #[sqlx(rename = "passwords")]
    pub password: String,

    #[sqlx(rename = "roles")]
    pub role: String
}
