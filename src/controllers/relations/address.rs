// use actix_web::{
//     get, post, web, HttpResponse, Responder, Error
// };
// use sqlx::PgPool;

// use crate::controllers::apis::{order::OrderData, user::UserData};

// #[post("/addresses/{address_id}/users/{user_id}")]
// pub async fn assign_user_to_address(
//     path: web::Path<(i32, i32)>,
//     pool: web::Data<PgPool>,
// ) -> Result<impl Responder, Error> {
//     let (address_id, user_id) = path.into_inner();

//     sqlx::query(
//         r#"
//         INSERT INTO address_users (address_id, user_id)
//         VALUES ($1, $2)
//         "#
//     )
//     .bind(address_id)
//     .bind(user_id)
//     .execute(pool.get_ref())
//     .await
//     .map_err(|e| {
//         eprintln!("Database error: {:?}", e);
//         actix_web::error::ErrorInternalServerError("Failed to assign user")
//     })?;

//     Ok(HttpResponse::Ok().body("User assigned to address successfully"))
// }

// #[get("/addresses/{id}/users")]
// pub async fn get_user_by_address(
//     address_id: web::Path<i32>,
//     pool: web::Data<PgPool>
// ) -> Result<impl Responder, Error> {
//     let address_id = address_id.into_inner();

//     let address = sqlx::query_as::<_, UserData> (
//         r#"
//         SELECT u.id, u.names, u.emails, u.password_hashes, u.roles
//         FROM users u
//         JOIN address_users au ON u.id = au.user_id
//         WHERE au.address_id = $1
//         "#
//     )
//     .bind(address_id)
//     .fetch_all(pool.get_ref())
//     .await
//     .map_err(|e| {
//         eprintln!("Database error: {:?}", e);
//         actix_web::error::ErrorInternalServerError(e)
//     })?;

//     Ok(HttpResponse::Ok().json(address))
// }

// #[get("/addresses/{id}/orders")]
// pub async fn get_orders_by_address(
//     address_id: web::Path<i32>,
//     pool: web::Data<PgPool>
// ) -> Result<impl Responder, Error> {
//     let address_id = address_id.into_inner();

//     let addresses = sqlx::query_as::<_, OrderData>(
//         r#"
//         SELECT o.id, o.products, o.quantities, o.totals, o.statuses, o.order_dates
//         FROM orders o
//         JOIN order_addresses oa ON o.id = order_id
//         WHERE oa.address_id = $1
//         "#,
//     )
//     .bind(address_id)
//     .fetch_all(pool.get_ref())
//     .await
//     .map_err(|e| {
//         eprintln!("Database error: {:?}", e);
//         actix_web::error::ErrorInternalServerError(e)
//     })?;

//     Ok(HttpResponse::Ok().json(addresses))
// }