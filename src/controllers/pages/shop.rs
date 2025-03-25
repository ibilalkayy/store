use actix_web::{HttpResponse, Responder, get, web};
use tera::{Context, Tera};

#[get("/shop")]
pub async fn shop(tmpl: web::Data<Tera>) -> impl Responder {
    let ctx = Context::new();
    let s = tmpl.render("shop.html", &ctx).unwrap();
    HttpResponse::Ok().body(s)
}
