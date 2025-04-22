use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    #[sqlx(rename = "names")]
    #[serde(rename = "name")]
    pub name: String,

    #[sqlx(rename = "emails")]
    #[serde(rename = "email")]
    pub email: String,

    #[sqlx(rename = "password_hashes")]
    #[serde(rename = "password_hash")]
    pub password_hash: String,

    #[sqlx(rename = "roles")]
    #[serde(rename = "role")]    
    pub role: String,
}
