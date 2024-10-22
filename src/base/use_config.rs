use config::{Config, ConfigError};

use crate::startup::YouTubeDiscordBotSettings;

pub(crate) fn use_config() -> Result<YouTubeDiscordBotSettings, ConfigError> {
    let config = crate::startup::CONFIG.as_ref().map(|c| c.clone()).unwrap();
    config.try_deserialize::<YouTubeDiscordBotSettings>()
}
