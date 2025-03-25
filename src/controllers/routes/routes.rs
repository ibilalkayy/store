use crate::controllers::pages::{
    about_us::about_us, blogs::blogs, checkout::checkout, contact::contact, home::home, info::info,
    login::login, shop::shop, shopping_cart::shopping_cart, signup::signup,
};
use actix_web::web;

pub fn configure_service(cfg: &mut web::ServiceConfig) {
    cfg.service(home)
        .service(shop)
        .service(info)
        .service(blogs)
        .service(contact)
        .service(about_us)
        .service(shopping_cart)
        .service(checkout)
        .service(signup)
        .service(login);
}
