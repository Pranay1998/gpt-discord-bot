use serenity::{model::prelude::Message, async_trait, prelude::Context};

use crate::{handler::Handler, ServerError};

use super::{Command, get_commands_for_help};

#[derive(Debug)]
pub struct Help;

pub const PREFIX: &str = "!";
pub const COMMAND: &str = "help";
pub const FULL_COMMAND: &str = "!help";
pub const DESCRIPTION: &str = "Returns a list of commands";
pub const USAGE_EXAMPLE: &str = "!help";

#[async_trait]
impl Command for Help {
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
        let mut commands = String::new();
        for command in get_commands_for_help() {
            commands.push_str(&format!("{} - {}\n", command.get_usage_example(), command.get_description()));
        }
        msg.channel_id.say(&ctx.http, commands).await?;
        Ok(())
    }
}