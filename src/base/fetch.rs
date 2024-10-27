use crate::startup::YouTubeDiscordBotSettings;
use log::info;
use serde_json::Value;
use serde_yaml::from_str;
use std::error::Error;

// #fetch last user's YouTube video

pub async fn fetch_latest_video_id(
    youtube: &YouTubeDiscordBotSettings,
) -> Result<String, Box<dyn Error>> {
    //loading keys from Config
    //let youtube_key = use_config()?.get::<String>("youtube_key")?;
    //channel = use_config()?.get::<String>("youtube_channel")?;

    //configuring client and url for request
    //creating new instance of request::Client
    let client = reqwest::Client::new();

    /*Url takes CHANNEL and YOUTUBE_KEY, and does a request to YouTube API where part = snippet, channelId is CHANNEL_ID
    it does order videos of CHANNEL_ID YouTube channel by date and as Results 1 it shows LAST video of YouTube channel*/
    let url = format!("https://www.googleapis.com/youtube/v3/search?part=snippet&channelId={}&order=date&maxResults=1&key={}"
        ,youtube.channel
        ,youtube.youtube_key
        );

    //does request to a YouTube API
    let resp = client
        .get(url)
        .send()
        .await
        .expect("Failed to send request to a YouTube API");

    //takes a json as a String answer of YouTube API
    let text = resp
        .text()
        .await
        .expect("Failed to convert response to String");
    //println!("{text}");
    //does parse json to get videoId (Id of last video)
    let json_value: Value = from_str(&text).expect("Failed to parse text response to a JSON");
    let video_id = json_value["items"][0]["id"]["videoId"]
        .as_str()
        .unwrap()
        .to_string();

    info!("Last video ID fetched");
    Ok(video_id)
}
