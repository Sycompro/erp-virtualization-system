use sqlx::PgPool;
use anyhow::Result;

pub async fn run_migrations_if_needed(pool: &PgPool) -> Result<()> {
    // Check if migrations table exists
    let table_exists: bool = sqlx::query_scalar(
        "SELECT EXISTS (
            SELECT FROM information_schema.tables 
            WHERE table_schema = 'public' 
            AND table_name = 'users'
        )"
    )
    .fetch_one(pool)
    .await?;

    if !table_exists {
        tracing::info!("Running database initialization...");
        
        // Run the initialization SQL directly (embedded in binary)
        let init_sql = include_str!("../../../database/init.sql");
        
        // Split and execute SQL statements
        for statement in init_sql.split(';') {
            let statement = statement.trim();
            if !statement.is_empty() && !statement.starts_with("--") {
                sqlx::query(statement).execute(pool).await.ok(); // Ignore errors for IF NOT EXISTS
            }
        }
        
        tracing::info!("Database initialization completed");
    } else {
        tracing::info!("Database already initialized, skipping migrations");
    }

    Ok(())
}