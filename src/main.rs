mod logger;
mod app_config;
mod database;
mod entity;
mod server;
mod app;
mod route;
mod error;
mod extract;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    
    app::run(route::build_router()).await?;
    
    Ok(())
}