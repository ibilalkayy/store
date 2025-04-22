use actix_web::{
    get, post, put, delete, web, HttpResponse, Responder, error::ErrorNotFound, Error
};

use serde::Serialize;
use sqlx::PgPool;

use crate::models::entities::product::Product;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct ProductData {
    pub id: i32,

    #[sqlx(rename = "names")]
    pub name: String,

    #[sqlx(rename = "prices")]
    pub price: f32,
    
    #[sqlx(rename = "images")]
    pub image: String
}

#[derive(Serialize)]
pub struct ProductResponse {
    pub message: String,
    pub product: Option<ProductData>
}

#[post("/products")]
pub async fn create_product(
    product_data: web::Json<Product>,
    pool: web::Data<PgPool>
) -> Result<impl Responder, Error> {
    let product = sqlx::query_as::<_, ProductData>(
        r#"
        INSERT INTO products (names, prices, images)
        VALUES ($1, $2, $3)
        RETURNING id, names, prices, images
        "#,
    )
    .bind(&product_data.name)
    .bind(&product_data.price)
    .bind(&product_data.image)
    .fetch_one(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    let response = ProductResponse {
        message: "Created successfully".to_string(),
        product: Some(product),
    };

    Ok(HttpResponse::Created().json(response))
}

#[get("/products/{id}")]
pub async fn get_product(
    product_id: web::Path<i32>,
    pool: web::Data<PgPool>,
) -> Result<impl Responder, actix_web::Error> {
    let product_id = product_id.into_inner();

    let product = sqlx::query_as::<_, ProductData>(
        "SELECT id, names, prices, images FROM products WHERE id = $1",
    )
    .bind(product_id)
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    match product {
        Some(product_data) => Ok(HttpResponse::Ok().json(product_data)),
        None => Err(actix_web::error::ErrorNotFound("Product not found")),
    }
}

#[put("/products/{id}")]
pub async fn update_product(
    product_data: web::Json<Product>,
    product_id: web::Path<i32>,
    pool: web::Data<PgPool>,
) -> Result<impl Responder, Error> {
    let product_id = product_id.into_inner();

    let product = sqlx::query_as::<_, Product>(
        r#"
        UPDATE products
        SET names = $1, prices = $2, images = $3 
        WHERE id = $4
        RETURNING id, names, prices, images 
        "#,
    )
    .bind(&product_data.name)
    .bind(&product_data.price)
    .bind(&product_data.image)
    .bind(product_id)
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error {:?}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    match product {
        Some(product_data) => {
            let response = ProductResponse {
                message: "Updated successfully".to_string(),
                product: Some(ProductData { 
                    id: product_id,
                    name: product_data.name,
                    price: product_data.price,
                    image: product_data.image
                }),
            };
            Ok(HttpResponse::Ok().json(response))
        },
        None => Err(ErrorNotFound("Product not found"))
    }
}

#[delete("/products/{id}")]
pub async fn delete_product(
    product_id: web::Path<i32>,
    pool: web::Data<PgPool>,
) -> Result<impl Responder, Error> {
    let product_id = product_id.into_inner();

    let product = sqlx::query_as::<_, Product>(
        r#"
        DELETE FROM products WHERE id = $1
        RETURNING names, prices, images
        "#,
    )
    .bind(product_id)
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    match product {
        Some(_) => {
            let response = ProductResponse {
                message: "Deleted successfully".to_string(),
                product: None,
            };
            Ok(HttpResponse::Ok().json(response))
        }
        None => Err(ErrorNotFound("Product not found")),
    }
}