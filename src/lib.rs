mod error;
mod handler;
mod command;

pub use error::ServerError;
use serenity::prelude::GatewayIntents;
use serenity::prelude::Client as SerenityClient;
use songbird::SerenityInit;

pub async fn start_server(discord_token: String, openai_token: String) -> Result<(), error::ServerError> {
    let intents = GatewayIntents::non_privileged()
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_VOICE_STATES;

    let handler = handler::Handler::new(openai_token, 350, None);

    let mut client =
        SerenityClient::builder(discord_token, intents)
            .event_handler(handler)
            .register_songbird()
            .await?;

    client.start().await?;
    Ok(())
}