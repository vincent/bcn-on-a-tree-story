use yew::{classes, function_component, html, Callback, Properties, Html};

use crate::models::Message;

#[derive(Properties, PartialEq)]
pub struct MessageItemProps {
    pub message: Message,
    pub on_delete_message: Callback<(String, String)>,
}

#[function_component(MessageItem)]
pub fn message(
    MessageItemProps {
        message,
        on_delete_message,
    }: &MessageItemProps,
) -> Html {
    let list_item_class = match message.completed {
        true => Some("completed"),
        false => None,
    };

    let on_delete_click = {
        let message = message.clone();
        let on_delete_message = on_delete_message.clone();
        move |_| on_delete_message.emit((message.tree_id.clone(), message.id.clone()))
    };

    html! {
        <li class={classes!(list_item_class, "center")}>
            <label>{&message.text}</label>
            <button onclick={on_delete_click}>
                {"Delete"}
            </button>
        </li>
    }
}
