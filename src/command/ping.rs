use serenity::{async_trait, prelude::Context, model::prelude::Message};

use crate::{ServerError, handler::Handler};

use super::Command;

const PREFIX: &str = "!";
const COMMAND: &str = "ping";
const DESCRIPTION: &str = "Returns Pong!";
const USAGE_EXAMPLE: &str = "!ping";

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

    fn matches(&self, msg: &Message) -> bool {
        msg.content == "!ping"
    }

    async fn handle(&self, _handler: &Handler, ctx: &Context, msg: &Message) -> Result<(), ServerError> {
        msg.channel_id.say(&ctx.http, "Pong!").await?;
        Ok(())
    }
}