//! # Blogs
//! 
//! Blogs page of the store application

use actix_web::{HttpResponse, Responder, get, web};
use tera::{Context, Tera};

/// Takes a parameter, sends a GET request to a URL, creates a context, renders an HTML file with it, and returns it in an HttpResponse.
#[get("/blogs")]
pub async fn blogs(tmpl: web::Data<Tera>) -> impl Responder {
    let ctx = Context::new();
    let s = tmpl.render("blogs.html", &ctx).unwrap();
    HttpResponse::Ok().body(s)
}
