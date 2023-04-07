use serenity::{async_trait, prelude::Context, model::prelude::Message};

use crate::{ServerError, handler::Handler};

use super::Command;

const PREFIX: &str = "!";
const COMMAND: &str = "gpt-prompt";
const FULL_COMMAND: &str = "!gpt-prompt";
const DESCRIPTION: &str = "Set the system prompt for ChatGPT new questions";
const USAGE_EXAMPLE: &str = "!gpt-prompt <prompt>";

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

    async fn matches(&self, _handler: &Handler, msg: &Message) -> bool {
        msg.content.starts_with(FULL_COMMAND)
    }

    async fn handle(&self, handler: &Handler, _ctx: &Context, msg: &Message) -> Result<(), ServerError> {
        let prompt = msg.content.strip_prefix(FULL_COMMAND).unwrap().trim();
        handler.set_prompt(prompt.to_string());
        Ok(())
    }
}