CREATE TABLE IF NOT EXISTS product_categories (
  product_id INT REFERENCES products(id) ON DELETE CASCADE,
  category_id INT REFERENCES categories(id) ON DELETE CASCADE,
  PRIMARY KEY (product_id, category_id)
);

CREATE TABLE IF NOT EXISTS blog_users (
  blog_id INT REFERENCES blogs(id) ON DELETE CASCADE,
  user_id INT REFERENCES users(id) ON DELETE CASCADE,
  PRIMARY KEY (blog_id, user_id)
);

CREATE TABLE IF NOT EXISTS payment_orders (
  payment_id INT REFERENCES payments(id) ON DELETE CASCADE,
  order_id INT REFERENCES orders(id) ON DELETE CASCADE,
  PRIMARY KEY (payment_id, order_id)
);

CREATE TABLE IF NOT EXISTS payment_users (
  payment_id INT REFERENCES payments(id) ON DELETE CASCADE,
  user_id INT REFERENCES users(id) ON DELETE CASCADE,
  PRIMARY KEY (payment_id, user_id)
);

CREATE TABLE IF NOT EXISTS address_users (
  address_id INT REFERENCES addresses(id) ON DELETE CASCADE,
  user_id INT REFERENCES users(id) ON DELETE CASCADE,
  PRIMARY KEY (address_id, user_id)
);

CREATE TABLE IF NOT EXISTS order_users (
  order_id INT REFERENCES orders(id) ON DELETE CASCADE,
  user_id INT REFERENCES users(id) ON DELETE CASCADE,
  PRIMARY KEY (order_id, user_id)
);

CREATE TABLE IF NOT EXISTS order_addresses (
  order_id INT REFERENCES orders(id) ON DELETE CASCADE,
  address_id INT REFERENCES addresses(id) ON DELETE CASCADE,
  PRIMARY KEY (order_id, address_id)
)