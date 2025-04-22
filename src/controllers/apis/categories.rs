use actix_web:: {
    get, post, put, delete, web, HttpResponse, Responder, error::ErrorNotFound, Error 
};

use sqlx::PgPool;
use serde::Serialize;

use crate::models::entities::category::Category;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct CategoryData {
    pub id: i32,

    #[sqlx(rename = "names")]
    pub name: String
}

#[derive(Serialize)]
pub struct CategoryResponse {
    pub message: String,
    pub category: Option<CategoryData>
}

#[post("/category")]
pub async fn create_category(
    category_data: web::Json<Category>,
    pool: web::Data<PgPool>
) -> Result<impl Responder, Error> {
    let category = sqlx::query_as::<_, CategoryData>(
        r#"
        INSERT INTO categories (names) VALUES ($1)
        RETURNING id, names
        "#,
    )
    .bind(&category_data.name)
    .fetch_one(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    let response = CategoryResponse {
        message: "Created successfully".to_string(),
        category: Some(category)
    };

    Ok(HttpResponse::Created().json(response))
}

#[get("/categories")]
pub async fn get_categories(
    pool: web::Data<PgPool>
) -> Result<impl Responder, Error> {
    let categories = sqlx::query_as::<_, CategoryData>(
        r#"
        SELECT id, names FROM categories
        "#,
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    Ok(HttpResponse::Ok().json(categories))
}

#[put("/categories/{id}")]
pub async fn update_category(
    category_data: web::Json<Category>,
    category_id: web::Path<i32>,
    pool: web::Data<PgPool>
) -> Result<impl Responder, Error> {
    let category_id = category_id.into_inner();

    let category = sqlx::query_as::<_, Category>(
        r#"
        UPDATE categories
        SET names = $1 WHERE id = $2
        RETURNING names
        "#,
    )
    .bind(&category_data.name)
    .bind(category_id)
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    match category {
        Some(category_data) => {
            let response = CategoryResponse {
                message: "Updated successfully".to_string(),
                category: Some(CategoryData {
                    id: category_id,
                    name: category_data.name
                }),
            };
            Ok(HttpResponse::Ok().json(response))
        },
        None => Err(ErrorNotFound("Category not found")) 
    }
}

#[delete("/categories/{id}")]
pub async fn delete_category(
    category_id: web::Path<i32>,
    pool: web::Data<PgPool>
) -> Result<impl Responder, Error> {
    let category_id = category_id.into_inner();

    sqlx::query(
        r#"
        DELETE FROM product_categories WHERE category_id = $1
        "#
    )
    .bind(category_id)
    .execute(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    let category = sqlx::query_as::<_, Category>(
        r#"
        DELETE FROM categories WHERE id = $1
        RETURNING names
        "#,
    )
    .bind(category_id)
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    match category {
        Some(_) => {
            let response = CategoryResponse {
                message: "Deleted successfully".to_string(),
                category: None
            };
            Ok(HttpResponse::Ok().json(response))
        }
        None => Err(ErrorNotFound("Product not found"))
    }
}