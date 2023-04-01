use crate::model::chat_completions;

pub fn get_chat_message(response: &chat_completions::ChatCompletionsResponse, index: usize) -> Option<&str> {
    let choice = response
        .choices
        .get(index)?;

    Some(&choice.message.content)
}