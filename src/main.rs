mod database;
mod models;
mod web;
use web::serve;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(10)
        .connect("postgres://localhost:5432/moto_auto?user=superadmin&password=superadmin")
        .await?;

    let _ = serve("127.0.0.1:8080").await?;

    Ok(())
}
