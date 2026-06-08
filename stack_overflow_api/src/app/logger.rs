use crate::app::config;


pub fn init_logger(app_config: &config::AppConfig) {
    let env_filter =
        tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "debug".into());

    let builder = tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_target(true)
        .with_file(true)
        .with_line_number(true);

    match app_config.get_app_env() {
        config::AppEnv::Development => builder.pretty().init(),
        config::AppEnv::Production => builder.json().init(),
    };
}
