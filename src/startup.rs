use crate::base::{check_for_new_video, fetch, use_config};
use config::{Config, ConfigError};
use env_logger::{Builder, Target};
use lazy_static::lazy_static;
use log::{error, info};
use serde::Deserialize;
use serde_yaml;
use serenity::all::{
    ChannelId, Context, EventHandler, GatewayIntents, Message, MessageBuilder, Ready,
};
use serenity::{async_trait, Client};
use std::fs;
use std::fs::File;
use std::io::Read;

pub struct Handler;

#[derive(Deserialize)]
pub struct YoutubeDiscordBotSettings {
    pub api: String,
    pub channel: String,
}
lazy_static! {
    pub static ref CONFIG: YoutubeDiscordBotSettings = {
        let config_data = fs::read_to_string("config.yaml").expect("Failed to read config.yaml");
        serde_yaml::from_str(&config_data).expect("Failed to parse config data");
        todo!("Dodelai")
    };
}

#[async_trait]
impl EventHandler for Handler {
    //handling messages received by bot
    async fn message(&self, context: Context, msg: Message) {
        info!("message received");
        match msg.content.as_str() {
            //sets current channel as a target channel
            "/set clips" => {
                info!("/set clips message received");

                let target_channel = msg.channel_id.to_string();

                //response to /set clips
                let mut response: String = MessageBuilder::new()
                    .push("setting up clips-channel...")
                    .build();
                //saying massage
                msg.channel_id
                    .say(&context.http, &response)
                    .await
                    .expect("Message sending failed");
                match fs::write("target_channel.txt", target_channel) {
                    Ok(()) => {
                        response = MessageBuilder::new()
                            .push("target-channel set up successful")
                            .build();
                    }
                    Err(..) => {
                        let _response = MessageBuilder::new()
                            .push_bold("target-channel set up failed")
                            .build();
                    }
                }
                msg.channel_id
                    .say(&context.http, &response)
                    .await
                    .expect("Message sending failed");
            }
            //unimplemented feature (does nothing for now)
            "/set channel" => {
                info!("new /set channel request");
                let channel = msg.content;
                fs::write("channel.txt", channel).expect("failed writing channel.txt");
            }
            //return last video as answer in same channel
            "!bot" => {
                info!("!bot message received");
                let channel = match msg.channel_id.to_channel(&context).await {
                    Ok(channel) => channel,
                    Err(why) => {
                        error!("Error getting channel: {why:?}");
                        return;
                    }
                };

                let video_id = match fetch::fetch_latest_video_id().await {
                    Ok(id) => id,
                    Err(err) => {
                        error!("Error fetching video ID: {}", err);
                        return;
                    }
                };

                let response = MessageBuilder::new()
                    .push("User ")
                    .push_bold_safe(&msg.author.name)
                    .push(" latest video: https://www.youtube.com/watch?v=")
                    .push(video_id)
                    .mention(&channel)
                    .push(" channel")
                    .build();
                info!("!bot command responded");
                if let Err(why) = msg.channel_id.say(&context.http, &response).await {
                    error!("Error sending message: {why:?}");
                }
            }
            _ => {}
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        Builder::from_default_env().target(Target::Stderr).init();
        info!("{} is connected!", ready.user.name);

        let mut holder = String::new();
        match File::open("target_channel.txt") {
            Ok(mut file) => {
                file.read_to_string(&mut holder).unwrap();
            }
            Err(_err) => {
                error!("File target_channel.txt not found, creating a new one");
                fs::write("target_channel.txt", "").unwrap();
            }
        }
        let channel_id = ChannelId::new(holder.parse().unwrap());

        let _message_content = "test message";

        check_for_new_video::check_for_new_video(ctx, channel_id)
            .await
            .expect("failed to start check_for_new_video");
    }
}
//run the bot
pub async fn run() -> Result<(), ConfigError> {
    //takes token from Config
    let token = use_config::use_config()?.get::<String>("token")?;

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
    Ok(())
}
