use yew::{function_component, html, Callback, Html, Properties};

use super::MessageItem;
use crate::models::Message;

#[derive(Properties, PartialEq)]
pub struct MessageListProps {
    pub messages: Vec<Message>,
    pub on_delete_message: Callback<(String, String)>,
}

#[function_component(MessageList)]
pub fn message_list(
    MessageListProps {
        messages,
        on_delete_message,
    }: &MessageListProps,
) -> Html {
    let messages: Html = messages
        .iter()
        .map(|message| html!( <MessageItem message={message.clone()} on_delete_message={on_delete_message} /> ))
        .collect();

    html!(
        <ul id="message-list">
            {messages}
        </ul>
    )
}
