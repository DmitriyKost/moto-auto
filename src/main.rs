mod database;
mod models;
mod web;
mod cache;
use sqlx::PgPool;
use web::serve;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool: PgPool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(10)
        .connect("postgres://localhost:5432/moto_auto?user=superadmin&password=superadmin")
        .await?;

    let _ = serve(pool, "127.0.0.1:8080").await;

    Ok(())
}
