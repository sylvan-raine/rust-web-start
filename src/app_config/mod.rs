use crate::app_config::database::DatabaseConfig;
use crate::app_config::server::ServerConfig;
use config::Config;
use serde::Deserialize;
use std::sync::LazyLock;

mod database;
mod server;

static CONFIG: LazyLock<AppConfig> = LazyLock::new(|| AppConfig::load().unwrap());

#[derive(Deserialize)]
pub struct AppConfig {
    server: ServerConfig,     // server 配置字段
    database: DatabaseConfig, // database 配置字段
}

impl AppConfig {
    pub fn load() -> anyhow::Result<Self> {
        let configuration: AppConfig = Config::builder()
            .add_source(
                // 添加 app_config 的来源：web-start.toml
                config::File::with_name("web-start.toml")
                    .required(true)
                    .format(config::FileFormat::Toml),
            )
            .add_source(
                // 添加 app_config 的来源：环境变量
                config::Environment::with_prefix("WS")
                    .try_parsing(true)
                    .separator("_")
                    .list_separator(","),
            )
            .build() // 启动 I/O 读取配置文件（可能出错）
            .map_err(|e| {
                tracing::error!("读取配置时出错: {e}");
                anyhow::anyhow!("{e}")
            })? // 将错误映射为 anyhow 的错误
            .try_deserialize() // 解析配置文件（可能出错）
            .map_err(|e| {
                tracing::error!("无法反序列化配置文件: {e}");
                anyhow::anyhow!("无法反序列化配置文件.\n{e}")
            })?; // 将错误映射为 anyhow 的错误

        let server_config = &configuration.server;
        if !server_config.ipv6_enabled() ^ server_config.ipv4_enabled() {
            panic!("无法同时支持 IPv4 和 IPv6 监听.")
        }
        Ok(configuration)
    }
}

pub fn get_server() -> &'static ServerConfig {
    &CONFIG.server
}

pub fn get_database() -> &'static DatabaseConfig {
    &CONFIG.database
}
