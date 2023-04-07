mod command;
mod ping;
mod gpt;
mod reply;
mod help;
mod error;
mod prompt;

pub use command::Command;
pub use error::CommandError;
use ping::Ping;
use gpt::Gpt;
use reply::GptReply;
use help::Help;
use prompt::GptPrompt;

static COMMANDS: &'static [&dyn Command] = &[
    &Ping,
    &Gpt,
    &Help,
    &GptPrompt,
    &GptReply, // This matches all messages not sent by the bot, so it should be last
];

static COMMANDS_HELP: &'static [&dyn Command] = &[
    &Help,
    &Ping,
    &GptPrompt,
    &Gpt,
    &GptReply, // This matches all messages not sent by the bot, so it should be last
];

pub fn get_commands() -> &'static [&'static dyn Command] {
    COMMANDS
}

pub fn get_commands_for_help() -> &'static [&'static dyn Command] {
    COMMANDS_HELP
}