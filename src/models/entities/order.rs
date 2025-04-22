use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]

pub struct Order {
    #[sqlx(rename = "products")]
    #[serde(rename = "product")]
    pub product: String,

    #[sqlx(rename = "quantities")]
    #[serde(rename = "quantity")]
    pub quantity: String,

    #[sqlx(rename = "totals")]
    #[serde(rename = "total")]
    pub total: String,

    #[sqlx(rename = "statuses")]
    #[serde(rename = "status")]
    pub status: String,

    #[sqlx(rename = "order_dates")]
    #[serde(rename = "order date")]
    pub order_date: String
}