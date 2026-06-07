use std::fmt::Debug;
use std::str::FromStr;

const APP_ENV_VAR: &str = "APP_ENV";
const HTTP_HOST_ENV_VAR: &str = "HTTP_HOST";
const HTTP_PORT_ENV_VAR: &str = "HTTP_PORT";

pub enum AppEnv {
    Development,
    Production,
}

impl FromStr for AppEnv {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "development" | "dev" => Ok(AppEnv::Development),
            "production" | "prod" => Ok(AppEnv::Production),
            _ => Err(format!("Invalid APP_ENV value: {s}")),
        }
    }
}

pub struct HttpServerConfig {
    pub host: String,
    pub port: u16,
}

pub struct AppConfig {
    app_env: AppEnv,
    http_server: HttpServerConfig,
}

impl AppConfig {
    pub fn init() -> Self {
        println!("AppConfig::init() -> .env file loaded");

        Self {
            app_env: read_env_var::<AppEnv>(APP_ENV_VAR, Some(AppEnv::Development)),
            http_server: HttpServerConfig {
                host: read_env_var::<String>(HTTP_HOST_ENV_VAR, Some("0.0.0.0".to_string())),
                port: read_env_var::<u16>(HTTP_PORT_ENV_VAR, None),
            },
        }
    }

    pub fn get_http_addr(&self) -> String {
        format!("{}:{}", self.http_server.host, self.http_server.port)
    }

    pub fn get_app_env(&self) -> &AppEnv {
        &self.app_env
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
