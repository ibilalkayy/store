use actix_web::{HttpResponse, Responder, get, web};
use tera::{Context, Tera};

#[get("/")]
pub async fn home(tmpl: web::Data<Tera>) -> impl Responder {
    let ctx = Context::new();
    let s = tmpl.render("home.html", &ctx).unwrap();
    HttpResponse::Ok().body(s)
}
