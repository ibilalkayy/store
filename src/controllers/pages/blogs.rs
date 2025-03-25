use actix_web::{HttpResponse, Responder, get, web};
use tera::{Context, Tera};

#[get("/blogs")]
pub async fn blogs(tmpl: web::Data<Tera>) -> impl Responder {
    let ctx = Context::new();
    let s = tmpl.render("blogs.html", &ctx).unwrap();
    HttpResponse::Ok().body(s)
}
