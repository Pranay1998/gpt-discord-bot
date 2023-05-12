use serenity::{async_trait, prelude::Context, model::prelude::Message};
use songbird::input::{Restartable, Input};

use crate::{ServerError, handler::Handler};

use super::{Command, join_channel};

pub const PREFIX: &str = "!";
pub const COMMAND: &str = "play";
pub const FULL_COMMAND: &str = "!play";
pub const DESCRIPTION: &str = "Joins current channel and adds the song to the queue";
pub const USAGE_EXAMPLE: &str = "!play <song name>";

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

    async fn matches(&self, msg: &Message) -> bool {
        msg.content.starts_with(FULL_COMMAND)
    }

    async fn handle(&self, _: &Handler, ctx: &Context, msg: &Message) -> Result<(), ServerError> {
        join_channel(self, ctx, msg).await?;

        let search_string = msg.content.strip_prefix(FULL_COMMAND).unwrap().trim().to_owned();
        let guild_id = match msg.guild_id {
            Some(id) => id,
            None => return self.command_error(String::from("This command can only be used in a guild")),
        };

        let manager = songbird::get(ctx).await;
        
        if manager.is_none() {
            return self.command_error(String::from("Songbird not initialized"));
        }

        let manager = manager.unwrap().clone();
        let handler = manager.get(guild_id);
        
        if handler.is_none() {
            return self.command_error(String::from("No handler found"));
        }

        let handler = handler.unwrap();
        let mut handler = handler.lock().await;

        let source =  Restartable::ytdl_search(search_string, true).await?;
        let source: Input = source.into();

        match source.metadata.source_url.clone() {
            Some(url) => {
                msg.channel_id.say(&ctx.http, format!("Added to the queue {}", url)).await?;
                handler.enqueue_source(source);
            },
            None => return self.command_error(String::from("No source url found")),
        }
        
        Ok(())
    }
}
