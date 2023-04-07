use std::fmt::Debug;

use serenity::{async_trait, prelude::Context, model::prelude::Message};

use crate::{ServerError, handler::Handler};

#[async_trait]
pub trait Command : Sync + Debug {
    fn get_prefix(&self) -> &str;
    fn get_command(&self) -> &str;
    fn get_description(&self) -> &str;
    fn get_usage_example(&self) -> &str;
    async fn matches(&self, handler: &Handler, msg: &Message) -> bool;
    async fn handle(&self, handler: &Handler, ctx: &Context, msg: &Message) -> Result<(), ServerError>;
}