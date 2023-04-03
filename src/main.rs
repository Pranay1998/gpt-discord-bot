use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::Client as SerenityClient;
use serenity::prelude::EventHandler;
use serenity::prelude::Context;
use serenity::prelude::GatewayIntents;

use std::collections::HashMap;

use std::env;
use std::process;
use std::sync::Arc;
use std::sync::RwLock;

use ogpt::model::chat_completions;
use ogpt::client::OGptAsyncClient;

mod error;

struct Handler {
    ogpt_async_client: OGptAsyncClient,
    message_cache: Arc<RwLock<HashMap<u64, Message>>>,
}

impl Handler {
    pub fn new(open_api_key: String) -> Handler {
        Handler {
            ogpt_async_client: OGptAsyncClient::new(open_api_key),
            message_cache: Arc::new(RwLock::new(HashMap::new()))
        }
    }

    pub fn cache_message(&self, msg: Message) {
        let mut r = self.message_cache.write().unwrap();
        r.insert(msg.id.0, msg);
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with("!ping gpt") {
            let question = msg.content.strip_prefix("!ping gpt ").expect("Expected string to start with !ping gpt");
            
            let message = chat_completions::Message {
                role: chat_completions::Role::User,
                content: question.to_string(),
            };

            let messages = vec![
                chat_completions::Message {
                    role: chat_completions::Role::System,
                    content: String::from("You are a bot that answers questions accurately."),
                },
                message
            ];

            let request = chat_completions::ChatCompletionsRequest::default(String::from("gpt-3.5-turbo"), messages);

            let response = match self.ogpt_async_client.chat_completion_async(&request).await {
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

            if let Err(err) = msg.reply(&ctx.http, message).await {
                eprintln!("Error sending message: {:?}", err);
            }
        } else if msg.author.name != "tbot"  {
            let mut msg_list: Vec<chat_completions::Message> = vec![];
            let mut cur_msg_option: Option<Message> = Some(msg.clone());
            let mut is_valid: bool = false;

            while let Some(cur_msg) = cur_msg_option {
                let first_question = cur_msg.content.strip_prefix("!ping gpt ");
                match first_question {
                    Some(first_question) => {
                        msg_list.push(
                            chat_completions::Message {
                                role: chat_completions::Role::User,
                                content: first_question.to_string()
                            }
                        );
                        is_valid = true;
                        cur_msg_option = None;
                    },
                    None => {
                        let role = if cur_msg.author.name == "tbot" { 
                            chat_completions::Role::Assistant
                        } else {
                            chat_completions::Role::User
                        };

                        msg_list.push(
                            chat_completions::Message {
                                role,
                                content: cur_msg.content.to_string(),
                            }
                        );

                        cur_msg_option = match &cur_msg.referenced_message {
                            Some(ref_msg) => {
                                let r = self.message_cache.read().unwrap();
                                let msg = r.get(&ref_msg.id.0);
                                if let Some(m) = msg {
                                    Some(m.clone())
                                } else {
                                    None
                                }
                            }
                            None => None
                        };
                    },
                }
            }

            msg_list.push(
                chat_completions::Message {
                    role: chat_completions::Role::System,
                    content: String::from("You are a bot that answers questions accurately."),
                }
            );

            if is_valid {
                msg_list.reverse();
                
                let request = chat_completions::ChatCompletionsRequest::default(String::from("gpt-3.5-turbo"), msg_list);
                let response = match self.ogpt_async_client.chat_completion_async(&request).await {
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

                if let Err(err) = msg.reply(&ctx.http, message).await {
                    eprintln!("Error sending message: {:?}", err);
                }
            }
        }
        self.cache_message(msg)
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() -> Result<(), error::ServerError> {
    let discord_token = env::var("DISCORD_TOKEN")?;
    let openai_token = env::var("OPENAI_TOKEN")?;
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_VOICE_STATES;

    let handler = Handler::new(openai_token);

    let mut client =
        SerenityClient::builder(discord_token, intents).event_handler(handler).await?;

    client.start().await?;
    Ok(())
}