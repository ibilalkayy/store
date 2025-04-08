use std::error::Error;
use sqlx::{Row, postgres::PgPool};

pub async fn connect() -> Result<(), Box<dyn Error>> {
    let url = "postgres://postgres:1122@localhost:5432/ecommerce";
    let pool = PgPool::connect(url).await?;

    let res = sqlx::query("SELECT 1 + 1 as sum")
        .fetch_one(&pool)
        .await?;

    let sum: i32 = res.get("sum");
    println!("1 + 1 = {}", sum);

    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(())
}
