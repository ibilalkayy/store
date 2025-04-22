use actix_web::{
    Error, HttpResponse, Responder, delete, error::ErrorNotFound, get, post, put, web,
};
use serde::Serialize;
use sqlx::PgPool;

use crate::models::entities::order::Order;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct OrderData {
    pub id: i32, 

    #[sqlx(rename = "products")]
    pub product: String,

    #[sqlx(rename = "quantities")]
    pub quantity: String,

    #[sqlx(rename = "totals")]
    pub total: String,

    #[sqlx(rename = "statuses")]
    pub status: String,

    #[sqlx(rename = "order_dates")]
    pub order_date: String
}

#[derive(Serialize)]
pub struct OrderResponse {
    pub message: String,
    pub order: Option<OrderData>
}

#[post("/orders")]
pub async fn create_order(
    order_data: web::Json<Order>,
    pool: web::Data<PgPool>,
) -> Result<impl Responder, Error> {
    let order = sqlx::query_as::<_, OrderData>(
        r#"
        INSERT INTO orders (products, quantities, totals, statuses, order_dates)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, products, quantities, totals, statuses, order_dates
        "#,
    )
    .bind(&order_data.product)
    .bind(&order_data.quantity)
    .bind(&order_data.total)
    .bind(&order_data.status)
    .bind(&order_data.order_date)
    .fetch_one(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    let response = OrderResponse {
        message: "Created successfully".to_string(),
        order: Some(order)
    };

    Ok(HttpResponse::Created().json(response))
}

#[get("/orders/{id}")]
pub async fn get_order(
    order_id: web::Path<i32>,
    pool: web::Data<PgPool>,
) -> Result<impl Responder, Error> {
    let order_id = order_id.into_inner();

    let order = sqlx::query_as::<_, OrderData>(
        "SELECT id, products, quantities, totals, statuses, order_dates 
        FROM orders WHERE id = $1",
    )
    .bind(order_id)
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    match order {
        Some(order_data) => Ok(HttpResponse::Ok().json(order_data)),
        None => Err(ErrorNotFound("Order not found")),
    }
}

#[put("/orders/{id}")]
pub async fn update_order(
    order_data: web::Json<Order>,
    order_id: web::Path<i32>,
    pool: web::Data<PgPool>,
) -> Result<impl Responder, Error> {
    let order_id = order_id.into_inner();
    let order = sqlx::query_as::<_, Order>(
        r#"
        UPDATE orders 
        SET products = $1, quantities = $2, totals = $3, statuses = $4, order_dates = $5
        WHERE id = $6
        RETURNING products, quantities, totals, statuses, order_dates
        "#,
    )
    .bind(&order_data.product)
    .bind(&order_data.quantity)
    .bind(&order_data.total)
    .bind(&order_data.status)
    .bind(&order_data.order_date)
    .bind(order_id)
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    match order {
        Some(order_data) => {
            let response = OrderResponse {
                message: "Updated successfully".to_string(),
                order: Some(OrderData {
                    id: order_id,
                    product: order_data.product,
                    quantity: order_data.quantity,
                    total: order_data.total,
                    status: order_data.status,
                    order_date: order_data.order_date
                }),
            };
            Ok(HttpResponse::Ok().json(response))
        }
        None => Err(ErrorNotFound("Address not found")),
    }
}

#[delete("/orders/{id}")]
pub async fn delete_order(
    order_id: web::Path<i32>,
    pool: web::Data<PgPool>,
) -> Result<impl Responder, Error> {
    let order_id = order_id.into_inner();

    sqlx::query(
        r#"
        DELETE FROM payment_orders WHERE order_id = $1
        "#
    )
    .bind(order_id)
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    sqlx::query(
        r#"
        DELETE FROM order_addresses WHERE order_id = $1
        "#
    )
    .bind(order_id)
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    let order = sqlx::query_as::<_, Order>(
        r#"
        DELETE FROM orders WHERE id = $1
        RETURNING products, quantities, totals, statuses, order_dates
        "#,
    )
    .bind(order_id)
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    match order {
        Some(_) => {
            let response = OrderResponse {
                message: "Deleted successfully".to_string(),
                order: None,
            };
            Ok(HttpResponse::Ok().json(response))
        }
        None => Err(ErrorNotFound("Order not found")),
    }
}