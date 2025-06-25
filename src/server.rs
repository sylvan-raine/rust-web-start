use std::net::SocketAddr;
use axum::Router;
use sea_orm::DatabaseConnection;
use tokio::net::TcpListener;
use crate::app_config;

/// 将传进来的 [Router] 和 [ServerState] 绑定，并开始在指定的端口运行服务器
pub async fn start(router: Router<ServerState>, state: ServerState) -> anyhow::Result<()> {
    let port = app_config::get_server().port();
    let listener = TcpListener::bind(("0.0.0.0", port)).await?;
    tracing::info!("listening on {}.", listener.local_addr()?);

    axum::serve(
        listener,
        build_router(router, state).into_make_service_with_connect_info::<SocketAddr>()
    ).await?;
    Ok(())
}

/// 创建一个 [Router]，将 [ServerState] 嵌入到 [Router] 中，使得所有的请求处理函数能够共享这个资源
fn build_router(router: Router<ServerState>, state: ServerState) -> Router {
    Router::new()
        .merge(router)
        .with_state(state)
}

/// 保存了服务器运行状态，我叫他上下文，不知道对不对，包括
/// 
/// - 数据库连接池
/// 
/// `router` 的分支可以通过这个添加参数获取数据库连接等信息
#[derive(Clone)]
pub struct ServerState {
    db: DatabaseConnection
}

impl ServerState {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
    
    pub fn db(&self) -> &DatabaseConnection {
        &self.db
    }
}