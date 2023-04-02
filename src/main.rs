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
    ogpt_async_client: OGptAsyncClient,
    system_prompt: Arc<Mutex<String>>,
    default_prompt: String,
}

impl Handler {
    pub fn new(open_ai_api_key: String) -> Handler {
        Handler {
            ogpt_async_client: OGptAsyncClient::new(open_ai_api_key),
            system_prompt: Arc::new(Mutex::new(String::from("You are a bot that answers questions accurately."))),
            default_prompt: String::from("You are a bot that answers questions accurately."),
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            println!("{}", msg.content);
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

            let messages = vec![
                chat_completions::Message {
                    role: chat_completions::Role::System,
                    content: system_prompt,
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
        } else if msg.content.starts_with("!ping gpt-prompt ") {
            let new_prompt = msg.content.strip_prefix("!ping gpt-prompt ").expect("Expected string to start with !ping gpt prompt");

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
        } else if !msg.is_own(&ctx.cache) { // replies
            let mut msg_list: Vec<chat_completions::Message> = vec![];
            let first_msg_id = msg.id;
            let mut cur_msg_option = Some(msg.clone());
            let mut is_valid: bool = false;

            while let Some(ref cur_msg) = cur_msg_option {
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
                        let role = if cur_msg.is_own(&ctx.cache) { 
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

                        if cur_msg.id == first_msg_id {
                            cur_msg_option = match cur_msg.referenced_message {
                                Some(ref m) => Some((**m).clone()),
                                None => None
                            }
                        } else {
                            // Check cache first, if not present make a http request to get msg
                            let fetched = match ctx.cache.message(cur_msg.channel_id, cur_msg.id) {
                                Some(cache) => Some(cache),
                                None => {
                                    let fetched = ctx.http.get_message(cur_msg.channel_id.0, cur_msg.id.0).await;

                                    match fetched {
                                        Ok(fetched_m) => Some(fetched_m),
                                        Err(_) => None,
                                    }
                                }
                            };

                            cur_msg_option = match fetched {
                                Some(ref m) => {
                                    match m.referenced_message {
                                        Some(ref m) => Some((**m).clone()),
                                        None => None
                                    }
                                },
                                None => None
                            }
                        }
                    },
                }
            }

            msg_list.push(
                chat_completions::Message {
                    role: chat_completions::Role::System,
                    content: self.default_prompt.to_owned()
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