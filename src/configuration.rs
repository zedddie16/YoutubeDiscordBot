#[derive(serde::Deserialize)]
pub struct Config {
    pub token: String,
    pub youtube_key: String,
    pub channel: String,
}

pub fn get_configuration() -> Result<Config, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let settings = config::Config::builder()
        .add_source(config::File::from(base_path.join("config.yaml")))
        .build()?;
    settings.try_deserialize::<Config>()
}
