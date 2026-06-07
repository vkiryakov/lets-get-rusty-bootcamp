use std::fmt::Debug;
use std::str::FromStr;

const HTTP_HOST: &str = "HTTP_HOST";
const HTTP_PORT_ENV_VAR: &str = "HTTP_PORT";

pub struct HttpServerConfig {
    pub host: String,
    pub port: u16,
}

pub struct AppConfig {
    http_server: HttpServerConfig,
}

impl AppConfig {
    pub fn init() -> Self {
        dotenvy::dotenv().ok();
        println!("AppConfig::init() -> .env file loaded");

        Self {
            http_server: HttpServerConfig {
                host: read_env_var::<String>(HTTP_HOST, Some("0.0.0.0".to_string())),
                port: read_env_var::<u16>(HTTP_PORT_ENV_VAR, None),
            },
        }
    }

    pub fn get_http_addr(&self) -> String {
        format!("{}:{}", self.http_server.host, self.http_server.port)
    }
}

fn read_env_var<T>(var_name: &str, default_value: Option<T>) -> T
where
    T: FromStr,
    T::Err: Debug,
{
    match std::env::var(var_name) {
        Ok(value) => value
            .parse::<T>()
            .unwrap_or_else(|e| panic!("Configuration error! Failed to parse {var_name}: {e:?}")),
        Err(_) => default_value
            .unwrap_or_else(|| panic!("Configuration error! {var_name} is not defined")),
    }
}
