use actix_web::{HttpResponse, Responder, get, web};
use tera::{Context, Tera};

#[get("/shopping_cart")]
pub async fn shopping_cart(tmpl: web::Data<Tera>) -> impl Responder {
    let ctx = Context::new();
    let s = tmpl.render("shopping_cart.html", &ctx).unwrap();
    HttpResponse::Ok().body(s)
}
