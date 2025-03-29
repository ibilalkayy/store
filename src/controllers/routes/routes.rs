use crate::controllers::pages::{
    about_us::about_us, blogs::blogs, checkout::checkout, contact::contact, faqs::faqs, home::home,
    info::info, login::login, page_not_found::page_not_found, shop::shop,
    shopping_cart::shopping_cart, signup::signup,
};
use actix_web::web;
use actix_files as fs;

pub fn configure_service(cfg: &mut web::ServiceConfig) {
    cfg
    .service(fs::Files::new("/assets", "./src/assets/").show_files_listing())
        .service(home)
        .service(shop)
        .service(info)
        .service(blogs)
        .service(contact)
        .service(about_us)
        .service(shopping_cart)
        .service(checkout)
        .service(signup)
        .service(login)
        .service(faqs)
        .default_service(web::route().to(page_not_found));
}
