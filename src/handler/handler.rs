use ogpt::model::chat_completions;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::EventHandler;
use serenity::prelude::Context;
use std::num::NonZeroUsize;
use std::sync::Arc;
use std::sync::Mutex;

use ogpt::client::OGptAsyncClient;

use lru::LruCache;

use crate::ServerError;
use crate::command;

pub const GPT_DEFAULT_SYSTEM_PROMPT: &str = "You are a bot that answers questions accurately.";

pub struct Handler {
    ogpt_async_client: OGptAsyncClient,
    message_cache: Arc<Mutex<LruCache<u64, MessageLite>>>,
    prompt: Arc<Mutex<String>>,
}

impl Handler {
    pub fn new(open_api_key: String, lru_cache_size: usize, default_prompt: Option<String>) -> Handler {
        let prompt = match default_prompt {
            Some(prompt) => prompt,
            None => String::from(GPT_DEFAULT_SYSTEM_PROMPT),
        };

        Handler {
            ogpt_async_client: OGptAsyncClient::new(open_api_key),
            message_cache: Arc::new(Mutex::new(LruCache::new(NonZeroUsize::new(lru_cache_size).unwrap()))),
            prompt: Arc::new(Mutex::new(prompt)),
        }
    }

    pub fn cache_message(&self, msg: &Message, ctx: &Context) {
        let mut r = self.message_cache.lock().unwrap();
        r.put(msg.id.0, MessageLite::from_msg(msg, ctx));
    }

    pub fn get_referenced_from_cache(&self, msg: &MessageLite) -> Option<MessageLite> {
        match &msg.ref_msg_id {
            Some(ref_id) => {
                let mut r = self.message_cache.lock().unwrap();
                r.get(ref_id).map(|m| m.clone())
            },
            None => None
        }
    }

    pub fn get_prompt(&self) -> String {
        self.prompt.lock().unwrap().to_owned()
    }

    pub fn set_prompt(&self, prompt: String) {
        let mut r = self.prompt.lock().unwrap();
        *r = prompt;
    }

    pub async fn get_gpt_response(&self, messages: Vec<chat_completions::Message>) -> Result<chat_completions::ChatCompletionsResponse, ServerError> {
        let response = self
            .ogpt_async_client
            .chat_completion_async(
                &chat_completions::ChatCompletionsRequest::default(
                    String::from("gpt-3.5-turbo"), messages))
            .await?;
        Ok(response)
    }
}

#[derive(Clone, Debug)]
pub struct MessageLite {
    pub ref_msg_id: Option<u64>,
    pub content: String,
    pub author_name: String,
    pub is_own: bool,
}

impl MessageLite {
    pub fn from_msg(msg: &Message, ctx: &Context) -> MessageLite {
        MessageLite {
            ref_msg_id: msg.referenced_message.as_ref().map(|x| x.id.0),
            content: msg.content.to_owned(),
            author_name: msg.author.name.to_owned(),
            is_own: msg.is_own(&ctx.cache),
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {

        for command in command::get_commands() {
            if !msg.author.bot && command.matches(&msg).await {
                if let Err(err) = command.handle(self, &ctx, &msg).await {
                    if let Err(err) = msg.channel_id.say(&ctx.http, format!("{}", err)).await {
                        eprintln!("Error sending error message - {}", err);
                    }
                }
                break;
            }
        }
        self.cache_message(&msg, &ctx);        
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}
