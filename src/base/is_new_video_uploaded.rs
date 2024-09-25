use crate::base::fetch;
use log::{error, info, trace};
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Read;

pub async fn is_new_video_uploaded() -> Result<String, Box<dyn Error>> {
    //initializing an old id var
    let mut old_id = String::new();
    //does open vid.txt file and reads its content to old_id
    match File::open("vid_id.txt") {
        Ok(mut file) => {
            file.read_to_string(&mut old_id).unwrap();
        }
        Err(_err) => {
            error!("File vid_id.txt not found, creating a new one");
            fs::write("vid_id.txt", "").unwrap();
        }
    }
    //creates id variable containing videoId of last video of channel
    let id = fetch::fetch_latest_video_id().await?;
    info!("fetched id is {}", id);
    //compares old and new id
    if old_id != id {
        trace!("new video id founded:{}, old id: {}", id, old_id);
        //in case old id and id are not same it write vid_id.txt with new id
        fs::write("vid_id.txt", id.clone()).expect("Error while writing new id to vid_id.txt");
        Ok(id)
    } else {
        info!("id = old_id");
        Ok("".parse().unwrap())
    }
}
