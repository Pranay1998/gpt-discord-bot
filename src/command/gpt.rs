use ogpt::model::chat_completions;
use serenity::{async_trait, prelude::Context, model::prelude::Message};

use crate::{ServerError, handler::Handler};

use super::{Command, CommandError};

const PREFIX: &str = "!";
const COMMAND: &str = "gpt";
const DESCRIPTION: &str = "Generates a response using ChatGPT";
const USAGE_EXAMPLE: &str = "!gpt <prompt>";

#[derive(Debug)]
pub struct Gpt;

#[async_trait]
impl Command for Gpt {
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
        msg.content.starts_with(&format!("{}{}", self.get_prefix(), self.get_command()))
    }

    async fn handle(&self, handler: &Handler, ctx: &Context, msg: &Message) -> Result<(), ServerError> {
        let question = match msg.content.strip_prefix("!gpt") {
            Some(question) => question.trim(),
            None => {
                return Err(ServerError::CommandError(
                    CommandError::new(self.get_command().to_owned(), String::from("Failed to get a question from the message")))
                );
            },
        };
            
        let message = chat_completions::Message {
            role: chat_completions::Role::User,
            content: question.to_string(),
        };

        let content = String::from("You are a bot that answers questions accurately.");

        let messages = vec![
            chat_completions::Message {
                role: chat_completions::Role::System,
                content,
            },
            message
        ];

        let request = chat_completions::ChatCompletionsRequest::default(String::from("gpt-3.5-turbo"), messages);

        let response = handler.ogpt_async_client.chat_completion_async(&request).await?;

        let message: &str = match ogpt::utils::get_chat_message(&response, 0) {
            Some(message) => message,
            None => {
                return Err(ServerError::CommandError(
                    CommandError::new(self.get_command().to_owned(), String::from("Failed to get a response from ChatGPT")))
                ); 
            }
        };

        msg.reply(&ctx.http, message).await?;
        Ok(())
    }
}