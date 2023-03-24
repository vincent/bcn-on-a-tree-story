use web_sys::HtmlInputElement;
use yew::{function_component, html, use_node_ref, Callback, Properties, Html};

#[derive(Properties, PartialEq)]
pub struct MessageFormProps {
    pub current_tree_id: String,
    pub on_create_message: Callback<(String, String)>,
}

#[function_component(MessageForm)]
pub fn tree_form(MessageFormProps { current_tree_id, on_create_message }: &MessageFormProps) -> Html {
    let tree_id_node_ref = use_node_ref();
    let input_node_ref = use_node_ref();

    let on_click = {
        let input_node_ref = input_node_ref.clone();
        let tree_id_node_ref = tree_id_node_ref.clone();
        let on_create_message = on_create_message.clone();

        Callback::from(move |_| {
            let input = input_node_ref.cast::<HtmlInputElement>();
            let tree_id = tree_id_node_ref.cast::<HtmlInputElement>();

            if let Some(tree_id) = tree_id {
                if let Some(input) = input {
                    on_create_message.emit((tree_id.value(), input.value()));
                    input.set_value("");
                }
            }
        })
    };

    html!(
        <div>
            <div class="center message-form">
                <input ref={tree_id_node_ref} value={current_tree_id.clone()} type="hidden" />
                <input ref={input_node_ref} id="new-tree" type="text" placeholder={ "Write me something nice" } />
                <button onclick={on_click}>{"Add"}</button>
            </div>
        </div>
    )
}
