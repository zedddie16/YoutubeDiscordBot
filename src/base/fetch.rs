
use log::info;
use serde_json::{from_str, Value};
use std::error::Error;
use crate::base::use_config::use_config;

pub async fn fetch_latest_video_id() -> Result<String, Box<dyn Error>> {
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
    let video_id = json_value["items"][0]["id"]["videoId"]
        .as_str()
        .unwrap()
        .to_string();

    info!("last video id fetched");
    Ok(video_id)
}
