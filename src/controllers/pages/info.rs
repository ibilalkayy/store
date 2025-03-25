use actix_web::{HttpResponse, Responder, get, web};
use tera::{Context, Tera};

#[get("/info")]
pub async fn info(tmpl: web::Data<Tera>) -> impl Responder {
    let ctx = Context::new();
    let s = tmpl.render("info.html", &ctx).unwrap();
    HttpResponse::Ok().body(s)
}
