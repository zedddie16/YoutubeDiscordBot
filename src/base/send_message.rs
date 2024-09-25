use serenity::all::{ChannelId, Context, Message};

pub async fn send_message(
    context: Context,
    channel_id: ChannelId,
    content: &str,
) -> Result<Message, serenity::Error> {
    channel_id.say(context.http, content).await
}
