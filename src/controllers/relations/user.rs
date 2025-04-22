use actix_web::{
    get, web, HttpResponse, Responder, Error
};
use sqlx::PgPool;

use crate::controllers::apis::{address::AddressData, blogs::BlogData, order::OrderData, payment::PaymentData};

#[get("/users/{id}/blogs")]
pub async fn get_blogs_by_user(
    user_id: web::Path<i32>,
    pool: web::Data<PgPool>
) -> Result<impl Responder, Error> {
    let user_id = user_id.into_inner();

    let blogs = sqlx::query_as::<_, BlogData>(
        r#"
        SELECT b.id, b.titles, b.publish_dates, b.descriptions
        FROM blogs b
        JOIN blog_users bu ON b.id = blog_id
        WHERE bu.user_id = $1
        "#,
    )
    .bind(user_id)
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    Ok(HttpResponse::Ok().json(blogs))
}

#[get("/users/{id}/payments")]
pub async fn get_payments_by_user(
    user_id: web::Path<i32>,
    pool: web::Data<PgPool>
) -> Result<impl Responder, Error> {
    let user_id = user_id.into_inner();

    let users = sqlx::query_as::<_, PaymentData>(
        r#"
        SELECT p.id, p.names, p.card_numbers, p.cvvs, p.amounts, p.payment_statuses
        FROM payments p
        JOIN payment_users pu ON p.id = payment_id
        WHERE pu.user_id = $1
        "#,
    )
    .bind(user_id)
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    Ok(HttpResponse::Ok().json(users))
}

#[get("/users/{id}/addresses")]
pub async fn get_addresses_by_user(
    user_id: web::Path<i32>,
    pool: web::Data<PgPool>
) -> Result<impl Responder, Error> {
    let user_id = user_id.into_inner();

    let users = sqlx::query_as::<_, AddressData>(
        r#"
        SELECT a.id, a.first_names, a.last_names, a.addresses, a.town_cities, a.countries, a.zip_codes, a.phone_numbers, a.emails, a.passwords, a.notes
        FROM addresses a
        JOIN address_users au ON a.id = address_id
        WHERE au.user_id = $1
        "#,
    )
    .bind(user_id)
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    Ok(HttpResponse::Ok().json(users))
}

#[get("/users/{id}/orders")]
pub async fn get_orders_by_user(
    user_id: web::Path<i32>,
    pool: web::Data<PgPool>
) -> Result<impl Responder, Error> {
    let user_id = user_id.into_inner();

    let users = sqlx::query_as::<_, OrderData>(
        r#"
        SELECT o.id, o.products, o.quantities, o.totals, o.statuses, o.order_dates
        FROM orders o
        JOIN order_users ou ON o.id = order_id
        WHERE ou.user_id = $1
        "#,
    )
    .bind(user_id)
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    Ok(HttpResponse::Ok().json(users))
}