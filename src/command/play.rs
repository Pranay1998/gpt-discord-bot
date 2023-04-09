use serenity::{async_trait, prelude::Context, model::prelude::{Message, ChannelId}};

use crate::{ServerError, handler::Handler};

use super::{Command, CommandError};

pub const PREFIX: &str = "!";
pub const COMMAND: &str = "play";
pub const FULL_COMMAND: &str = "!play";
pub const DESCRIPTION: &str = "Plays a song from Youtubel";
pub const USAGE_EXAMPLE: &str = "!play <url>";

#[derive(Debug)]
pub struct Play;

#[async_trait]
impl Command for Play {
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

    async fn handle(&self, handler: &Handler, ctx: &Context, msg: &Message) -> Result<(), ServerError> {
        let url = msg.content.strip_prefix(FULL_COMMAND).unwrap().trim();

        if !url.starts_with("http") {
            return Err(ServerError::CommandError(CommandError::new(COMMAND.to_owned(), String::from("Invalid URL"))));
        }

        let guild_id = msg.guild_id.unwrap();
        let mut def_voice_channel = handler.default_voice_channel.read().await.clone();

        if def_voice_channel.is_none() {
            let channels = ctx.http.get_channels(guild_id.0).await?;
            for channel in channels {
                if channel.kind == serenity::model::channel::ChannelType::Voice {
                    def_voice_channel = Some(channel.id.0);
                    handler.default_voice_channel.write().await.replace(channel.id.0);
                    break;
                }
            }
        }

        if let Some(channel_id) = def_voice_channel {
            let manager = songbird::get(ctx).await.unwrap().clone();

            let _handler = manager.join(guild_id, ChannelId(channel_id)).await;
    
            if let Some(handler_lock) = manager.get(guild_id) {
                let mut handler = handler_lock.lock().await;
                let source = songbird::ytdl(url).await.unwrap();
                let _handle = handler.play_only_source(source);
            } else {
                return Err(ServerError::CommandError(CommandError::new(COMMAND.to_owned(), String::from("Cannot acquire handler"))));
            }
        } else {
            return Err(ServerError::CommandError(CommandError::new(COMMAND.to_owned(), String::from("No voice channel found"))));
        }

        Ok(())
    }
}
