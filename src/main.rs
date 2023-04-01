use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::Client as SerenityClient;
use serenity::prelude::EventHandler;
use serenity::prelude::Context;
use serenity::prelude::GatewayIntents;

use std::env;
use std::process;
use std::sync::Arc;
use std::sync::Mutex;

use ogpt::model::chat_completions;
use ogpt::client::OGptAsyncClient;

mod error;

struct Handler {
    open_ai_client: OGptAsyncClient,
    system_prompt: Arc<Mutex<String>>,
    default_prompt: String
}

impl Handler {
    pub fn new(open_ai_api_key: String) -> Handler {
        Handler {
            open_ai_client: OGptAsyncClient::new(open_ai_api_key),
            system_prompt: Arc::new(Mutex::new(String::from("You are a bot that answers questions accurately."))),
            default_prompt: String::from("You are a bot that answers questions accurately."),
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
            
            let message = chat_completions::Message {
                role: chat_completions::Role::User,
                content: question.to_string(),
            };

            let system_prompt = {
                match self.system_prompt.lock() {
                    Ok(mutex_guard) => (*mutex_guard).clone(),
                    Err(err) => {
                        eprintln!("Error acquiring lock on system_prompt: {:?}", err);
                        self.default_prompt.clone()
                    }
                }
            };

            let request = chat_completions::ChatCompletionsRequest {
                model: "gpt-3.5-turbo".to_string(),
                messages: vec![
                    chat_completions::Message {
                        role: chat_completions::Role::System,
                        content: system_prompt,
                    },
                    message
                ],
                temperature: Some(1_f64),
                top_p: None,
                stream: Some(false),
                n: Some(1),
                max_tokens: None,
            };

            let response = match self.open_ai_client.chat_completion_async(&request).await {
                Ok(response) => response,
                Err(why) => {
                    eprint!("Error getting a response from ChatGpt: {:?}", why);
                    process::exit(1)
                },
            };

            let message: &str = match ogpt::utils::get_chat_message(&response, 0) {
                Some(message) => message,
                None => "Failed to get a response from ChatGPT"
            };

            if let Err(err) = msg.channel_id.say(&ctx.http, message).await {
                eprintln!("Error sending message: {:?}", err);
            }
        } else if msg.content.starts_with("!ping gpt prompt ") {
            let new_prompt = msg.content.strip_prefix("!ping gpt prompt ").expect("Expected string to start with !ping gpt prompt");

            let reply: String = match self.system_prompt.lock() {
                Ok(mut mutex_guard) => {
                    *mutex_guard = new_prompt.to_string();
                    String::from("Successfully changed system prompt.")
                },
                Err(err) => {
                    eprintln!("Error acquiring lock on system_prompt: {:?}", err);
                    String::from("Something went wrong, could not change system prompt.")
                }
            };

            if let Err(err) = msg.channel_id.say(&ctx.http, reply).await {
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
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_VOICE_STATES;

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