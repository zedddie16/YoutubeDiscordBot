use config::{Config, ConfigError};

use crate::startup::YoutubeDiscordBotSettings;

pub(crate) fn use_config() -> Result<YoutubeDiscordBotSettings, ConfigError> {
    let config = crate::startup::CONFIG.map(|c| c.clone()).unwrap();
    config.try_deserialize::<YoutubeDiscordBotSettings>()
}
