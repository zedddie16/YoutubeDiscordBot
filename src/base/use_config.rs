use config::{Config, ConfigError};

pub(crate) fn use_config() -> Result<Config, ConfigError> {
    let config = crate::startup::CONFIG.as_ref().map(|c| c.clone()).unwrap();
    Ok(config)
}
