use actix_web::{
    get, post, web, HttpResponse, Responder, Error
};
use sqlx::PgPool;

use crate::controllers::apis::user::UserData;

#[post("/blogs/{blog_id}/users/{user_id}")]
pub async fn assign_user_to_blog(
    path: web::Path<(i32, i32)>,
    pool: web::Data<PgPool>,
) -> Result<impl Responder, Error> {
    let (blog_id, user_id) = path.into_inner();

    sqlx::query(
        r#"
        INSERT INTO blog_users (blog_id, user_id)
        VALUES ($1, $2)
        "#
    )
    .bind(blog_id)
    .bind(user_id)
    .execute(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        actix_web::error::ErrorInternalServerError("Failed to assign user")
    })?;

    Ok(HttpResponse::Ok().body("User assigned to blog successfully"))
}

#[get("/blogs/{id}/users")]
pub async fn get_user_by_blog(
    blog_id: web::Path<i32>,
    pool: web::Data<PgPool>
) -> Result<impl Responder, Error> {
    let blog_id = blog_id.into_inner();

    let blog = sqlx::query_as::<_, UserData> (
        r#"
        SELECT u.id, u.names, u.emails, u.password_hashes, u.roles
        FROM users u
        JOIN blog_users bu ON u.id = bu.user_id
        WHERE bu.blog_id = $1
        "#
    )
    .bind(blog_id)
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    Ok(HttpResponse::Ok().json(blog))
}
