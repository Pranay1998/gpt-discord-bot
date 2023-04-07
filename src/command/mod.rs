mod command;
mod ping;
mod gpt;
mod reply;
mod help;
mod error;

pub use command::Command;
pub use error::CommandError;
use ping::Ping;
use gpt::Gpt;
use reply::GptReply;
use help::Help;

static COMMANDS: &'static [&dyn Command] = &[
    &Ping,
    &Gpt,
    &Help,
    &GptReply, // This matches all messages not sent by the bot, so it should be last
];

static COMMANDS_HELP: &'static [&dyn Command] = &[
    &Help,
    &Ping,
    &Gpt,
    &GptReply, // This matches all messages not sent by the bot, so it should be last
];

pub fn get_commands() -> &'static [&'static dyn Command] {
    COMMANDS
}

pub fn get_commands_for_help() -> &'static [&'static dyn Command] {
    COMMANDS_HELP
}