use std::rc::Rc;
use web_sys::HtmlElement;
use yew::prelude::*;

mod boats_api;
mod components;
mod controllers;
mod models;
mod state;
mod data;

use components::*;
use controllers::*;
use state::*;

use crate::{models::Tree, data::random_intro};

#[function_component(App)]
fn app() -> Html {
    let messages = use_reducer(MessageState::default);
    let messages_controller = Rc::new(MessageController::new(messages.clone()));
    // let proximity = 10000 / messages.proximity.max(1);

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
            messages_controller.update_proximity(lat, long);
        })
    };

    let on_select_tree = {
        let messages_controller = messages_controller.clone();
        Callback::from(move |tree: Tree| {
            messages_controller.init_messages(tree);
        })
    };

    let on_show_more = {
        let messages_controller = messages_controller.clone();
        Callback::from(move |tree: Tree| {
            messages_controller.open_search(tree.name_sci.unwrap_or_default());
        })
    };

    let on_back_to_list = {
        let messages_controller = messages_controller.clone();
        Callback::from(move |_e: MouseEvent| {
            messages_controller.clear_selection();
        })
    };

    let intro_node_ref = use_node_ref();
    let on_intro_continue = {
        let intro_node_ref = intro_node_ref.clone();
        Callback::from(move |_e: MouseEvent| {
            if let Some(intro) = intro_node_ref.cast::<HtmlElement>() {
                intro.set_class_name("intro open");
            } 
        })
    };

    html! {
        <div class="container">
            // <AnimatedTree completion={proximity} />
            <h1>{ "Based On A Tree Story" }</h1>

            <Geolocation on_coords_change={on_coords_change} />

            if let Some(tree) = &messages.current_tree {
                <div class="selection">
                    <span onclick={on_back_to_list}>{"⮪"}</span>
                    <TreeInfos tree={tree.clone()} on_show_more={on_show_more} />
                    <MessageForm current_tree_id={tree.tree_id.clone()} on_create_message={on_create_message} />

                    <MessageList
                        messages={messages.messages.clone()}
                        on_delete_message={on_delete_message}
                    />
                </div>

            } else if messages.trees.len() > 0 {
                    <TreeList trees={messages.trees.clone()} on_select_tree={on_select_tree} />

            } else {
                <div ref={intro_node_ref} class="intro" onclick={on_intro_continue}>
                    <SafeHtml html={random_intro()} />
                </div>
            }
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
