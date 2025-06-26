use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    port: Option<u16>,
    log_level: Option<String>,
}

impl ServerConfig {
    pub fn port(&self) -> u16 {
        self.port.unwrap_or(8080)
    }
    
    pub fn log_level(&self) -> &str {
        self.log_level.as_deref().unwrap_or("info")
    }
}