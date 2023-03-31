use model::chat_response;

pub mod model;
pub mod constants;
pub mod client;
pub mod error;

pub fn get_chat_message(response: &chat_response::ChatResponse, index: usize) -> Option<&str> {
    let choice = response
        .choices
        .get(index)?;

    Some(&choice.message.content)
}