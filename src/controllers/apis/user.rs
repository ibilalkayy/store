use actix_web::{
    Error, HttpResponse, Responder, delete, error::ErrorNotFound, get, post, put, web,
};
use serde::Serialize;
use sqlx::PgPool;

use crate::models::entities::user::User;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct UserData {
    pub id: i32,

    #[sqlx(rename = "names")]
    pub name: String,

    #[sqlx(rename = "emails")]
    pub email: String,

    #[sqlx(rename = "password_hashes")]
    pub password_hash: String,
    
    #[sqlx(rename = "roles")]
    pub role: String
}

#[derive(Serialize)]
pub struct UserResponse {
    pub message: String,
    pub user: Option<UserData>
}

#[post("/users")]
pub async fn create_user(
    user_data: web::Json<User>,
    pool: web::Data<PgPool>,
) -> Result<impl Responder, Error> {
    let user = sqlx::query_as::<_, UserData>(
        r#"
        INSERT INTO users (names, emails, password_hashes, roles)
        VALUES ($1, $2, $3, $4)
        RETURNING id, names, emails, password_hashes, roles
        "#,
    )
    .bind(&user_data.name)
    .bind(&user_data.email)
    .bind(&user_data.password_hash)
    .bind(&user_data.role)
    .fetch_one(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    let response = UserResponse {
        message: "Created successfully".to_string(),
        user: Some(user),
    };

    Ok(HttpResponse::Created().json(response))
}

#[get("/users/{id}")]
pub async fn get_user(
    user_id: web::Path<i32>,
    pool: web::Data<PgPool>,
) -> Result<impl Responder, Error> {
    let user_id = user_id.into_inner();

    let user = sqlx::query_as::<_, UserData>(
        "SELECT id, names, emails, password_hashes, roles FROM users WHERE id = $1",
    )
    .bind(user_id)
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    match user {
        Some(user_data) => Ok(HttpResponse::Ok().json(user_data)),
        None => Err(ErrorNotFound("User not found")),
    }
}

#[put("/users/{id}")]
pub async fn update_user(
    user_data: web::Json<User>,
    user_id: web::Path<i32>,
    pool: web::Data<PgPool>,
) -> Result<impl Responder, Error> {
    let user_id = user_id.into_inner();
    let user = sqlx::query_as::<_, User>(
        r#"
        UPDATE users 
        SET names = $1, emails = $2, password_hashes = $3, roles = $4
        WHERE id = $5
        RETURNING names, emails, password_hashes, roles
        "#,
    )
    .bind(&user_data.name)
    .bind(&user_data.email)
    .bind(&user_data.password_hash)
    .bind(&user_data.role)
    .bind(user_id)
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    match user {
        Some(user_data) => {
            let response = UserResponse {
                message: "Updated successfully".to_string(),
                user: Some(UserData {
                    id: user_id,
                    name: user_data.name,
                    email: user_data.email,
                    password_hash: user_data.password_hash,
                    role: user_data.role,
                }),
            };
            Ok(HttpResponse::Ok().json(response))
        }
        None => Err(ErrorNotFound("User not found")),
    }
}

#[delete("/users/{id}")]
pub async fn delete_user(
    user_id: web::Path<i32>,
    pool: web::Data<PgPool>,
) -> Result<impl Responder, Error> {
    let user_id = user_id.into_inner();

    sqlx::query(
        r#"
        DELETE FROM blog_users WHERE user_id = $1
        "#
    )
    .bind(user_id)
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    sqlx::query(
        r#"
        DELETE FROM payment_users WHERE user_id = $1
        "#
    )
    .bind(user_id)
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    sqlx::query(
        r#"
        DELETE FROM address_users WHERE user_id = $1
        "#
    )
    .bind(user_id)
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    sqlx::query(
        r#"
        DELETE FROM order_users WHERE user_id = $1
        "#
    )
    .bind(user_id)
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;
    
    let user = sqlx::query_as::<_, User>(
        r#"
        DELETE FROM users WHERE id = $1
        RETURNING names, emails, password_hashes, roles
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    match user {
        Some(_) => {
            let response = UserResponse {
                message: "Deleted successfully".to_string(),
                user: None,
            };
            Ok(HttpResponse::Ok().json(response))
        }
        None => Err(ErrorNotFound("User not found")),
    }
}