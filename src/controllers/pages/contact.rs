use actix_web::{HttpResponse, Responder, get, web};
use tera::{Context, Tera};

#[get("/contact")]
pub async fn contact(tmpl: web::Data<Tera>) -> impl Responder {
    let ctx = Context::new();
    let s = tmpl.render("contact.html", &ctx).unwrap();
    HttpResponse::Ok().body(s)
}
