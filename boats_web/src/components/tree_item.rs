use yew::{classes, function_component, html, Callback, Properties, Html};

use crate::models::Tree;

#[derive(Properties, PartialEq)]
pub struct TreeItemProps {
    pub tree: Tree,
    pub on_select_tree: Callback<Tree>,
}

#[function_component(TreeItem)]
pub fn tree(
    TreeItemProps {
        tree,
        on_select_tree,
    }: &TreeItemProps,
) -> Html {
    let on_click_tree = {
        let tree = tree.clone();
        let on_select_tree = on_select_tree.clone();
        move |_| on_select_tree.emit(tree.clone())
    };

    let unknown = String::from("Unknown specie");
    let name_sci = tree.name_sci.as_ref().unwrap_or(&unknown);
    let name_cat = tree.name_cat.as_ref().unwrap_or(&unknown);
    let name_es  =  tree.name_es.as_ref().unwrap_or(&unknown);

    html! {
        <li class={classes!("center", "tree-item")} onclick={on_click_tree}>
            <label>
                <b>{name_es}</b> {" / "}<b>{name_cat}</b>
                <br/><i>{name_sci}</i>
            </label>
        </li>
    }
}
