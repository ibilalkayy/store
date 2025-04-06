pub struct users {
    id: u64,
    name: String,
    email: String,
    password: String,
    role: String
}

pub struct products {
    id: u64,
    name: String,
    price: u64,
    category: String,
    images: String
}

pub struct blogs {
    id: u64,
    title: String,
    publish_date: String,
    description: String,
}

pub struct contacts {
    id: u64,
    name: String,
    email: String,
    message: String
}

pub struct payments {
    id: u64,
    order_id: u8,
    name: String,
    card_number: u64,
    pin: u8,
    amount: u64,
    payment_status: String,
}

pub struct addresses {
    first_name: String,
    last_name: String,
    address: String,
    town_city: String,
    country: String,
    zip_code: u16,
    phone_num: u16,
    email: String,
    password: String,
    notes: Option<String>,
}

pub struct cart {
    products: Vec<String>,
    quantities: Vec<u64>,
    totals: Vec<u64>,
    grand_total: u128,
    discount_code: Option<String>,
}