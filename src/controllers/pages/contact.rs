//! # Contact
//! 
//! Contact page of the store application

use actix_web::{HttpResponse, Responder, get, web};
use tera::{Context, Tera};

/// Takes a parameter, sends a GET request to a URL, creates a context, renders an HTML file with it, and returns it in an HttpResponse.
#[get("/contact")]
pub async fn contact(tmpl: web::Data<Tera>) -> impl Responder {
    let ctx = Context::new();
    let s = tmpl.render("contact.html", &ctx).unwrap();
    HttpResponse::Ok().body(s)
}
