use tokio::time::{Duration, sleep};
use serenity::{async_trait, prelude::Context, model::prelude::Message};

use crate::{ServerError, handler::Handler};

use super::Command;

pub const PREFIX: &str = "!";
pub const COMMAND: &str = "join";
pub const FULL_COMMAND: &str = "!join";
pub const DESCRIPTION: &str = "Join voice channel that you are currently in.";
pub const USAGE_EXAMPLE: &str = "!join";

#[derive(Debug)]
pub struct Join;

pub async fn join_channel(command: &dyn Command, ctx: &Context, msg: &Message) -> Result<(), ServerError> {
    match msg.guild(&ctx.cache) {
        Some(guild) => {
            let guild_id = guild.id;

            let channel_id = guild
                .voice_states
                .get(&msg.author.id)
                .and_then(|voice_state| voice_state.channel_id);

            match channel_id {
                Some(channel_id) => {
                    let manager = songbird::get(ctx).await.expect("Songbird not initialized").clone();
                    
                    manager.join(guild_id, channel_id).await.1?;
                    tokio::spawn(async move {
                        let mut backoff_seconds = 30;
                        let handler = manager.get(guild_id).expect("No handler found");

                        loop {
                            sleep(Duration::from_secs(backoff_seconds)).await;
                            if handler.lock().await.queue().is_empty() {
                                manager.remove(guild_id).await.expect("Cannot leave");
                                break;
                            } else {
                                backoff_seconds = backoff_seconds * 2;
                            }
                        }

                    });
                },
                None => return command.command_error("You must be in a voice channel to use this command".to_owned()),
            }

        },
        None => return command.command_error(String::from("Failed to get channel details")),
    }
    Ok(())
}

#[async_trait]
impl Command for Join {
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

    async fn handle(&self, _: &Handler, ctx: &Context, msg: &Message) -> Result<(), ServerError> {
        join_channel(self, ctx, msg).await
    }
}
