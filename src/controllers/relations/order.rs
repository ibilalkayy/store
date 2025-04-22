use actix_web::{
    get, post, web, HttpResponse, Responder, Error
};
use sqlx::PgPool;

use crate::controllers::apis::{address::AddressData, payment::PaymentData, user::UserData};

#[get("/orders/{id}/payments")]
pub async fn get_payments_by_order(
    order_id: web::Path<i32>,
    pool: web::Data<PgPool>
) -> Result<impl Responder, Error> {
    let order_id = order_id.into_inner();

    let orders = sqlx::query_as::<_, PaymentData>(
        r#"
        SELECT p.id, p.names, p.card_numbers, p.cvvs, p.amounts, p.payment_statuses
        FROM payments p
        JOIN payment_orders po ON p.id = payment_id
        WHERE po.order_id = $1
        "#,
    )
    .bind(order_id)
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    Ok(HttpResponse::Ok().json(orders))
}

#[post("/orders/{order_id}/users/{user_id}")]
pub async fn assign_user_to_order(
    path: web::Path<(i32, i32)>,
    pool: web::Data<PgPool>,
) -> Result<impl Responder, Error> {
    let (order_id, user_id) = path.into_inner();

    sqlx::query(
        r#"
        INSERT INTO order_users (order_id, user_id)
        VALUES ($1, $2)
        "#
    )
    .bind(order_id)
    .bind(user_id)
    .execute(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        actix_web::error::ErrorInternalServerError("Failed to assign user")
    })?;

    Ok(HttpResponse::Ok().body("User assigned to order successfully"))
}

#[get("/orders/{id}/users")]
pub async fn get_user_by_order(
    order_id: web::Path<i32>,
    pool: web::Data<PgPool>
) -> Result<impl Responder, Error> {
    let order_id = order_id.into_inner();

    let order = sqlx::query_as::<_, UserData> (
        r#"
        SELECT u.id, u.names, u.emails, u.password_hashes, u.roles
        FROM users u
        JOIN order_users ou ON u.id = ou.user_id
        WHERE ou.order_id = $1
        "#
    )
    .bind(order_id)
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    Ok(HttpResponse::Ok().json(order))
}

#[post("/orders/{order_id}/addresses/{address_id}")]
pub async fn assign_address_to_order(
    path: web::Path<(i32, i32)>,
    pool: web::Data<PgPool>,
) -> Result<impl Responder, Error> {
    let (order_id, address_id) = path.into_inner();

    sqlx::query(
        r#"
        INSERT INTO order_addresses (order_id, address_id)
        VALUES ($1, $2)
        "#
    )
    .bind(order_id)
    .bind(address_id)
    .execute(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        actix_web::error::ErrorInternalServerError("Failed to assign order")
    })?;

    Ok(HttpResponse::Ok().body("Address assigned to order successfully"))
}

#[get("/orders/{id}/addresses")]
pub async fn get_address_by_order(
    order_id: web::Path<i32>,
    pool: web::Data<PgPool>
) -> Result<impl Responder, Error> {
    let order_id = order_id.into_inner();

    let order = sqlx::query_as::<_, AddressData> (
        r#"
        SELECT a.id, a.first_names, a.last_names, a.addresses, a.town_cities, a.countries, a.zip_codes, a.phone_numbers, a.emails, a.passwords, a.notes
        FROM addresses a
        JOIN order_addresses ao ON a.id = ao.address_id
        WHERE ao.order_id = $1
        "#
    )
    .bind(order_id)
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    Ok(HttpResponse::Ok().json(order))
}