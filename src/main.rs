mod database;
mod models;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(10)
        .connect("postgres://localhost:5432/moto_auto?user=superadmin&password=superadmin")
        .await?;
    Ok(())
}
