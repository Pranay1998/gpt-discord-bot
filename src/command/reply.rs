use ogpt::model::chat_completions;
use serenity::{async_trait, prelude::Context, model::prelude::Message};

use crate::{ServerError, handler::Handler, handler::MessageLite};

use super::{Command, gpt};

pub const DESCRIPTION: &str = "After getting a response from ChatGPT, you can reply to continue the conversation";
pub const USAGE_EXAMPLE: &str = "<reply>";

#[derive(Debug)]
pub struct GptReply;

#[async_trait]
impl Command for GptReply {
    fn get_prefix(&self) -> &'static str {
        ""
    }

    fn get_command(&self) -> &'static str {
        ""
    }

    fn get_description(&self) -> &'static str {
        DESCRIPTION
    }

    fn get_usage_example(&self) -> &'static str {
        USAGE_EXAMPLE
    }

    async fn matches(&self, msg: &Message) -> bool {
       return !msg.author.bot; 
    }

    async fn handle(&self, handler: &Handler, ctx: &Context, msg: &Message) -> Result<(), ServerError> {
        let mut msg_list: Vec<chat_completions::Message> = vec![];
        let mut cur_msg_option: Option<MessageLite> = Some(MessageLite::from_msg(msg, ctx));
        let mut is_valid: bool = false;
        let mut expecting_own_msg = false;

        while let Some(cur_msg) = cur_msg_option {
            let is_own = cur_msg.is_own;
            println!("is_own: {}, expecting_own_msg: {}", is_own, expecting_own_msg);
            if is_own != expecting_own_msg || (!is_own && msg.author.bot) { break; }
            let first_question = cur_msg.content.strip_prefix(gpt::FULL_COMMAND);
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
                    let role = if is_own {
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

                    cur_msg_option = handler.get_referenced_from_cache(&cur_msg);
                },
            }
            expecting_own_msg = !expecting_own_msg;
        }

        msg_list.push(
            chat_completions::Message {
                role: chat_completions::Role::System,
                content: handler.get_prompt(),
            }
        );

        if is_valid {
            msg_list.reverse();
            
            let response = handler.get_gpt_response(msg_list).await?;

            let message: &str = match ogpt::utils::get_chat_message(&response, 0) {
                Some(message) => message,
                None => {
                    return self.command_error(String::from("Failed to get 0th choice from response"));
                }
            };

            msg.reply(&ctx.http, message).await?;
        }
        Ok(())
    }
}
