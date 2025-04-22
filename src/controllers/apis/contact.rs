use actix_web::{
    Error, HttpResponse, Responder, delete, error::ErrorNotFound, get, post, web,
};
use serde::Serialize;
use sqlx::PgPool;

use crate::models::entities::contact::Contact;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct ContactData {
    pub id: i32,

    #[sqlx(rename = "names")]
    pub name: String,

    #[sqlx(rename = "emails")]
    pub email: String,
    
    #[sqlx(rename = "messages")]
    pub message: String
}

#[derive(Serialize)]
pub struct ContactResponse {
    pub message: String,
    pub contact: Option<ContactData>
}

#[post("/contact")]
pub async fn create_contact(
    contact_data: web::Json<Contact>,
    pool: web::Data<PgPool>,
) -> Result<impl Responder, Error> {
    let contact = sqlx::query_as::<_, ContactData>(
        r#"
        INSERT INTO contact (names, emails, messages)
        VALUES ($1, $2, $3)
        RETURNING id, names, emails, messages
        "#,
    )
    .bind(&contact_data.name)
    .bind(&contact_data.email)
    .bind(&contact_data.message)
    .fetch_one(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    let response = ContactResponse {
        message: "Created successfully".to_string(),
        contact: Some(contact),
    };

    Ok(HttpResponse::Created().json(response))
}

#[get("/contact/{id}")]
pub async fn get_contact(
    contact_id: web::Path<i32>,
    pool: web::Data<PgPool>,
) -> Result<impl Responder, Error> {
    let contact_id = contact_id.into_inner();

    let contact = sqlx::query_as::<_, ContactData>(
        "SELECT id, names, emails, messages FROM contact WHERE id = $1",
    )
    .bind(contact_id)
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    match contact {
        Some(contact_data) => Ok(HttpResponse::Ok().json(contact_data)),
        None => Err(ErrorNotFound("Contact not found")),
    }
}

#[delete("/contact/{id}")]
pub async fn delete_contact(
    contact_id: web::Path<i32>,
    pool: web::Data<PgPool>,
) -> Result<impl Responder, Error> {
    let contact_id = contact_id.into_inner();

    let contact = sqlx::query_as::<_, Contact>(
        r#"
        DELETE FROM contact WHERE id = $1
        RETURNING names, emails, messages
        "#,
    )
    .bind(contact_id)
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    match contact {
        Some(_) => {
            let response = ContactResponse {
                message: "Deleted successfully".to_string(),
                contact: None,
            };
            Ok(HttpResponse::Ok().json(response))
        }
        None => Err(ErrorNotFound("Contact not found")),
    }
}
