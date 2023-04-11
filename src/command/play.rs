use serenity::{async_trait, prelude::Context, model::prelude::Message};
use songbird::input::{Restartable, Input};

use crate::{ServerError, handler::Handler};

use super::Command;

pub const PREFIX: &str = "!";
pub const COMMAND: &str = "play";
pub const FULL_COMMAND: &str = "!play";
pub const DESCRIPTION: &str = "Plays a song from Youtubel";
pub const USAGE_EXAMPLE: &str = "!play <url>";

#[derive(Debug)]
pub struct Play;

#[async_trait]
impl Command for Play {
    fn get_prefix(&self) -> &'static str {
        PREFIX
    }

    fn get_command(&self) -> &'static str {
        COMMAND
    }

    fn get_description(&self) -> &'static str {
        DESCRIPTION
    }

    fn get_usage_example(&self) -> &'static str {
        USAGE_EXAMPLE
    }

    async fn matches(&self, _handler: &Handler, msg: &Message) -> bool {
        msg.content.starts_with(FULL_COMMAND)
    }

    async fn handle(&self, _: &Handler, ctx: &Context, msg: &Message) -> Result<(), ServerError> {
        let search_string = msg.content.strip_prefix(FULL_COMMAND).unwrap().trim().to_owned();
        let guild_id = msg.guild_id.unwrap();

        let manager = songbird::get(ctx).await.unwrap().clone();
        let handler = manager.get(guild_id).unwrap();
        let mut handler = handler.lock().await;

        let source = Restartable::ytdl_search(search_string, true).await.unwrap();
        let source: Input = source.into();
        
        msg.channel_id.say(&ctx.http, format!("Added song to the queue {}", source.metadata.source_url.clone().unwrap())).await.unwrap();

        handler.enqueue_source(source);
        
        Ok(())
    }
}
use serenity::{async_trait, prelude::Context, model::prelude::Message};
use songbird::input::{Restartable, Input};

use crate::{ServerError, handler::Handler};

use super::Command;

pub const PREFIX: &str = "!";
pub const COMMAND: &str = "play";
pub const FULL_COMMAND: &str = "!play";
pub const DESCRIPTION: &str = "Plays a song from Youtubel";
pub const USAGE_EXAMPLE: &str = "!play <url>";

#[derive(Debug)]
pub struct Play;

#[async_trait]
impl Command for Play {
    fn get_prefix(&self) -> &'static str {
        PREFIX
    }

    fn get_command(&self) -> &'static str {
        COMMAND
    }

    fn get_description(&self) -> &'static str {
        DESCRIPTION
    }

    fn get_usage_example(&self) -> &'static str {
        USAGE_EXAMPLE
    }

    async fn matches(&self, _handler: &Handler, msg: &Message) -> bool {
        msg.content.starts_with(FULL_COMMAND)
    }

    async fn handle(&self, _: &Handler, ctx: &Context, msg: &Message) -> Result<(), ServerError> {
        let search_string = msg.content.strip_prefix(FULL_COMMAND).unwrap().trim().to_owned();
        let guild_id = msg.guild_id.unwrap();

        let manager = songbird::get(ctx).await.unwrap().clone();
        let handler = manager.get(guild_id).unwrap();
        let mut handler = handler.lock().await;

        let source = Restartable::ytdl_search(search_string, true).await.unwrap();
        let source: Input = source.into();
        
        msg.channel_id.say(&ctx.http, format!("Added song to the queue {}", source.metadata.source_url.clone().unwrap())).await.unwrap();

        handler.enqueue_source(source);
        
        Ok(())
    }
}
