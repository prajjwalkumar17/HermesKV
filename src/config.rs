use crate::logger::LogConfig;

#[derive(Clone, Debug)]
pub struct GlobalConfig {
    pub server: Server,
    pub log: LogConfig,
    // pub database: Database,
    // pub log: Log,
}

#[derive(Clone, Debug)]
pub struct Server {
    pub host: String,
    pub port: u16,
    pub request_body_limit: usize,
}

impl Default for GlobalConfig {
    fn default() -> Self {
        Self {
            server: Server::default(),
            log: LogConfig::default(),
        }
    }
}

impl Default for Server {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 8080,
            request_body_limit: 32768,
        }
    }
}
