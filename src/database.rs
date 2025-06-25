use std::cmp::max;
use std::time::Duration;
use crate::app_config;
use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection, DbBackend, Statement};
use tracing::log;

pub async fn init() -> anyhow::Result<DatabaseConnection> {
    let db_config = app_config::get_database();
    let mut options = ConnectOptions::new(
        format!(
            "postgres://{}:{}@{}:{}/{}",
            db_config.user(),
            db_config.password(),
            db_config.host(),
            db_config.port(),
            db_config.database(),
        )
    );
    
    options.min_connections(max((num_cpus::get() * 4) as u32, 10))
        .max_connections(max((num_cpus::get() * 8) as u32, 10))
        .connect_timeout(Duration::from_secs(5))
        .acquire_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_secs(5))
        .max_lifetime(Duration::from_secs(300))
        .sqlx_logging(false)
        .set_schema_search_path(db_config.schema());
    
    let conn = Database::connect(options).await.map_err(|e| {
        tracing::error!("Error connecting to database.");
        anyhow::anyhow!("{e}")
    })?;
    tracing::info!("Database connection established.");
    log_database_version(&conn).await?;
    
    Ok(conn)
}

async fn log_database_version(db: &DatabaseConnection) -> anyhow::Result<()> {
    let version = db.query_one(
        Statement::from_string(
            DbBackend::Postgres,
            String::from("SELECT version()"),
        )
    )
        .await?
        .ok_or_else(|| anyhow::anyhow!("Failed to get version failed."))?;
    
    log::info!("Database version: {}", version.try_get_by_index::<String>(0)?);
    Ok(())
}