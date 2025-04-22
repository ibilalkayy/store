use actix_web::{
    get, web, HttpResponse, Responder, Error
};
use sqlx::PgPool;

use crate::controllers::apis::product::ProductData;

#[get("/categories/{id}/products")]
pub async fn get_products_by_category(
    category_id: web::Path<i32>,
    pool: web::Data<PgPool>
) -> Result<impl Responder, Error> {
    let category_id = category_id.into_inner();

    let products = sqlx::query_as::<_, ProductData>(
        r#"
        SELECT p.id, p.names, p.prices, p.images
        FROM products p
        JOIN product_categories pc ON p.id = product_id
        WHERE pc.category_id = $1
        "#,
    )
    .bind(category_id)
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    Ok(HttpResponse::Ok().json(products))
}