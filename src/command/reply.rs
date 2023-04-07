use ogpt::model::chat_completions;
use serenity::{async_trait, prelude::Context, model::prelude::Message};

use crate::{ServerError, handler::Handler, handler::MessageLite};

use super::{Command, CommandError};

const PREFIX: &str = "";
const COMMAND: &str = "";
const DESCRIPTION: &str = "After getting a response from ChatGPT, you can reply to continue the conversation";
const USAGE_EXAMPLE: &str = "<reply>";

#[derive(Debug)]
pub struct GptReply;

#[async_trait]
impl Command for GptReply {
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
        msg.author.name != "tbot"
    }

    async fn handle(&self, handler: &Handler, ctx: &Context, msg: &Message) -> Result<(), ServerError> {
        let mut msg_list: Vec<chat_completions::Message> = vec![];
        let mut cur_msg_option: Option<MessageLite> = Some(MessageLite::from_msg(&msg));
        let mut is_valid: bool = false;

        while let Some(cur_msg) = cur_msg_option {
            let first_question = cur_msg.content.strip_prefix("!gpt");
            match first_question {
                Some(first_question) => {
                    msg_list.push(
                        chat_completions::Message {
                            role: chat_completions::Role::User,
                            content: first_question.to_string()
                        }
                    );
                    is_valid = true;
                    cur_msg_option = None;
                },
                None => {
                    let role = if cur_msg.author_name == "tbot" { 
                        chat_completions::Role::Assistant
                    } else {
                        chat_completions::Role::User
                    };

                    msg_list.push(
                        chat_completions::Message {
                            role,
                            content: cur_msg.content.to_string(),
                        }
                    );

                    cur_msg_option = handler.get_referenced(&cur_msg);
                },
            }
        }

        let content = String::from("You are a bot that answers questions accurately.");

        msg_list.push(
            chat_completions::Message {
                role: chat_completions::Role::System,
                content,
            }
        );

        if is_valid {
            msg_list.reverse();
            
            let request = chat_completions::ChatCompletionsRequest::default(String::from("gpt-3.5-turbo"), msg_list);
            let response = handler.ogpt_async_client.chat_completion_async(&request).await?;

            let message: &str = match ogpt::utils::get_chat_message(&response, 0) {
                Some(message) => message,
                None => {
                    return Err(ServerError::CommandError(
                        CommandError::new(self.get_command().to_owned(), String::from("Failed to get 0th choice from response")))
                    );
                }
            };

            msg.reply(&ctx.http, message).await?;
        }
        Ok(())
    }
}