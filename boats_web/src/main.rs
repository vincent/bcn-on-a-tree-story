use std::rc::Rc;
use yew::prelude::*;

mod boats_api;
mod components;
mod controllers;
mod models;
mod state;

use components::*;
use controllers::*;
use state::*;

use crate::models::Tree;

#[function_component(App)]
fn app() -> Html {
    let messages = use_reducer(MessageState::default);
    let messages_controller = Rc::new(MessageController::new(messages.clone()));

    let on_create_message = {
        let messages_controller = messages_controller.clone();
        Callback::from(move |(tree_id, title): (String, String)| {
            messages_controller.create_message(tree_id.clone(), title);
        })
    };

    let on_delete_message = {
        let messages_controller = messages_controller.clone();
        Callback::from(move |(tree_id, id): (String, String)| {
            messages_controller.delete_message(tree_id.clone(), id);
        })
    };

    let on_coords_change = {
        let messages_controller = messages_controller.clone();
        Callback::from(move |(lat, long): (f64, f64)| {
            messages_controller.list_trees(lat, long);
        })
    };

    let on_select_tree = {
        let messages_controller = messages_controller.clone();
        Callback::from(move |tree: Tree| {
            messages_controller.init_messages(tree);
        })
    };

    html! {
        <div class="container">
            <h1>{ "Based On A Tree Story" }</h1>

            <Geolocation on_coords_change={on_coords_change} />

            <TreeList trees={messages.trees.clone()} on_select_tree={on_select_tree} />

            if let Some(tree) = &messages.current_tree {
                <MessageForm current_tree_id={tree.tree_id.clone()} on_create_message={on_create_message} />

                <MessageList
                    messages={messages.messages.clone()}
                    on_delete_message={on_delete_message}
                />
            }
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
