use serenity::{async_trait, prelude::Context, model::prelude::{Message, ChannelType}};

use crate::{ServerError, handler::Handler};

use super::Command;

pub const PREFIX: &str = "!";
pub const COMMAND: &str = "join";
pub const FULL_COMMAND: &str = "!join";
pub const DESCRIPTION: &str = "Join a random voice channel";
pub const USAGE_EXAMPLE: &str = "!join";

#[derive(Debug)]
pub struct Join;


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

    async fn matches(&self, _handler: &Handler, msg: &Message) -> bool {
        msg.content.starts_with(FULL_COMMAND)
    }

    async fn handle(&self, handler: &Handler, ctx: &Context, msg: &Message) -> Result<(), ServerError> {
        let guild_id = msg.guild_id.unwrap();
        let mut def_voice_channel = handler.default_voice_channel.write().await;

        if def_voice_channel.is_none() {
            *def_voice_channel = Some(ctx.http.get_channels(guild_id.0).await.unwrap().iter().find(|x| x.kind == ChannelType::Voice).unwrap().id.0);
        }
        
        let channel_id = def_voice_channel.unwrap();
        let manager = songbird::get(ctx).await.unwrap().clone();

        manager.join(guild_id, channel_id).await.1.unwrap();
        Ok(())
    }
}
