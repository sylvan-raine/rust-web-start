use std::fmt::Debug;
use std::sync::LazyLock;
use config::Config;
use serde::Deserialize;
use crate::app_config::database::DatabaseConfig;
use crate::app_config::server::ServerConfig;

mod server;
mod database;

static CONFIG: LazyLock<AppConfig> = LazyLock::new(|| AppConfig::load().unwrap());

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    server: ServerConfig,       // server 配置字段
    database: DatabaseConfig,   // database 配置字段
}

impl AppConfig {
    pub fn load() -> anyhow::Result<Self> {
        Config::builder()
            .add_source(    // 添加 app_config 的来源：web-start.toml
                config::File::with_name("web-start.toml")
                    .required(true)
                    .format(config::FileFormat::Toml)
            )
            .add_source(    // 添加 app_config 的来源：环境变量
                config::Environment::with_prefix("WS")
                    .try_parsing(true)
                    .separator("_")
                    .list_separator(",")
            )
            .build()    // 启动 I/O 读取配置文件（可能出错） 
            .map_err(|e| {
                tracing::error!("Error initializing config: {}", e);
                anyhow::anyhow!("{e}")
            })? // 将错误映射为 anyhow 的错误
            .try_deserialize()  // 解析配置文件（可能出错）
            .map_err(|e| {
                tracing::error!("Error deserializing config: {}", e);
                anyhow::anyhow!("Failed to deserialize app_config.\n{e}")
            })   // 将错误映射为 anyhow 的错误
    }
}

pub fn get_server() -> &'static ServerConfig {
    &CONFIG.server
}

pub fn get_database() -> &'static DatabaseConfig {
    &CONFIG.database
}