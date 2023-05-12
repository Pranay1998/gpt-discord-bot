
use serenity::{async_trait, prelude::Context, model::prelude::Message};

use crate::{ServerError, handler::Handler};

use super::Command;

pub const PREFIX: &str = "!";
pub const COMMAND: &str = "queue";
pub const FULL_COMMAND: &str = "!queue";
pub const DESCRIPTION: &str = "Lists songs in the queue";
pub const USAGE_EXAMPLE: &str = "!queue";

#[derive(Debug)]
pub struct Queue;

#[async_trait]
impl Command for Queue {
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
        msg.content == FULL_COMMAND
    }

    async fn handle(&self, _: &Handler, ctx: &Context, msg: &Message) -> Result<(), ServerError> {
        let guild_id = msg.guild_id.unwrap();

        let manager = songbird::get(ctx).await.unwrap().clone();
        let handler = manager.get(guild_id).unwrap();
        let handler = handler.lock().await;
        handler.queue().stop();
        Ok(())
    }
}
