use actix_web::{
    web::Form, web::Data, web::Path, HttpResponse, http::header, Responder, get, post,
};
use sqlx::PgPool;
use tera::{Tera, Context};

use crate::models::entities::user::{NewUser, User};

#[post("/")]
pub async fn create_user(
    form: Form<NewUser>,
    pool: Data<PgPool>,
) -> impl Responder {
    let user_data = form.into_inner();
    sqlx::query ("INSERT INTO users (names, emails, passwords, roles) VALUES ($1, $2, $3, $4)")
        .bind(&user_data.name)
        .bind(&user_data.email)
        .bind(&user_data.password)
        .bind(&user_data.role)
        .execute(pool.get_ref())
        .await
        .unwrap();

    HttpResponse::SeeOther()
        .insert_header((header::LOCATION, "/"))
        .finish()
}

#[get("/user/{id}")]
pub async fn get_user(
    user_id: Path<i32>,
    tmpl: Data<Tera>,
    pool: Data<PgPool>,
) -> impl Responder {
    let user_id = user_id.into_inner();
    let user = sqlx::query_as::<_, User>("SELECT id, names, emails, passwords, roles FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_optional(pool.get_ref())
        .await
        .unwrap();

    match user {
        Some(user_data) => {
            let mut context = Context::new();
            context.insert("user", &user_data);
            let rendered = tmpl.render("user/user_info.html", &context).unwrap();
            HttpResponse::Ok().body(rendered)
        }
        None => HttpResponse::SeeOther()
            .insert_header((header::LOCATION, "/page_not_found"))
            .finish()
    }
}

#[post("/update/{id}")]
pub async fn update_user(
    user_id: Path<i32>,
    form: Form<NewUser>,
    pool: Data<PgPool>,
    tmpl: Data<Tera>
) -> impl Responder {
    sqlx::query("UPDATE users SET names = $1, emails = $2, passwords = $3, roles = $4 WHERE id = $5")
        .bind(&form.name)
        .bind(&form.email)
        .bind(&form.password)
        .bind(&form.role)
        .bind(*user_id)
        .execute(pool.get_ref())
        .await
        .unwrap();

    let user = sqlx::query_as::<_, User>(
        "SELECT id, names, emails, passwords, roles FROM users WHERE id = $1"
    )
    .bind(*user_id)
    .fetch_one(pool.get_ref())
    .await
    .unwrap();

    let mut ctx = Context::new();
    ctx.insert("user", &user);

    let rendered = tmpl.render("user/user_row.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[get("/edit_user/{id}")]
pub async fn edit_user(
    user_id: Path<i32>,
    tmpl: Data<Tera>,
    pool: Data<PgPool>
) -> impl Responder {
    let user = sqlx::query_as::<_, User>(
        "SELECT id, names, emails, passwords, roles FROM users WHERE id = $1"
    )
    .bind(*user_id)
    .fetch_one(pool.get_ref())
    .await
    .unwrap();

    let mut ctx = Context::new();
    ctx.insert("user", &user);

    let rendered = tmpl.render("user/user_edit.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[post("/delete_user/{id}")]
pub async fn delete_user(
    user_id: Path<i32>,
    pool: Data<PgPool>,
) -> impl Responder {
    sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(user_id.into_inner())
        .execute(pool.get_ref())
        .await
        .unwrap();
    
    HttpResponse::Ok().body("")
}

// #[delete("/users/{id}")]
// pub async fn delete_user(
//     user_id: web::Path<i32>,
//     pool: web::Data<PgPool>,
// ) -> Result<impl Responder, Error> {
//     let user_id = user_id.into_inner();

//     sqlx::query(
//         r#"
//         DELETE FROM blog_users WHERE user_id = $1
//         "#
//     )
//     .bind(user_id)
//     .fetch_optional(pool.get_ref())
//     .await
//     .map_err(|e| {
//         eprintln!("Database error: {:?}", e);
//         actix_web::error::ErrorInternalServerError(e)
//     })?;

//     sqlx::query(
//         r#"
//         DELETE FROM payment_users WHERE user_id = $1
//         "#
//     )
//     .bind(user_id)
//     .fetch_optional(pool.get_ref())
//     .await
//     .map_err(|e| {
//         eprintln!("Database error: {:?}", e);
//         actix_web::error::ErrorInternalServerError(e)
//     })?;

//     sqlx::query(
//         r#"
//         DELETE FROM address_users WHERE user_id = $1
//         "#
//     )
//     .bind(user_id)
//     .fetch_optional(pool.get_ref())
//     .await
//     .map_err(|e| {
//         eprintln!("Database error: {:?}", e);
//         actix_web::error::ErrorInternalServerError(e)
//     })?;

//     sqlx::query(
//         r#"
//         DELETE FROM order_users WHERE user_id = $1
//         "#
//     )
//     .bind(user_id)
//     .fetch_optional(pool.get_ref())
//     .await
//     .map_err(|e| {
//         eprintln!("Database error: {:?}", e);
//         actix_web::error::ErrorInternalServerError(e)
//     })?;
    
//     let user = sqlx::query_as::<_, User>(
//         r#"
//         DELETE FROM users WHERE id = $1
//         RETURNING names, emails, password_hashes, roles
//         "#,
//     )
//     .bind(user_id)
//     .fetch_optional(pool.get_ref())
//     .await
//     .map_err(|e| {
//         eprintln!("Database error: {:?}", e);
//         actix_web::error::ErrorInternalServerError(e)
//     })?;

//     match user {
//         Some(_) => {
//             let response = UserResponse {
//                 message: "Deleted successfully".to_string(),
//                 user: None,
//             };
//             Ok(HttpResponse::Ok().json(response))
//         }
//         None => Err(ErrorNotFound("User not found")),
//     }
// }