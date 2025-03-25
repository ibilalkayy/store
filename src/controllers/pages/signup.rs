use actix_web::{HttpResponse, Responder, get, web};
use tera::{Context, Tera};

#[get("/signup")]
pub async fn signup(tmpl: web::Data<Tera>) -> impl Responder {
    let ctx = Context::new();
    let s = tmpl.render("signup.html", &ctx).unwrap();
    HttpResponse::Ok().body(s)
}
