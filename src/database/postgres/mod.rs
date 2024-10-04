use sqlx::{Postgres, Pool, postgres::PgPoolOptions};

pub async fn start_con() -> Pool<Postgres> {
    let postg_env = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&postg_env)
        .await
        .expect("Failed to connect to the database");

    sqlx::migrate!("./src/database/migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    println!("Migrations ran successfully");

    pool
}
