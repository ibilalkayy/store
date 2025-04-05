use actix_web::{HttpResponse, Responder, web};
use tera::{Context, Tera};

/// Takes a parameter, creates a context, renders an HTML file with it, and returns it in an HttpResponse.
pub async fn page_not_found(tmpl: web::Data<Tera>) -> impl Responder {
    let ctx = Context::new();
    let s = tmpl.render("page_not_found.html", &ctx).unwrap();
    HttpResponse::Ok().body(s)
}
