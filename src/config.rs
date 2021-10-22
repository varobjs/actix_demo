use envconfig::Envconfig;

#[derive(Debug, Clone)]
pub struct Config {
    pub api: ApiConfig,
    pub database: DbConfig,
}

#[derive(Envconfig, Debug, Clone)]
pub struct ApiConfig {
    #[envconfig(from = "API_HOST", default = "127.0.0.1")]
    pub host: String,
    #[envconfig(from = "API_PORT", default = "7788")]
    pub port: u32,
}

#[derive(Envconfig, Debug, Clone)]
pub struct DbConfig {
    #[envconfig(from = "DATABASE_URL")]
    pub url: String,

    #[envconfig(from = "POOL_SIZE", default = "5")]
    pub pool_size: u32,
}


impl Config {
    pub fn new() -> Config {
        Config {
            api: ApiConfig::init_from_env().unwrap(),
            database: DbConfig::init_from_env().unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        dotenv::dotenv().ok();
        let config = Config::new();
        println!("{:?}", config);
        println!("Hello world, {}", config.api.host);
        assert_eq!("127.0.0.1", config.api.host);
    }
}