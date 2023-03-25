use wasm_bindgen::{JsCast};
use web_sys::{MouseEvent, HtmlInputElement};
use yew::{function_component, html, Callback, Html, Properties, classes, use_node_ref, use_effect_with_deps};
use yew_hooks::prelude::*;

use crate::{models::{Tree, Message}, components::{MessageList, MessageForm}};

#[derive(Properties, PartialEq)]
pub struct TreeListProps {
    pub waiting: bool,
    pub trees: Vec<Tree>,
    pub selected_tree: Option<Tree>,
    pub on_select_tree: Callback<Tree>,
    pub messages: Vec<Message>,
    pub on_show_more: Callback<Tree>,
    pub on_create_message: Callback<(String, String)>,
    pub on_delete_message: Callback<(String, String)>,
}

#[function_component(TreeListPlayer)]
pub fn tree_list_player(
    TreeListProps {
        trees,
        waiting,
        selected_tree,
        messages,
        on_show_more,
        on_create_message,
        on_delete_message,
        on_select_tree,
    }: &TreeListProps,
) -> Html {
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

    let own_trees = trees.clone();
    let on_cb_click = {
        let on_select_tree = on_select_tree.clone();

        Callback::from(move |e: MouseEvent| {
            let trees = own_trees.clone();
            let selected_id = e.target().unwrap().dyn_ref::<HtmlInputElement>().unwrap().value();
            on_select_tree.emit(trees.into_iter().find(|t| t.tree_id == selected_id).expect("unknown tree").to_owned());
        })
    };

    let own_selected_tree = selected_tree.clone();
    let on_click_tree = {
        let on_select_tree = on_show_more.clone();
        move |_| {
            let tree = own_selected_tree.clone().unwrap();
            on_select_tree.emit(tree.clone())
        }
    };

    let (previous, current, next) = tree_positions(trees.clone(), selected_tree.clone());

    let trees_cbs: Html = trees.clone()
        .iter()
        .map(|tree| {
            let position_class = match tree.id.to_owned() {
                x if x == previous => "previous",
                x if x == current => "current",
                x if x == next => "next",
                _ => ""
            };
            html!(
                <input type="radio" name="slider" 
                    class={classes!(position_class, "song-info")}
                    id={tree.tree_id.clone()} value={tree.tree_id.clone()}
                    checked={current == tree.id.to_owned()}
                    onclick={on_cb_click.clone()} />
            )
        })
        .collect();

    let trees_infos: Html = trees.clone()
        .iter()
        .map(|tree| {
            let position_class = match tree.id.to_owned() {
                x if x == previous => "previous",
                x if x == current => "current",
                x if x == next => "next",
                _ => ""
            };
            html!(
                <label class={classes!(position_class, "song-info")}>
                    <div class="title">{tree.name_cat.clone()}</div>
                    <div class="sub-line">
                        <div class="subtitle">
                            if tree.name_es != tree.name_cat { {tree.name_es.clone()} }
                        </div>
                        <div class="time" onclick={on_click_tree.clone()}>{tree.name_sci.clone()}</div>
                    </div>
                </label>
            )
        })
        .collect();

    let trees_cards: Html = trees.clone()
        .iter()
        .map(|tree| {
            let position_class = match tree.id.to_owned() {
                x if x == previous => "previous",
                x if x == current => "current",
                x if x == next => "next",
                _ => ""
            };
            html!(
                <label class={classes!(position_class, "card")} for={tree.tree_id.clone()} id={"song-".to_owned() + &tree.tree_id.clone()}>
                    <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/c/ca/London_Plane_Whole.jpg/802px-London_Plane_Whole.jpg" alt="Tree" />
                </label>
            )
        })
        .collect();

    html!(
        <div class="container">
            {trees_cbs}

            <div class="cards" ref={cards}>
                {trees_cards}
            </div>

            <div class="player">
                <div class="upper-part">
                    if waiting.to_owned() {
                        {"Waiting for geolocation"}<br />
                        {"Looking for trees around you ..."}

                    } else {
                        <div class="info-area">
                            {trees_infos}
                        </div>
                    }
                </div>

                if let Some(tree) = selected_tree {
                    <div class="lower-part">
                        <MessageForm current_tree_id={tree.tree_id.clone()} on_create_message={on_create_message} />

                        <MessageList
                            messages={messages.clone()}
                            on_delete_message={on_delete_message}
                        />
                    </div>                    
                }

                // <div class="progress-bar">
                // <span class="progress"></span>
                // </div>
            </div>
        </div>
    )
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

pub fn icon_play() -> Html {
    html!(
        <div class="play-icon">
            <svg
                width="20" height="20" fill="#2992dc" stroke="#2992dc" stroke-linecap="round" 
                stroke-linejoin="round" stroke-width="2" 
                class="feather feather-play" viewBox="0 0 24 24">
                <defs/>
                <path d="M5 3l14 9-14 9V3z"/>
            </svg>
        </div>
    )
}