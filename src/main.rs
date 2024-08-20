
use std::error::Error;
use serde_json::{Value, from_str};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::guild::Member;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, Read, stdin};
use std::{fs, thread};
use std::sync::mpsc::channel;
use std::thread::sleep;
use log::{error, info, Level, LevelFilter, trace};
use env_logger::{Builder, Target};
use serenity::all::{ChannelId, CommandInteraction};
use time::Duration;
use config::{Config, ConfigBuilder, ConfigError};
use lazy_static::lazy_static;
use serde::Serialize;

struct ChannelsList {
    youtube_channel: String,
    target_channel_id: u32,
}
//setting up config as lazy_static
lazy_static! {
    static ref CONFIG: Result<Config, ConfigError> = {
            let mut builder: Config = Config::builder()
                .add_source(config::File::with_name("src/config.toml"))
                .build()?;
            Ok(builder)
    };
}
//:Config val to reach keys
fn use_config() -> Result<Config, ConfigError>{
    let config = CONFIG.as_ref().map(|c| c.clone()).unwrap();
    Ok(config)
}

//          /\_____/\
//         /  o   o  \
//        ( ==  ^  == )
//         )         (
//        (  \     /  )
//         \  \   /  /
//          `-    -`

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    //a block for handling messages received by bot
    async fn message(&self, context: Context, msg: Message) {
        info!("message received");
        match msg.content.as_str() {
            //sets a current channel as a target channel (look what target channel is in README.md)
            "/set clips" => {
                info!("/set clips message received");

                let target_channel = msg.channel_id.to_string();

                //responses to /set clips
                let mut response: String = MessageBuilder::new()
                    .push("setting up clips-channel...")
                    .build();
                //saying a massage
                msg.channel_id.say(&context.http, &response).await.expect("Message sending failed");
                match std::fs::write("target_channel.txt", target_channel){
                    Ok(()) => {
                        response = MessageBuilder::new().push("target-channel set up successful").build();
                    }
                    Err(..) => {
                        let response = MessageBuilder::new().push_bold("target-channel set up failed").build();
                    }
                }
                msg.channel_id.say(&context.http, &response).await.expect("Message sending failed");

            },
            //unimplemented feature (do not use)
            "/set channel" => {
                info!("new /set channel request");
                let channel = msg.content;
                std::fs::write("channel.txt", channel).expect("failed writing channel.txt");
            },
            /*
            Asks a fetch_latest_video_id
            (request to google YouTubeV3 API for last video id of user channel[setup user channel in config.toml]
             and answer to a !bot initiator with a link to last video)

             */
            "!bot" => {
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
            },
            _ => {}
        }

    }

    async fn ready(&self, ctx: Context, ready: Ready) {

        //starting env_logger
        Builder::from_default_env().target(Target::Stderr).init();
        info!("{} is connected!", ready.user.name);

        //work with json of channels


        let mut holder = String::new();
        match File::open("target_channel.txt") {
            Ok(mut file) => {
                file.read_to_string(&mut holder).unwrap();
            },
            Err(_err) => {
                error!("File target_channel.txt not found, creating a new one");
                std::fs::write("target_channel.txt", "").unwrap();
            }
        }
        let channel_id = ChannelId::new(holder.parse().unwrap());

        //let channel_id = ChannelId::new(std::fs::read_to_string("target_channel.txt").unwrap().parse().unwrap());
        let message_content = "test message";

        check_for_new_video(ctx, channel_id).await.expect("failed to start check_for_new_video");
    }

}
//does send message in given channelId with provided content and context
pub async fn send_message(context: Context, channel_id: ChannelId, content: &str) -> Result<Message, serenity::Error> {
    channel_id.say(context.http, content).await
}
//checks for new video and sends it in TARGET CHANNEL
async fn check_for_new_video(context: Context, channel_id: ChannelId) -> Result<(), Box<dyn Error>> {
    //starts a loop of checking for new video
    loop {
        trace!("checking for new video");
        //define new_id with is_new_video_uploaded() function which returns id
        let new_id = is_new_video_uploaded().await.unwrap();
        if new_id != "".to_string() {
            //in case new_id is not empty (that means there is new video uploaded)
            //creates message content with MessageBuilder
            let message_content = MessageBuilder::new()
                .push_bold("test")
                .push("https://www.youtube.com/watch?v=")
                .push(new_id)
                .build();
            //calls send_message to send it in TARGET CHANNEL
            send_message(context.clone(), channel_id, &message_content).await.expect("Couldn't send message");
            info!("New video message send");
        } else {
            info!("No new video, sleep for 1 hour");
            //in case new_id is empty it does sleep for 1 hour (for debug it may be less)
            tokio::time::sleep(std::time::Duration::from_secs(30)).await;
        }
    }
}
async fn is_new_video_uploaded() -> Result<String, Box<dyn Error>>{
    //initializing an old id var
    let mut old_id = String::new();
    //does open vid.txt file and reads its content to old_id
    match File::open("vid_id.txt") {
        Ok(mut file) => {
            file.read_to_string(&mut old_id).unwrap();
        },
        Err(err) => {
            error!("File vid_id.txt not found, creating a new one");
            std::fs::write("vid_id.txt", "").unwrap();
        }
    }
    //creates id variable containing videoId of last video of channel
    let id = fetch_latest_video_id().await?;
    info!("fetched id is {}", id);
    //compares old and new id
    if old_id != id{
        trace!("new video id founded:{}, old id: {}", id, old_id);
        //in case old id and id are not same it write vid_id.txt with new id
        std::fs::write("vid_id.txt", id.clone()).expect("Error while writing new id to vid_id.txt");
        Ok(id)
    }else {
        info!("id = old_id");
        Ok("".parse().unwrap())
    }

}
//fetches last video id of YouTube channel
async fn fetch_latest_video_id() -> Result<String, Box<dyn Error>> {

    //loading keys from Config
    let youtube_key = use_config()?.get::<String>("youtube_key")?;
    let channel = use_config()?.get::<String>("youtube_channel")?;

    //configuring client and url for request
    //creating new instance of request::Client
    let client = reqwest::Client::new();

    /*Url takes CHANNEL and YOUTUBE_KEY, and does a request to YouTube API where part = snippet, channelId is CHANNEL_ID
    it does order videos of CHANNEL_ID YouTube channel by date and as Results 1 it shows LAST video of YouTube channel*/
    let url = format!("https://www.googleapis.com/youtube/v3/search?part=snippet&channelId={}&order=date&maxResults=1&key={}",channel, youtube_key);

    //does request to a YouTube API
    let resp = client.get(url).send().await?;

    //takes a json as a String answer of YouTube API
    let text = resp.text().await?;
    //println!("{text}");
    //does parse json to get videoId (Id of last video)
    let json_value: Value = from_str(&text)?;
    let video_id = json_value["items"][0]["id"]["videoId"].as_str().unwrap().to_string();

    info!("lastest video id fetched");
    Ok(video_id)
}
//Checks is new video uploaded


//configuring bot
#[tokio::main]
async fn main() -> Result<(), ConfigError>{
    //takes token from Config
    let token = use_config()?.get::<String>("token")?;

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
