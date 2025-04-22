// API paths
use crate::controllers::apis::{
    blogs::{create_blog, get_blog, update_blog, delete_blog}, 
    categories::{create_category, get_categories, update_category, delete_category},
    contact::{create_contact, get_contact, delete_contact},
    product::{create_product, delete_product, get_product, update_product},
    user::{create_user, get_user, update_user, delete_user},
    payment::{create_payment, get_payment, update_payment, delete_payment},
    address::{create_address, get_address, update_address, delete_address},
    order::{create_order, get_order, update_order, delete_order}
};

// API relationships paths
use crate::controllers::relations::{
    user::{get_blogs_by_user, get_payments_by_user, get_addresses_by_user, get_orders_by_user},
    blogs::{assign_user_to_blog, get_user_by_blog},
    categories::get_products_by_category,
    product::{assign_category_to_product, get_categories_by_product},
    payment::{assign_payment_to_order, get_order_by_payment, assign_payment_to_user, get_user_by_payment},
    address::{assign_user_to_address, get_user_by_address, get_orders_by_address},
    order::{get_payments_by_order, assign_user_to_order, get_user_by_order, assign_address_to_order, get_address_by_order}
};

use actix_files as fs;
use actix_web::web;

// Web pages paths
use crate::controllers::pages::{
    home::home,
    shop::shop,
    blogs::blogs,
    contact::contact,
    about_us::about_us,
    shopping_cart::shopping_cart,
    checkout::checkout,
    signup::signup,
    login::login,
    forgot_password::forgot_password,
    page_not_found::page_not_found
};

pub fn page_services(cfg: &mut web::ServiceConfig) {
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

pub fn user_services(cfg: &mut web::ServiceConfig) {
    cfg.service(create_user)
        .service(get_user)
        .service(update_user)
        .service(delete_user)
        .service(get_blogs_by_user)
        .service(get_payments_by_user)
        .service(get_addresses_by_user)
        .service(get_orders_by_user);
}

pub fn product_services(cfg: &mut web::ServiceConfig) {
    cfg.service(create_product)
        .service(get_product)
        .service(update_product)
        .service(delete_product)
        .service(assign_category_to_product)
        .service(get_categories_by_product);
}

pub fn category_services(cfg: &mut web::ServiceConfig) {
    cfg.service(create_category)
        .service(get_categories)
        .service(get_products_by_category)
        .service(update_category)
        .service(delete_category);
}

pub fn contact_services(cfg: &mut web::ServiceConfig) {
    cfg.service(create_contact)
        .service(get_contact)
        .service(delete_contact);
}

pub fn blog_services(cfg: &mut web::ServiceConfig) {
    cfg.service(create_blog)
        .service(get_blog)
        .service(update_blog)
        .service(delete_blog)
        .service(assign_user_to_blog)
        .service(get_user_by_blog);
}

pub fn payment_services(cfg: &mut web::ServiceConfig) {
    cfg.service(create_payment)
        .service(get_payment)
        .service(update_payment)
        .service(delete_payment)
        .service(assign_payment_to_order)
        .service(get_order_by_payment)
        .service(assign_payment_to_user)
        .service(get_user_by_payment);
}

pub fn address_services(cfg: &mut web::ServiceConfig) {
    cfg.service(create_address)
        .service(get_address)
        .service(update_address)
        .service(delete_address)
        .service(assign_user_to_address)
        .service(get_user_by_address)
        .service(get_orders_by_address);
}

pub fn order_services(cfg: &mut web::ServiceConfig) {
    cfg.service(create_order)
        .service(get_order)
        .service(update_order)
        .service(delete_order)
        .service(get_payments_by_order)
        .service(assign_user_to_order)
        .service(get_user_by_order)
        .service(assign_address_to_order)
        .service(get_address_by_order);
}