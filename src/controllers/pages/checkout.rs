use actix_web::{HttpResponse, Responder, get, web};
use tera::{Context, Tera};

#[get("/checkout")]
pub async fn checkout(tmpl: web::Data<Tera>) -> impl Responder {
    let ctx = Context::new();
    let s = tmpl.render("checkout.html", &ctx).unwrap();
    HttpResponse::Ok().body(s)
}
