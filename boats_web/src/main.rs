use std::rc::Rc;
use web_sys::HtmlElement;
use wasm_bindgen::{JsCast};
use yew::prelude::*;
use yew_hooks::prelude::*;

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

    let (previous, current, next) = tree_positions(messages.trees.clone(), messages.current_tree.clone());
    // let proximity = 10000 / messages.proximity.max(1);

    let cards = use_node_ref();

    let state = use_swipe(cards.clone());
    {
        let state = state.clone();
        use_effect_with_deps(move |direction| {
            // Do something based on direction.
            match **direction {
                UseSwipeDirection::Left => (),
                UseSwipeDirection::Right => (),
                UseSwipeDirection::Up => (),
                UseSwipeDirection::Down => (),
                _ => (),
            }
            || ()
        }, state.direction);
    }

    // let on_create_message = {
    //     let messages_controller = messages_controller.clone();
    //     Callback::from(move |(tree_id, title): (String, String)| {
    //         messages_controller.create_message(tree_id.clone(), title);
    //     })
    // };

    // let on_delete_message = {
    //     let messages_controller = messages_controller.clone();
    //     Callback::from(move |(tree_id, id): (String, String)| {
    //         messages_controller.delete_message(tree_id.clone(), id);
    //     })
    // };

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

        Callback::from(move |e: MouseEvent| {
            let target = e.target()
                .expect("expecting the event target");
            let target = target
                .dyn_ref::<HtmlElement>()
                .expect("expecting an element");
            let tree_id = target
                .clone()
                .get_attribute("for")
                .unwrap_or("none".to_string());

            log::info!("clicked on {} select tree [{}]", target.tag_name(), tree_id);

            if let Some(tree) = messages.trees.clone().iter().find(|t| t.tree_id == tree_id) {
                let window = web_sys::window().expect("global window does not exists");    
                let document = window.document().expect("expecting a document on window");
                let body = document.body().expect("expecting a body on document");

                let color = vec![ "bg1", "bg2", "bg3", "bg4", "bg5" ][messages.inc];
                body.set_class_name(color);

                log::info!("selected tree [{}]", tree.clone().name_sci.unwrap_or("default".to_string()).to_owned());
                messages_controller.init_messages(tree.to_owned());
            }
        })
    };

    let on_show_more = {
        let messages_controller = messages_controller.clone();
        let current_tree = messages.current_tree.clone();

        Callback::from(move |_e: MouseEvent| {
            if let Some(tree) = current_tree.clone() {
                messages_controller.open_search(tree.name_sci.unwrap_or_default());
            }
        })
    };

    let on_show_tree_text = {
        let messages_controller = messages_controller.clone();
        let current_tree = messages.current_tree.clone();

        Callback::from(move |_e: MouseEvent| {
            if let Some(tree) = current_tree.clone() {
                let window = web_sys::window().expect("global window does not exists");    

                messages_controller.fetch_tree_text(
                    window.navigator().language().unwrap_or("english".to_string()),
                    tree.tree_id,
                    tree.name_sci.unwrap_or_default(),
                    tree.neighbor.unwrap_or("Ciutat".to_string()) + &", Barcelona".to_string()
                );
            }
        })
    };

    let on_delete = {
        let messages_controller = messages_controller.clone();
        let current_tree = messages.current_tree.clone();

        Callback::from(move |_e: MouseEvent| {
            if let Some(tree) = current_tree.clone() {
                let window = web_sys::window().expect("global window does not exists");    

                messages_controller.delete_tree_media(tree);
            }
        })
    };

    let trees_cards: Html = messages.trees.clone()
        .iter()
        .map(|tree| {
            let position_class = match tree.id.to_owned() {
                x if x == previous => "previous",
                x if x == current => "current",
                x if x == next => "next",
                _ => ""
            };
            html!(
                <label class={classes!(position_class, "card")} for={tree.tree_id.clone()} onclick={on_select_tree.clone()}>
                    <img src={"/api/img/".to_owned() + tree.name_sci.clone().unwrap().as_mut_str()} for={tree.tree_id.clone()} />
                </label>
            )
        })
        .collect();

    let mut tree_infos: Html = html!(<div></div>);
    if let Some(tree) = messages.current_tree.clone() {
        log::info!("show selected tree [{}]", tree.clone().name_sci.unwrap_or("default".to_string()).to_owned());

        let tree_name = vec![
            tree.name_cat.clone(),
            tree.name_es.clone(),
            tree.name_sci.clone(),
            Some("Unknown tree".to_string()),
        ]
        .iter()
        .find(|n| n.is_some() && !n.as_ref().unwrap().is_empty())
        .unwrap()
        .clone();

        tree_infos = html!(
            <label class="song-info">
                <div class="title">
                    <div class="play-icon" onclick={on_show_tree_text}>
                        {icon_play()}
                        {tree_name.clone()}
                    </div>
                    <div class="actions">
                        <span onclick={on_delete}>{"x"}</span>
                    </div>
                </div>
                <div class="sub-line">
                    if tree.name_es != tree_name {
                        <div class="subtitle">
                            {tree.name_es.clone()}
                        </div>
                    }
                    if tree.name_sci != tree_name {
                        <div class="time" onclick={on_show_more}>
                            {tree.name_sci.clone()}
                        </div>
                    }
                </div>
            </label>
        )
    }
    
    html! {
        <div class="app">

            <div class="container">
                <div class="cards" ref={cards}>
                    {trees_cards}
                </div>
            </div>

            <div class="player">
                <div class="upper-part">
                    if messages.waiting.to_owned() {
                        <div class="song-info">
                            <div class="title">
                                {"Waiting for geolocation"}<br />
                                {"Looking for trees around you ..."}
                            </div>
                        </div>

                    } else if messages.trees.len() < 1 {
                        {"Come closer ..."}

                    } else {
                        <div class="tree-info-area">
                            {tree_infos}
                        </div>
                    }
                </div>

                if let Some(tree) = messages.current_tree.clone() {
                    <div class="lower-part">
                        if let Some(text) = messages.current_tree_text.clone() {
                            <div class="tree-text">
                               <SafeHtml html={text.replace("\n", "<br/>")} />
                            </div>
                        }
                    </div>                    
                }
            </div>

            <Geolocation on_coords_change={on_coords_change} />
        </div>
    }
}

fn tree_positions(trees: Vec<Tree>, selected: Option<Tree>) -> (String, String, String) {
    let count = trees.len();
    let mut previous: String = "none".to_owned();
    let mut current: String = "none".to_owned();
    let mut next: String = "none".to_owned();

    if count == 0 {
        //

    } else if let Some(selected) = selected {
        let trees_copy = trees.clone();
        for (i, c) in trees.into_iter().enumerate() {
            if c.id == selected.id {
                current = c.id;
                if i < count - 1 {next = trees_copy[i + 1].id.clone()};
                if i > 0 {previous = trees_copy[i - 1].id.clone()};
            }
        }
    } else {
        current = trees[0].id.clone();
        if count > 1 {
            next = trees[1].id.clone();
        }
        if count > 2 {
            previous = trees[count - 1].id.clone();
        }
    }

    (previous, current, next)
}

fn icon_play() -> Html {
    html!(
        <svg
            width="50" height="50" fill="#2992dc" stroke="#2992dc" stroke-linecap="round" 
            stroke-linejoin="round" stroke-width="2" 
            class="feather feather-play" viewBox="0 0 24 24">
            <defs/>
            <path d="M5 3l14 9-14 9V3z"/>
        </svg>
    )
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
