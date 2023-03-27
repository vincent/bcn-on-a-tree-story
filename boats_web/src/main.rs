use std::rc::Rc;
use web_sys::HtmlElement;
use yew::prelude::*;

mod boats_api;
mod components;
mod controllers;
mod data;
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
        let messages = messages.clone();

        Callback::from(move |tree: Tree| {
            let window = web_sys::window().expect("global window does not exists");    
            let document = window.document().expect("expecting a document on window");
            let body = document.body().expect("expecting a body on document");

            let color = vec![ "bg1", "bg2", "bg3", "bg4", "bg5" ][messages.inc];
            body.set_class_name(color);

            messages_controller.init_messages(tree);
        })
    };

    let on_show_more = {
        let messages_controller = messages_controller.clone();
        Callback::from(move |tree: Tree| {
            messages_controller.open_search(tree.name_sci.unwrap_or_default());
        })
    };

    let on_show_tree_text = {
        let messages_controller = messages_controller.clone();
        Callback::from(move |tree: Tree| {
            let window = web_sys::window().expect("global window does not exists");    

            messages_controller.fetch_tree_text(
                window.navigator().language().unwrap_or("english".to_string()),
                tree.name_sci.unwrap_or_default(),
                tree.neighbor.unwrap_or("Ciutat".to_string()) + &", Barcelona".to_string()
            );
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
        <div class="app">
            <TreeListPlayer
                waiting={messages.waiting}
                trees={messages.trees.clone()}
                selected_tree={messages.current_tree.clone()}
                selected_tree_text={messages.current_tree_text.clone()}
                on_select_tree={on_select_tree}
                on_show_more={on_show_more}
                messages={messages.messages.clone()}
                on_create_message={on_create_message}
                on_delete_message={on_delete_message}
                on_show_tree_text={on_show_tree_text}
            />

            <Geolocation on_coords_change={on_coords_change} />


        // <div class="container">
        //     // <AnimatedTree completion={proximity} />

        //     <Geolocation on_coords_change={on_coords_change} />

        //     if let Some(tree) = &messages.current_tree {
        //         <div class="selection">
        //             <span onclick={on_back_to_list}>{"⮪"}</span>
        //             <TreeInfos tree={tree.clone()} on_show_more={on_show_more} />
        //             <MessageForm current_tree_id={tree.tree_id.clone()} on_create_message={on_create_message} />

        //             <MessageList
        //                 messages={messages.messages.clone()}
        //                 on_delete_message={on_delete_message}
        //             />
        //         </div>

        //     } else if messages.trees.len() > 0 {
        //             <TreeList trees={messages.trees.clone()} on_select_tree={on_select_tree} />

        //     } else {
        //         <div ref={intro_node_ref} class="intro" onclick={on_intro_continue}>
        //             <SafeHtml html={random_intro()} />
        //         </div>
        //     }
        // </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
