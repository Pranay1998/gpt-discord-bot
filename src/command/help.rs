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
            commands.push_str(&format!("`{}` **- {}**\n\n", command.get_usage_example(), command.get_description()));
        }

        let mut embed = serenity::builder::CreateEmbed::default();

        embed
            .title("Help - If you are seeing this, this repo is setup to autodeploy on a push to main using a webhook")
            .description(commands)
            .color(0x90_EE_90); // You can set the desired color here

        msg.channel_id.send_message(&ctx.http, |m| {
            m.set_embed(embed.to_owned()) 
        }).await?;

        Ok(())
    }
}
