use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::Client as SerenityClient;
use serenity::prelude::EventHandler;
use serenity::prelude::Context;
use serenity::prelude::GatewayIntents;

use std::env;
use std::process;

use rust_gpt::model::chat_message;
use rust_gpt::model::chat_request;
use rust_gpt::client::client::OpenAIClient;

mod error;

struct Handler {
    open_ai_client: OpenAIClient
}

impl Handler {
    pub fn new(open_ai_api_key: String) -> Handler {
        Handler {
            open_ai_client: OpenAIClient::new(open_ai_api_key)
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            println!("{}", msg.content);
            // Sending a message can fail, due to a network error, an
            // authentication error, or lack of permissions to post in the
            // channel, so log to stdout when some error happens, with a
            // description of it.
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                eprintln!("Error sending message: {:?}", why);
            }
        } else if msg.content.starts_with("!ping gpt ") {
            let question = msg.content.strip_prefix("!ping gpt ").expect("Expected string to start with !ping gpt");
            
            let message = chat_message::Message {
                role: chat_message::Role::User,
                content: question.to_string(),
            };

            let request = chat_request::ChatRequest {
                model: "gpt-3.5-turbo".to_string(),
                messages: vec![
                    chat_message::Message {
                        role: chat_message::Role::System,
                        content: "You are a bot that answers questions properly.".to_string(),
                    },
                    message
                ],
                temperature: Some(1_f64),
                top_p: None,
                stream: Some(false),
                n: Some(1),
                max_tokens: None,
            };

            let response = match self.open_ai_client.get_chat_completion(&request).await {
                Ok(response) => response,
                Err(why) => {
                    eprint!("Error getting a response from ChatGpt: {:?}", why);
                    process::exit(1)
                },
            };

            let message: &str = match rust_gpt::get_chat_message(&response, 0) {
                Some(message) => message,
                None => "Failed to get a response from ChatGPT"
            };

            if let Err(err) = msg.channel_id.say(&ctx.http, message).await {
                eprintln!("Error sending message: {:?}", err);
            }
        }
    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() -> Result<(), error::ServerError> {
    let discord_token = env::var("DISCORD_TOKEN")?;
    let openai_token = env::var("OPENAI_TOKEN")?;
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

        let handler = Handler::new(openai_token);

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client =
        SerenityClient::builder(discord_token, intents).event_handler(handler).await?;

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    client.start().await?;
    Ok(())
}