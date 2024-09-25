use log::{info, trace};
use serenity::all::{ChannelId, Context, MessageBuilder};
use std::error::Error;

use crate::base::is_new_video_uploaded::is_new_video_uploaded;
use crate::base::send_message::send_message;

pub async fn check_for_new_video(
    context: Context,
    channel_id: ChannelId,
) -> Result<(), Box<dyn Error>> {
    //starts a loop of checking for new video
    loop {
        trace!("checking for new video");
        //define new_id with is_new_video_uploaded() function which returns id
        let new_id = is_new_video_uploaded().await?;
        if new_id != *"" {
            //in case new_id is not empty (that means there is new video uploaded)
            //creates message content with MessageBuilder
            let message_content = MessageBuilder::new()
                .push_bold("test")
                .push("https://www.youtube.com/watch?v=")
                .push(new_id)
                .build();
            //calls send_message to send it in TARGET CHANNEL
            send_message(context.clone(), channel_id, &message_content)
                .await
                .expect("Couldn't send message");
            info!("New video message send");
        } else {
            info!("No new video, sleep for 1 hour");
            //in case new_id is empty it does sleep for 1 hour (for debug it may be less)
            tokio::time::sleep(std::time::Duration::from_secs(30)).await;
        }
    }
}
