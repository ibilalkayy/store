use actix_web::{
    Error, HttpResponse, Responder, delete, error::ErrorNotFound, get, post, put, web,
};
use serde::Serialize;
use sqlx::PgPool;

use crate::models::entities::blog::Blog;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct BlogData {
    pub id: i32,

    #[sqlx(rename = "titles")]
    pub title: String,

    #[sqlx(rename = "publish_dates")]
    pub publish_date: String,

    #[sqlx(rename = "descriptions")]
    pub description: String,
}

#[derive(Serialize)]
pub struct BlogResponse {
    pub message: String,
    pub blog: Option<BlogData>
}

#[post("/blogs")]
pub async fn create_blog(
    blog_data: web::Json<Blog>,
    pool: web::Data<PgPool>,
) -> Result<impl Responder, Error> {
    let blog = sqlx::query_as::<_, BlogData>(
        r#"
        INSERT INTO blogs (titles, publish_dates, descriptions)
        VALUES ($1, $2, $3)
        RETURNING id, titles, publish_dates, descriptions
        "#,
    )
    .bind(&blog_data.title)
    .bind(&blog_data.publish_date)
    .bind(&blog_data.description)
    .fetch_one(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    let response = BlogResponse {
        message: "Created successfully".to_string(),
        blog: Some(blog),
    };

    Ok(HttpResponse::Created().json(response))
}

#[get("/blogs/{id}")]
pub async fn get_blog(
    blog_id: web::Path<i32>,
    pool: web::Data<PgPool>,
) -> Result<impl Responder, Error> {
    let blog_id = blog_id.into_inner();

    let blog = sqlx::query_as::<_, BlogData>(
        "SELECT id, titles, publish_dates, descriptions FROM blogs WHERE id = $1",
    )
    .bind(blog_id)
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    match blog {
        Some(blog_data) => Ok(HttpResponse::Ok().json(blog_data)),
        None => Err(ErrorNotFound("Blog not found")),
    }
}

#[put("/blogs/{id}")]
pub async fn update_blog(
    blog_data: web::Json<Blog>,
    blog_id: web::Path<i32>,
    pool: web::Data<PgPool>,
) -> Result<impl Responder, Error> {
    let blog_id = blog_id.into_inner();
    let blog = sqlx::query_as::<_, Blog>(
        r#"
        UPDATE blogs 
        SET titles = $1, publish_dates = $2, descriptions = $3
        WHERE id = $4
        RETURNING titles, publish_dates, descriptions
        "#,
    )
    .bind(&blog_data.title)
    .bind(&blog_data.publish_date)
    .bind(&blog_data.description)
    .bind(blog_id)
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    match blog {
        Some(blog_data) => {
            let response = BlogResponse {
                message: "Updated successfully".to_string(),
                blog: Some(BlogData {
                    id: blog_id,
                    title: blog_data.title,
                    publish_date: blog_data.publish_date,
                    description: blog_data.description
                }),
            };
            Ok(HttpResponse::Ok().json(response))
        }
        None => Err(ErrorNotFound("Blog not found")),
    }
}

#[delete("/blogs/{id}")]
pub async fn delete_blog(
    blog_id: web::Path<i32>,
    pool: web::Data<PgPool>,
) -> Result<impl Responder, Error> {
    let blog_id = blog_id.into_inner();

    let blog = sqlx::query_as::<_, Blog>(
        r#"
        DELETE FROM blogs WHERE id = $1
        RETURNING titles, publish_dates, descriptions
        "#,
    )
    .bind(blog_id)
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    match blog {
        Some(_) => {
            let response = BlogResponse {
                message: "Deleted successfully".to_string(),
                blog: None,
            };
            Ok(HttpResponse::Ok().json(response))
        }
        None => Err(ErrorNotFound("User not found")),
    }
}