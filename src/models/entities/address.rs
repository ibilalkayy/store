use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Address {
    #[sqlx(rename = "first_names")]
    #[serde(rename = "first name")]
    pub first_name: String,

    #[sqlx(rename = "last_names")]
    #[serde(rename = "last name")]
    pub last_name: String,

    #[sqlx(rename = "addresses")]
    #[serde(rename = "address")]
    pub address: String,

    #[sqlx(rename = "town_cities")]
    #[serde(rename = "town or city")]
    pub town_or_city: String,

    #[sqlx(rename = "countries")]
    #[serde(rename = "country")]
    pub country: String,

    #[sqlx(rename = "zip_codes")]
    #[serde(rename = "zip code")]
    pub zip_code: i32,

    #[sqlx(rename = "phone_numbers")]
    #[serde(rename = "phone number")]
    pub phone_number: i32,

    #[sqlx(rename = "emails")]
    #[serde(rename = "email")]
    pub email: String,

    #[sqlx(rename = "passwords")]
    #[serde(rename = "password")]
    pub password: String,

    #[sqlx(rename = "notes")]
    #[serde(rename = "note")]
    pub note: String,
}
