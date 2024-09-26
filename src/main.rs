use YoutubeDiscordBot::startup::run;

#[tokio::main]
async fn main() {
    run().await.expect("Failed to execute");
}
