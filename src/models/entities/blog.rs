use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Blog {
    #[sqlx(rename = "titles")]
    #[serde(rename = "title")]
    pub title: String,

    #[sqlx(rename = "publish_dates")]
    #[serde(rename = "publish_date")]
    pub publish_date: String,

    #[sqlx(rename = "descriptions")]
    #[serde(rename = "description")]
    pub description: String,
}
