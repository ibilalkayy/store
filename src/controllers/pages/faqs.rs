use actix_web::{HttpResponse, Responder, get, web};
use tera::{Context, Tera};

#[get("/faqs")]
pub async fn faqs(tmpl: web::Data<Tera>) -> impl Responder {
    let ctx = Context::new();
    let s = tmpl.render("faqs.html", &ctx).unwrap();
    HttpResponse::Ok().body(s)
}
