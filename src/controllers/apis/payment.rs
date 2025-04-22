use actix_web::{
    Error, HttpResponse, Responder, delete, error::ErrorNotFound, get, post, put, web,
};
use serde::Serialize;
use sqlx::PgPool;

use crate::models::entities::payment::Payment;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct PaymentData {
    pub id: i32,

    #[sqlx(rename = "names")]
    pub name: String,

    #[sqlx(rename = "card_numbers")]
    pub card_number: i32,

    #[sqlx(rename = "cvvs")]
    pub cvv: i32,

    #[sqlx(rename = "amounts")]
    pub amount: f32,

    #[sqlx(rename = "payment_statuses")]
    pub payment_status: String,
}

#[derive(Serialize)]
pub struct PaymentResponse {
    pub message: String,
    pub payment: Option<PaymentData>
}

#[post("/payments")]
pub async fn create_payment(
    payment_data: web::Json<Payment>,
    pool: web::Data<PgPool>,
) -> Result<impl Responder, Error> {
    let payment = sqlx::query_as::<_, PaymentData>(
        r#"
        INSERT INTO payments (names, card_numbers, cvvs, amounts, payment_statuses)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, names, card_numbers, cvvs, amounts, payment_statuses
        "#,
    )
    .bind(&payment_data.name)
    .bind(&payment_data.card_number)
    .bind(&payment_data.cvv)
    .bind(&payment_data.amount)
    .bind(&payment_data.payment_status)
    .fetch_one(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    let response = PaymentResponse {
        message: "Created successfully".to_string(),
        payment: Some(payment)
    };

    Ok(HttpResponse::Created().json(response))
}

#[get("/payments/{id}")]
pub async fn get_payment(
    payment_id: web::Path<i32>,
    pool: web::Data<PgPool>,
) -> Result<impl Responder, Error> {
    let payment_id = payment_id.into_inner();

    let payment = sqlx::query_as::<_, PaymentData>(
        "SELECT id, names, card_numbers, cvvs, amounts, payment_statuses FROM payments WHERE id = $1",
    )
    .bind(payment_id)
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    match payment {
        Some(payment_data) => Ok(HttpResponse::Ok().json(payment_data)),
        None => Err(ErrorNotFound("Payment not found")),
    }
}

#[put("/payments/{id}")]
pub async fn update_payment(
    payment_data: web::Json<Payment>,
    payment_id: web::Path<i32>,
    pool: web::Data<PgPool>,
) -> Result<impl Responder, Error> {
    let payment_id = payment_id.into_inner();
    let payment = sqlx::query_as::<_, Payment>(
        r#"
        UPDATE payments 
        SET names = $1, card_numbers = $2, cvvs = $3, amounts = $4, payment_statuses = $5
        WHERE id = $6
        RETURNING names, card_numbers, cvvs, amounts, payment_statuses
        "#,
    )
    .bind(&payment_data.name)
    .bind(&payment_data.card_number)
    .bind(&payment_data.cvv)
    .bind(&payment_data.amount)
    .bind(&payment_data.payment_status)
    .bind(payment_id)
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    match payment {
        Some(payment_data) => {
            let response = PaymentResponse {
                message: "Updated successfully".to_string(),
                payment: Some(PaymentData {
                    id: payment_id,
                    name: payment_data.name,
                    card_number: payment_data.card_number,
                    cvv: payment_data.cvv,
                    amount: payment_data.amount,
                    payment_status: payment_data.payment_status
                }),
            };
            Ok(HttpResponse::Ok().json(response))
        }
        None => Err(ErrorNotFound("Amount not found")),
    }
}

#[delete("/payments/{id}")]
pub async fn delete_payment(
    payment_id: web::Path<i32>,
    pool: web::Data<PgPool>,
) -> Result<impl Responder, Error> {
    let payment_id = payment_id.into_inner();

    let payment = sqlx::query_as::<_, Payment>(
        r#"
        DELETE FROM payments WHERE id = $1
        RETURNING names, card_numbers, cvvs, amounts, payment_statuses
        "#,
    )
    .bind(payment_id)
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    match payment {
        Some(_) => {
            let response = PaymentResponse {
                message: "Deleted successfully".to_string(),
                payment: None,
            };
            Ok(HttpResponse::Ok().json(response))
        }
        None => Err(ErrorNotFound("Payment not found")),
    }
}