use anyhow::Result;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use sqlx::SqlitePool;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};

async fn connect(db_url: &str) -> Result<SqlitePool> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(pool)
}

pub async fn setup_db() -> Result<Pool<Sqlite>> {
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://sample.db".into());
    let should_seed_database: bool = std::env::var("SEED_DATABASE").unwrap().parse().unwrap();
    let db = connect(&db_url).await?;
    if should_seed_database {
        seed_admin(&db).await?;
        seed_user(&db).await?;
    }
    Ok(db)
}

async fn seed_admin(db: &SqlitePool) -> anyhow::Result<()> {
    let email = std::env::var("ADMIN_EMAIL").unwrap_or_else(|_| "admin@example.com".into());
    let password = std::env::var("ADMIN_PASSWORD").unwrap_or_else(|_| "admin123".into());

    let exists = sqlx::query_scalar!("SELECT COUNT(1) as count FROM users WHERE email = ?", email)
        .fetch_one(db)
        .await?;

    if exists == 0 {
        let salt = SaltString::generate(&mut OsRng);

        let hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .unwrap()
            .to_string();

        sqlx::query!(
            "INSERT INTO users (email, password_hash) VALUES (?, ?)",
            email,
            hash
        )
        .execute(db)
        .await?;

        tracing::info!("seeded admin user");
    }
    Ok(())
}

async fn seed_user(db: &SqlitePool) -> anyhow::Result<()> {
    let email = "user@example.com".to_string();
    let password = "user123".to_string();

    let exists = sqlx::query_scalar!("SELECT COUNT(1) as count FROM users WHERE email = ?", email)
        .fetch_one(db)
        .await?;

    if exists == 0 {
        let salt = SaltString::generate(&mut OsRng);

        let hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .unwrap()
            .to_string();

        sqlx::query!(
            "INSERT INTO users (email, password_hash) VALUES (?, ?)",
            email,
            hash
        )
        .execute(db)
        .await?;

        tracing::info!("seeded user");
    }
    Ok(())
}
