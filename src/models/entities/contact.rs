use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Contact {
    #[sqlx(rename = "names")]
    #[serde(rename = "name")]
    pub name: String,

    #[sqlx(rename = "emails")]
    #[serde(rename = "email")]
    pub email: String,

    #[sqlx(rename = "messages")]
    #[serde(rename = "message")]
    pub message: String
}