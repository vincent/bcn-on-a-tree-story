use yew::{function_component, html, Callback, Html, Properties};

use super::TreeItem;
use crate::models::{Tree};

#[derive(Properties, PartialEq)]
pub struct TreeListProps {
    pub trees: Vec<Tree>,
    pub on_select_tree: Callback<Tree>,
}

#[function_component(TreeList)]
pub fn message_list(
    TreeListProps {
        trees,
        on_select_tree,
    }: &TreeListProps,
) -> Html {
    let trees_count = trees.len();
    let trees: Html = trees
        .iter()
        .map(|tree| html!( <TreeItem tree={tree.clone()} on_select_tree={on_select_tree} /> ))
        .collect();

    html!(
        <ul id="message-list">
            if trees_count == 1 {
                <li>{ "Nearby, there is a tree" }</li>
            }
            if trees_count > 1 {
                <li>{ "Nearby, there are some trees" }</li>
            }

            {trees}
        </ul>
    )
}
