use serenity::{async_trait, prelude::Context, model::prelude::Message};

use crate::{ServerError, handler::Handler};

use super::Command;

pub const PREFIX: &str = "!";
pub const COMMAND: &str = "pause";
pub const FULL_COMMAND: &str = "!pause";
pub const DESCRIPTION: &str = "Pause playing song";
pub const USAGE_EXAMPLE: &str = "!pause";

#[derive(Debug)]
pub struct Pause;

#[async_trait]
impl Command for Pause {
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
        handler.queue().pause()?;
        Ok(())
    }
}
