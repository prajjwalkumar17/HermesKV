pub struct GlobalConfig {
    pub server: Server,
    // pub database: Database,
    // pub log: Log,
}

pub struct Server {
    pub host: String,
    pub port: u16,
}

