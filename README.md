## YoutubeDiscordBot
A Rust-based Discord bot that automatically posts new YouTube videos to a specified Discord channel.

This project serves as a learning experience for Rust programming. The bot monitors a selected YouTube channel and sends a message to a designated Discord channel whenever a new video is uploaded.

### Features

Channel Selection: Configure the YouTube channel and Discord channel to monitor and post to.
New Video Notification: Automatically posts a message with the new video's details to the Discord channel.
## Usage
### Configure config.toml:

* ```DISCORD_API_KEY```: Your **Discord bot** token.

* ```YOUTUBE_API_KEY```: Your **YouTube Data API key**.

* ```YOUTUBE_CHANNEL```: The ID of the **YouTube channel** to monitor.

**Run the bot: Start the bot application.**

### Discord Commands:

```/set clips```: Sets the **current channel** as the "clips" channel (for future feature implementation).
```/!bot```: Checks for the latest video from the configured **YouTube channel** and posts it to the **chosen Discord channel**.
Future Improvements
I plan to expand the bot's functionality by:

Supporting multiple YouTube channels.
Implementing additional Discord commands for customization.
Enhancing the notification message format.
Exploring error handling and logging.
***Note: Currently, the bot only supports a single YouTube channel. Multi-channel support is in development.***