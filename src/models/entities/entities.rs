use chrono::{DateTime, Utc};

pub struct User {
    id: u32,
    name: String,
    email: String,
    password: String,
    role: String
}

pub struct Product {
    id: u32,
    category_id: u32,
    name: String,
    price: f32,
    images: Vec<String>
}

pub struct Category {
    id: u32,
    name: String
}

pub struct Blog {
    id: u32,
    user_id: u32,
    title: String,
    publish_date: DateTime<Utc>,
    description: String
}

pub struct Contact {
    id: u32,
    name: String,
    email: String,
    message: String
}

pub struct Payment {
    id: u32,
    user_id: u32,
    order_id: u32,
    name: String,
    card_number: u32,
    cvv: u8,
    amount: f32,
    payment_status: String,
}

pub struct Address {
    id: u32,
    user_id: u32,
    first_name: String,
    last_name: String,
    address: String,
    town_city: String,
    country: String,
    zip_code: u32,
    phone_num: u32,
    email: String,
    password: String,
    notes: Option<String>,
}

pub struct Cart {
    id: u32,
    user_id: u32,
    product_id: Vec<u32>,
    products: Vec<String>,
    quantities: Vec<u32>,
    totals: Vec<u32>,
    grand_total: u128,
    discount_code: Option<String>,
}

pub struct Order {
    id: u32,
    user_id: u32,
    address_id: u32,
    products: Vec<String>,
    quantities: Vec<u32>,
    total: Vec<u32>,
    status: String,
    order_date: String,
}