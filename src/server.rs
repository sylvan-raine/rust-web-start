use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr};
use std::time::Duration;
use axum::extract::{DefaultBodyLimit, Request};
use axum::Router;
use base64::prelude::BASE64_STANDARD_NO_PAD;
use base64::Engine;
use sea_orm::DatabaseConnection;
use tokio::net::TcpListener;
use tower_http::cors;
use tower_http::cors::CorsLayer;
use tower_http::normalize_path::NormalizePathLayer;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::{DefaultOnResponse, TraceLayer};
use crate::app_config;

/// 将传进来的 [Router] 和 [ServerState] 绑定，并开始在指定的端口运行服务器
pub async fn start(router: Router<ServerState>, state: ServerState) -> anyhow::Result<()> {
    let port = app_config::get_server().port();
    let service = build_router(router, state).into_make_service_with_connect_info::<SocketAddr>();
    
    
    if app_config::get_server().ipv6_enabled() {
        let listener_v6 = TcpListener::bind((Ipv6Addr::UNSPECIFIED, port)).await?;
        tracing::info!("listening on {}.", listener_v6.local_addr()?);
        axum::serve(listener_v6, service).await?;
    } else {
        let listener_v4 = TcpListener::bind((Ipv4Addr::UNSPECIFIED, port)).await?;
        tracing::info!("listening on {}.", listener_v4.local_addr()?);
        axum::serve(listener_v4, service).await?;
    }
    
    Ok(())
}

/// 创建一个 [Router]，将 [ServerState] 嵌入到 [Router] 中，使得所有的请求处理函数能够共享这个资源
fn build_router(router: Router<ServerState>, state: ServerState) -> Router {
    let tracing_layer = TraceLayer::new_for_http()
        .make_span_with(|req: &Request| {
            let method = req.method().to_string();
            let uri = req.uri().to_string();
            let id = BASE64_STANDARD_NO_PAD.encode(uuid::Uuid::new_v4());   // 使用 base64 编码的 uuid 作为请求 id
            tracing::info_span!("http request", id, uri, method)
        })
        .on_failure(())
        .on_request(())
        .on_response(DefaultOnResponse::new().level(tracing::Level::INFO));
    let timeout_layer = TimeoutLayer::new(Duration::from_secs(120));
    let body_limit_layer = DefaultBodyLimit::max((1024 * 1024 * 16) as usize);    // 16 MB 的最大报文大小
    let cors_layer = CorsLayer::new()
        .allow_methods(cors::Any)
        .allow_headers(cors::Any)
        .allow_origin(cors::Any)
        .allow_credentials(false)
        .max_age(Duration::from_secs(3600 * 24));
    let path_normalize_layer = NormalizePathLayer::trim_trailing_slash();

    router
        .layer(path_normalize_layer)
        .layer(timeout_layer)
        .layer(body_limit_layer)
        .layer(tracing_layer)
        .layer(cors_layer)
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