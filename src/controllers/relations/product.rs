// use actix_web::{
//     get, post, web, HttpResponse, Responder, Error
// };
// use sqlx::PgPool;

// use crate::controllers::apis::categories::CategoryData;

// #[post("/products/{product_id}/categories/{category_id}")]
// pub async fn assign_category_to_product(
//     path: web::Path<(i32, i32)>,
//     pool: web::Data<PgPool>,
// ) -> Result<impl Responder, Error> {
//     let (product_id, category_id) = path.into_inner();

//     sqlx::query(
//         r#"
//         INSERT INTO product_categories (product_id, category_id)
//         VALUES ($1, $2)
//         "#
//     )
//     .bind(product_id)
//     .bind(category_id)
//     .execute(pool.get_ref())
//     .await
//     .map_err(|e| {
//         eprintln!("Database error: {:?}", e);
//         actix_web::error::ErrorInternalServerError("Failed to assign category")
//     })?;

//     Ok(HttpResponse::Ok().body("Category assigned to product successfully"))
// }

// #[get("/products/{id}/categories")]
// pub async fn get_categories_by_product(
//     product_id: web::Path<i32>,
//     pool: web::Data<PgPool>
// ) -> Result<impl Responder, Error> {
//     let product_id = product_id.into_inner();

//     let categories = sqlx::query_as::<_, CategoryData>(
//         r#"
//         SELECT c.id, c.names
//         FROM categories c
//         JOIN product_categories pc ON c.id = pc.category_id
//         WHERE pc.product_id = $1
//         "#
//     )
//     .bind(product_id)
//     .fetch_all(pool.get_ref())
//     .await
//     .map_err(|e| {
//         eprintln!("Database error: {:?}", e);
//         actix_web::error::ErrorInternalServerError(e)
//     })?;

//     Ok(HttpResponse::Ok().json(categories))
// }