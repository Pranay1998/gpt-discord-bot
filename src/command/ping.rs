use serenity::{async_trait, prelude::Context, model::prelude::Message};

use crate::{ServerError, handler::Handler};

use super::Command;

pub const PREFIX: &str = "!";
pub const COMMAND: &str = "ping";
pub const FULL_COMMAND: &str = "!ping";
pub const DESCRIPTION: &str = "Returns Pong!";
pub const USAGE_EXAMPLE: &str = "!ping";

#[derive(Debug)]
pub struct Ping;

#[async_trait]
impl Command for Ping {
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
        msg.channel_id.say(&ctx.http, "Pong!").await?;
        Ok(())
    }
}