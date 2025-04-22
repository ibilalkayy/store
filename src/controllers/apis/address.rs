use actix_web::{
    Error, HttpResponse, Responder, delete, error::ErrorNotFound, get, post, put, web,
};
use serde::Serialize;
use sqlx::PgPool;

use crate::models::entities::address::Address;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct AddressData {
    pub id: i32, 

    #[sqlx(rename = "first_names")]
    pub first_name: String,

    #[sqlx(rename = "last_names")]
    pub last_name: String,

    #[sqlx(rename = "addresses")]
    pub address: String,

    #[sqlx(rename = "town_cities")]
    pub town_or_city: String,

    #[sqlx(rename = "countries")]
    pub country: String,

    #[sqlx(rename = "zip_codes")]
    pub zip_code: i32,

    #[sqlx(rename = "phone_numbers")]
    pub phone_number: i32,

    #[sqlx(rename = "emails")]
    pub email: String,

    #[sqlx(rename = "passwords")]
    pub password: String,

    #[sqlx(rename = "notes")]
    pub note: String,
}

#[derive(Serialize)]
pub struct AddressResponse {
    pub message: String,
    pub address: Option<AddressData>
}

#[post("/addresses")]
pub async fn create_address(
    address_data: web::Json<Address>,
    pool: web::Data<PgPool>,
) -> Result<impl Responder, Error> {
    let address = sqlx::query_as::<_, AddressData>(
        r#"
        INSERT INTO addresses (first_names, last_names, addresses, town_cities, countries, zip_codes, phone_numbers, emails, passwords, notes)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        RETURNING id, first_names, last_names, addresses, town_cities, countries, zip_codes, phone_numbers, emails, passwords, notes
        "#,
    )
    .bind(&address_data.first_name)
    .bind(&address_data.last_name)
    .bind(&address_data.address)
    .bind(&address_data.town_or_city)
    .bind(&address_data.country)
    .bind(&address_data.zip_code)
    .bind(&address_data.phone_number)
    .bind(&address_data.email)
    .bind(&address_data.password)
    .bind(&address_data.note)
    .fetch_one(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    let response = AddressResponse {
        message: "Created successfully".to_string(),
        address: Some(address)
    };

    Ok(HttpResponse::Created().json(response))
}

#[get("/addresses/{id}")]
pub async fn get_address(
    address_id: web::Path<i32>,
    pool: web::Data<PgPool>,
) -> Result<impl Responder, Error> {
    let address_id = address_id.into_inner();

    let address = sqlx::query_as::<_, AddressData>(
        "SELECT id, first_names, last_names, addresses, town_cities, countries, zip_codes, phone_numbers, emails, passwords, notes 
        FROM addresses WHERE id = $1",
    )
    .bind(address_id)
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    match address {
        Some(address_data) => Ok(HttpResponse::Ok().json(address_data)),
        None => Err(ErrorNotFound("Address not found")),
    }
}

#[put("/addresses/{id}")]
pub async fn update_address(
    address_data: web::Json<Address>,
    address_id: web::Path<i32>,
    pool: web::Data<PgPool>,
) -> Result<impl Responder, Error> {
    let address_id = address_id.into_inner();
    let address = sqlx::query_as::<_, Address>(
        r#"
        UPDATE addresses 
        SET first_names = $1, last_names = $2, addresses = $3, town_cities = $4, countries = $5, zip_codes = $6, phone_numbers = $7, emails = $8, passwords = $9, notes = $10
        WHERE id = $11
        RETURNING first_names, last_names, addresses, town_cities, countries, zip_codes, phone_numbers, emails, passwords, notes
        "#,
    )
    .bind(&address_data.first_name)
    .bind(&address_data.last_name)
    .bind(&address_data.address)
    .bind(&address_data.town_or_city)
    .bind(&address_data.country)
    .bind(&address_data.zip_code)
    .bind(&address_data.phone_number)
    .bind(&address_data.email)
    .bind(&address_data.password)
    .bind(&address_data.note)
    .bind(address_id)
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    match address {
        Some(address_data) => {
            let response = AddressResponse {
                message: "Updated successfully".to_string(),
                address: Some(AddressData {
                    id: address_id,
                    first_name: address_data.first_name,
                    last_name: address_data.last_name,
                    address: address_data.address,
                    town_or_city: address_data.town_or_city,
                    country: address_data.country,
                    zip_code: address_data.zip_code,
                    phone_number: address_data.phone_number,
                    email: address_data.email,
                    password: address_data.password,
                    note: address_data.note
                }),
            };
            Ok(HttpResponse::Ok().json(response))
        }
        None => Err(ErrorNotFound("Address not found")),
    }
}

#[delete("/addresses/{id}")]
pub async fn delete_address(
    address_id: web::Path<i32>,
    pool: web::Data<PgPool>,
) -> Result<impl Responder, Error> {
    let address_id = address_id.into_inner();

    let address = sqlx::query_as::<_, Address>(
        r#"
        DELETE FROM addresses WHERE id = $1
        RETURNING first_names, last_names, addresses, town_cities, countries, zip_codes, phone_numbers, emails, passwords, notes
        "#,
    )
    .bind(address_id)
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    match address {
        Some(_) => {
            let response = AddressResponse {
                message: "Deleted successfully".to_string(),
                address: None,
            };
            Ok(HttpResponse::Ok().json(response))
        }
        None => Err(ErrorNotFound("Address not found")),
    }
}