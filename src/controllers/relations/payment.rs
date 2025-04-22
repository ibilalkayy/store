use actix_web::{
    get, post, web, HttpResponse, Responder, Error
};
use sqlx::PgPool;

use crate::controllers::apis::{order::OrderData, user::UserData};

#[post("/payments/{payment_id}/orders/{order_id}")]
pub async fn assign_payment_to_order(
    path: web::Path<(i32, i32)>,
    pool: web::Data<PgPool>,
) -> Result<impl Responder, Error> {
    let (payment_id, order_id) = path.into_inner();

    sqlx::query(
        r#"
        INSERT INTO payment_orders (payment_id, order_id)
        VALUES ($1, $2)
        "#
    )
    .bind(payment_id)
    .bind(order_id)
    .execute(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        actix_web::error::ErrorInternalServerError("Failed to assign payment")
    })?;

    Ok(HttpResponse::Ok().body("Payment assigned to order successfully"))
}

#[get("/payments/{id}/orders")]
pub async fn get_order_by_payment(
    payment_id: web::Path<i32>,
    pool: web::Data<PgPool>
) -> Result<impl Responder, Error> {
    let payment_id = payment_id.into_inner();

    let payment = sqlx::query_as::<_, OrderData> (
        r#"
        SELECT o.id, o.products, o.quantities, o.totals, o.statuses, o.order_dates
        FROM orders o
        JOIN payment_orders po ON o.id = po.order_id
        WHERE po.payment_id = $1
        "#
    )
    .bind(payment_id)
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    Ok(HttpResponse::Ok().json(payment))
}

#[post("/payments/{payment_id}/users/{user_id}")]
pub async fn assign_payment_to_user(
    path: web::Path<(i32, i32)>,
    pool: web::Data<PgPool>,
) -> Result<impl Responder, Error> {
    let (payment_id, user_id) = path.into_inner();

    sqlx::query(
        r#"
        INSERT INTO payment_users (payment_id, user_id)
        VALUES ($1, $2)
        "#
    )
    .bind(payment_id)
    .bind(user_id)
    .execute(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        actix_web::error::ErrorInternalServerError("Failed to assign payment")
    })?;

    Ok(HttpResponse::Ok().body("Payment assigned to user successfully"))
}

#[get("/payments/{id}/users")]
pub async fn get_user_by_payment(
    payment_id: web::Path<i32>,
    pool: web::Data<PgPool>
) -> Result<impl Responder, Error> {
    let payment_id = payment_id.into_inner();

    let payment = sqlx::query_as::<_, UserData> (
        r#"
        SELECT u.id, u.names, u.emails, u.password_hashes, u.roles
        FROM users u
        JOIN payment_users pu ON u.id = pu.user_id
        WHERE pu.payment_id = $1
        "#
    )
    .bind(payment_id)
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    Ok(HttpResponse::Ok().json(payment))
}