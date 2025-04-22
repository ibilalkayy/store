use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Payment {
    #[sqlx(rename = "names")]
    #[serde(rename = "name")]
    pub name: String,

    #[sqlx(rename = "card_numbers")]
    #[serde(rename = "card_number")]
    pub card_number: i32,

    #[sqlx(rename = "cvvs")]
    #[serde(rename = "cvv")]
    pub cvv: i32,

    #[sqlx(rename = "amounts")]
    #[serde(rename = "amount")]
    pub amount: f32,

    #[sqlx(rename = "payment_statuses")]
    #[serde(rename = "payment_status")]
    pub payment_status: String,
}
