use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    host: Option<String>,
    port: Option<u16>,
    user: Option<String>,
    passwd: Option<String>,
    database: Option<String>,
    schema: Option<String>,
}

impl DatabaseConfig {
    pub fn host(&self) -> &str {
        self.host.as_deref().unwrap_or("localhost")
    }
    
    pub fn port(&self) -> u16 {
        self.port.unwrap_or(5432)
    }
    
    pub fn user(&self) -> &str {
        self.user.as_deref().unwrap_or("postgres")
    }
    
    pub fn password(&self) -> &str {
        self.passwd.as_deref().expect("passwd unknown")
    }
    
    pub fn database(&self) -> &str {
        self.database.as_deref().unwrap_or("postgres")
    }
    
    pub fn schema(&self) -> &str {
        self.schema.as_deref().unwrap_or("public")
    }
}