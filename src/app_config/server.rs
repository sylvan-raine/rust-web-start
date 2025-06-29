use serde::Deserialize;

#[derive(Deserialize)]
pub struct ServerConfig {
    port: Option<u16>,
    log_level: Option<String>,
    ipv4_enabled: Option<bool>,
    ipv6_enabled: Option<bool>,
}

impl ServerConfig {
    pub fn port(&self) -> u16 {
        self.port.unwrap_or(8080)
    }
    
    pub fn log_level(&self) -> &str {
        self.log_level.as_deref().unwrap_or("info")
    }
    
    pub fn ipv4_enabled(&self) -> bool {
        self.ipv4_enabled.unwrap_or(true)
    }
    
    pub fn ipv6_enabled(&self) -> bool {
        self.ipv6_enabled.unwrap_or(false)
    }
}