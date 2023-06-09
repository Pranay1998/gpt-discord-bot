use serenity::{async_trait, prelude::Context, model::prelude::Message};

use crate::{ServerError, handler::Handler};

use super::Command;

pub const PREFIX: &str = "!";
pub const COMMAND: &str = "gpt-prompt";
pub const FULL_COMMAND: &str = "!gpt-prompt";
pub const DESCRIPTION: &str = "Set the system prompt for ChatGPT new questions";
pub const USAGE_EXAMPLE: &str = "!gpt-prompt <prompt>";

#[derive(Debug)]
pub struct GptPrompt;

#[async_trait]
impl Command for GptPrompt {
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

    async fn handle(&self, handler: &Handler, ctx: &Context, msg: &Message) -> Result<(), ServerError> {
        let prompt = msg.content.strip_prefix(FULL_COMMAND).unwrap().trim();
        handler.set_prompt(prompt.to_string());
        msg.channel_id.say(&ctx.http, "Prompt set").await?;
        Ok(())
    }
}