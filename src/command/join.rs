use serenity::{async_trait, prelude::Context, model::prelude::Message};

use crate::{ServerError, handler::Handler};

use super::Command;

pub const PREFIX: &str = "!";
pub const COMMAND: &str = "join";
pub const FULL_COMMAND: &str = "!join";
pub const DESCRIPTION: &str = "Join a random voice channel";
pub const USAGE_EXAMPLE: &str = "!join";

#[derive(Debug)]
pub struct Join;

pub async fn join_channel(command: &dyn Command, handler: &Handler, ctx: &Context, msg: &Message) -> Result<(), ServerError> {
    let user_id = msg.author.id.0;
    let guild_id =  match msg.guild_id {
        Some(guild_id) => guild_id,
        None => return command.command_error("This command can only be used in a guild".to_owned()),
    };

    match handler.get_voice_state_for_user(user_id).await.map(|vs| vs.channel_id).unwrap_or(None) {
        Some(channel_id) => {            
            let manager = songbird::get(ctx).await.expect("Songbird not initialized").clone();
            manager.join(guild_id, channel_id).await.1?;
        },
        None => return command.command_error("You must be in a voice channel to use this command".to_owned()),
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

    async fn matches(&self, _handler: &Handler, msg: &Message) -> bool {
        msg.content.starts_with(FULL_COMMAND)
    }

    async fn handle(&self, handler: &Handler, ctx: &Context, msg: &Message) -> Result<(), ServerError> {
        join_channel(self, handler, ctx, msg).await
    }
}
