mod logger;
mod app_config;
mod database;
mod entity;
mod server;
mod app;
mod query;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    
    app::run(query::build_router()).await?;
    
    Ok(())
}