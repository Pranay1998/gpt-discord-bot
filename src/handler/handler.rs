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

use crate::command;

pub struct Handler {
    pub ogpt_async_client: OGptAsyncClient,
    pub message_cache: Arc<Mutex<LruCache<u64, MessageLite>>>,
    pub prompt: Arc<Mutex<Option<String>>>,
}

impl Handler {
    pub fn new(open_api_key: String, lru_cache_size: usize) -> Handler {
        Handler {
            ogpt_async_client: OGptAsyncClient::new(open_api_key),
            message_cache: Arc::new(Mutex::new(LruCache::new(NonZeroUsize::new(lru_cache_size).unwrap()))),
            prompt: Arc::new(Mutex::new(None)),
        }
    }

    pub fn cache_message(&self, msg: &Message) {
        let mut r = self.message_cache.lock().unwrap();
        r.put(msg.id.0, MessageLite::from_msg(msg));
    }

    pub fn get_referenced(&self, msg: &MessageLite) -> Option<MessageLite> {
        match &msg.ref_msg_id {
            Some(ref_id) => {
                let mut r = self.message_cache.lock().unwrap();
                r.get(ref_id).map(|m| m.clone())
            },
            None => None
        }

    }
}

#[derive(Clone, Debug)]
pub struct MessageLite {
    pub ref_msg_id: Option<u64>,
    pub content: String,
    pub author_name: String,
}

impl MessageLite {
    pub fn from_msg(msg: &Message) -> MessageLite {
        MessageLite {
            ref_msg_id: msg.referenced_message.as_ref().map(|x| x.id.0),
            content: msg.content.to_owned(),
            author_name: msg.author.name.to_owned(),
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        for command in command::get_commands() {
            if command.matches(&msg) {
                if let Err(err) = command.handle(self, &ctx, &msg).await {
                    if let Err(err) = msg.channel_id.say(&ctx.http, format!("Error handling command - {}", err)).await {
                        eprintln!("Error sending error message - {}", err);
                    }
                }
                break;
            }   
        }
        self.cache_message(&msg);        
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}