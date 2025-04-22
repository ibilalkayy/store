use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Product {
    #[sqlx(rename = "names")]
    #[serde(rename = "name")]
    pub name: String,

    #[sqlx(rename = "prices")]
    #[serde(rename = "price")]
    pub price: f32,

    #[sqlx(rename = "images")]
    #[serde(rename = "image")]
    pub image: String
}
