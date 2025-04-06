use crate::controllers::pages::{
    about_us::about_us, blogs::blogs, checkout::checkout, contact::contact,
    forgot_password::forgot_password, home::home, login::login, page_not_found::page_not_found,
    shop::shop, shopping_cart::shopping_cart, signup::signup,
};
use actix_files as fs;
use actix_web::web;

pub fn configure_service(cfg: &mut web::ServiceConfig) {
    cfg.service(fs::Files::new("/assets", "./src/assets/").show_files_listing())
        .service(home)
        .service(shop)
        .service(blogs)
        .service(contact)
        .service(about_us)
        .service(shopping_cart)
        .service(checkout)
        .service(signup)
        .service(login)
        .service(forgot_password)
        .default_service(web::route().to(page_not_found));
}
