mod logger;
mod app_config;
mod database;
mod entity;
mod server;
mod app;
mod route;
mod error;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    app::run(route::build_router()).await?;
    Ok(())
}