mod app;
mod app_config;
mod database;
mod entity;
mod error;
mod logger;
mod route;
mod server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    app::run(route::build_router()).await?;
    Ok(())
}
