use crate::{
    state::{Action, MessageState},
    boats_api, models::Tree,
};
use web_sys::window;
use yew::UseReducerHandle;

pub struct MessageController {
    state: UseReducerHandle<MessageState>,
}

impl MessageController {
    pub fn new(state: UseReducerHandle<MessageState>) -> MessageController {
        MessageController { state }
    }

    pub fn list_trees(&self, lat: f64, long: f64) {
        let messages = self.state.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let trees = boats_api::fetch_trees(lat, long).await.unwrap();
            messages.dispatch(Action::ListNearbyTrees(trees.clone()));

            // if trees.len() == 1 {
            //     if let Some(tree) = trees.first() {
            //         messages.dispatch(Action::ChooseTree(tree.clone()));
            //         // self.init_messages(tree.clone());
            //     }
            // }
        })
    }

    pub fn update_proximity(&self, lat: f64, long: f64) {
        let messages = self.state.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let proximity = boats_api::closest(lat, long).await;
            messages.dispatch(Action::UpdateProximity(proximity));
        })
    }

    pub fn init_messages(&self, tree: Tree) {
        let messages = self.state.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let fetched_messages = boats_api::fetch_messages(&tree.tree_id).await.unwrap();
            messages.dispatch(Action::ShowMessages(fetched_messages));
            messages.dispatch(Action::ChooseTree(tree.clone()));
        })
    }

    pub fn create_message(&self, tree_id: String, title: String) {
        let messages = self.state.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let response = boats_api::create_message(&tree_id, &title).await.unwrap();
            messages.dispatch(Action::WriteMessageOnTree(tree_id.into(), response));
            messages.dispatch(Action::HideMessages())
        })
    }

    pub fn delete_message(&self, tree_id: String, id: String) {
        let messages = self.state.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let response = boats_api::delete_message(id.clone()).await.unwrap();
            if response.rows_affected == 1 {
                messages.dispatch(Action::DeleteMessageFromTree(tree_id, id.clone()))
            }
        })
    }

    pub fn clear_selection(&self) {
        let messages = self.state.clone();
        wasm_bindgen_futures::spawn_local(async move {
            messages.dispatch(Action::ClearSelection())
        })
    }

    pub fn open_search(&self, query: String) {
        let glg = String::from("https://www.google.com/search?q=");
        wasm_bindgen_futures::spawn_local(async move {
            if let Err(_e) = window().unwrap().open_with_url_and_target((glg + &query).as_str(), "_blank") {
                // 
            }
        })
    }

    pub fn fetch_tree_text(&self, lang: String, sci_name: String, nei_name: String) {
        let messages = self.state.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let text = boats_api::fetch_tree_text(&lang, &sci_name, &nei_name).await.unwrap();
            messages.dispatch(Action::ShowTreeText(text));
        })
    }
}
