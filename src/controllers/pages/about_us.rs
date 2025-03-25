use actix_web::{HttpResponse, Responder, get, web};
use tera::{Context, Tera};

#[get("/about_us")]
pub async fn about_us(tmpl: web::Data<Tera>) -> impl Responder {
    let ctx = Context::new();
    let s = tmpl.render("about_us.html", &ctx).unwrap();
    HttpResponse::Ok().body(s)
}
