use sqlx::{Postgres, Pool, postgres::PgPoolOptions};

pub async fn start_con() -> Pool<Postgres>{
    let postg_env = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(&postg_env)
    .await
    .expect("Fail to conect");

    let migrations = sqlx::migrate!("./src/database/migrations")
    .run(&pool)
    .await;

    match migrations {
        Ok(_) => println!("Migrations ran successfully"),
        Err(e) => println!("Error running migrations: {:?}",e),
    }

    pool
}