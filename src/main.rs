
use std::error::Error;
use std::hash::Hash;
use serde;
use serde_json::{Value, from_str};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::guild::Member;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, Read, stdin};
use std::{thread};
use std::sync::mpsc::channel;
use std::thread::sleep;
use log::{error, info, Level, LevelFilter, trace};
use env_logger;
use env_logger::{Builder, Target};
use serenity::all::ChannelId;
use time::Duration;
use config;
use config::{Config, ConfigBuilder, ConfigError};
use lazy_static::lazy_static;

lazy_static! {
    static ref CONFIG: Result<Config, ConfigError> = {
            let mut builder: Config = Config::builder()
                .add_source(config::File::with_name("C:/Program Files/ytdcbot/data/config.toml"))
                .build()
                .unwrap();
            Ok(builder)
    };
}
fn use_config() -> Result<Config, ConfigError>{
    let config = CONFIG.as_ref().map(|c| c.clone()).unwrap();
    Ok(config)
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {

    async fn message(&self, context: Context, msg: Message) {
        info!("message received");

        if msg.content == "/set clips" {
            info!("/set clips message received");
            let target_channel = msg.channel_id.to_string();
            std::fs::write("C:/Program Files/ytdcbot/data/target_channel.txt", target_channel).expect("Cannot write target channel id into target_channel");
        }
        if msg.content == "!bot" {
            info!("!bot message received");
            let channel = match msg.channel_id.to_channel(&context).await {
                Ok(channel) => channel,
                Err(why) => {
                    error!("Error getting channel: {why:?}");
                    return;
                },
            };


            let video_id = match fetch_latest_video_id().await {
                Ok(id) => id,
                Err(err) => {
                    error!("Error fetching video ID: {}", err);
                    return;
                },
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
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        Builder::from_default_env().target(Target::Stderr).init();
        info!("{} is connected!", ready.user.name);

        let channel_id = ChannelId::new(std::fs::read_to_string("C:/Program Files/ytdcbot/data/target_channel.txt").unwrap().parse().unwrap());
        let message_content = "test message";

        let mut input = String::new();
        loop{
            stdin().lock().read_line(&mut input).expect("failed reading line");
            input = input.trim().to_string();
            match input.as_str() {
                "1" => check_for_new_video(ctx.clone(), channel_id).await.expect("Error calling check_for_new_video"),
                "q" => break,
                _ => {}
            }
        }

    }

}
pub async fn send_message(context: Context, channel_id: ChannelId, content: &str) -> Result<Message, serenity::Error> {
    channel_id.say(context.http, content).await
}
async fn check_for_new_video(context: Context, channel_id: ChannelId) -> Result<(), Box<dyn Error>> {
    loop {
        trace!("cheking for new video");
        let new_id = is_new_video_uploaded().await.unwrap();
        if new_id != "".to_string() {
            let message_content = MessageBuilder::new()
                .push_bold("test")
                .push("https://www.youtube.com/watch?v=")
                .push(new_id)
                .build();
            send_message(context.clone(), channel_id, &message_content).await.expect("Couldn't send message");
            info!("New video message send");
        } else {
            trace!("No new video, sleep for 1 hour");
            sleep(std::time::Duration::from_secs(30));
        }
    }
}

async fn fetch_latest_video_id() -> Result<String, Box<dyn Error>> {

    let youtube_key = use_config()?.get::<&str>("youtube_key")?;

    let client = reqwest::Client::new();
    let url = format!("https://www.googleapis.com/youtube/v3/search?part=snippet&channelId=UCdKcHd14z3ej-EqtT5GdI1g&order=date&maxResults=1&key={:?}", youtube_key);

    let resp = client.get(url).send().await?;
    let text = resp.text().await?;

    let json_value: Value = from_str(&text)?;
    let video_id = json_value["items"][0]["id"]["videoId"].as_str().unwrap().to_string();

    info!("lastest video id fetched");
    Ok(video_id)
}

async fn is_new_video_uploaded() -> Result<String, Box<dyn Error>>{
    let mut old_id = String::new();
    match File::open("C:/Program Files/ytdcbot/data/vid_id.txt") {
        Ok(mut file) => {
            file.read_to_string(&mut old_id).unwrap();
        },
        Err(err) => {

            error!("File vid_id.txt not found, creating a new one");
            std::fs::write("C:/Program Files/ytdcbot/data/vid_id.txt", "").unwrap();
        }
    }
    let id = fetch_latest_video_id().await?;
    trace!("new id is {}", id);
    if old_id != id{
        trace!("new video id founded:{}, old id: {}", id, old_id);
        std::fs::write("C:/Program Files/ytdcbot/data/vid_id.txt", id.clone()).expect("Error while writing new id to vid_id.txt");
        Ok(id)
    }else {
        Ok("".parse().unwrap())
    }

}


#[tokio::main]
async fn main() -> Result<(), ConfigError>{

    let token = use_config()?.get::<&str>("token")?;

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client =
        Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
    Ok(())

}
