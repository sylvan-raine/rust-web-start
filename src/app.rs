use crate::server::ServerState;
use crate::{database, logger, server};
use axum::Router;

/// 初始化 [logger]
///
/// 和数据库连接 [database::init]
///
/// 并使用传进来的 [Router] 启动服务器
pub async fn run(router: Router<ServerState>) -> anyhow::Result<()> {
    logger::init();
    tracing::info!("正在启动服务器...");
    let db = database::init().await?;

    let state = ServerState::new(db);
    server::start(router, state).await
}
