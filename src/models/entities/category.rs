use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Category {
    #[sqlx(rename = "names")]
    #[serde(rename = "name")]
    pub name: String,
}