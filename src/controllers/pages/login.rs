use actix_web::{HttpResponse, Responder, get, web};
use tera::{Context, Tera};

#[get("/login")]
pub async fn login(tmpl: web::Data<Tera>) -> impl Responder {
    let ctx = Context::new();
    let s = tmpl.render("login.html", &ctx).unwrap();
    HttpResponse::Ok().body(s)
}
