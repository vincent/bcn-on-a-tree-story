use std::rc::Rc;

use yew::Reducible;

use crate::models::{Message, Tree};

pub enum Action {
    ShowTreeText(String),
    ShowMessages(Vec<Message>),
    HideMessages(),
    ListNearbyTrees(Vec<Tree>),
    UpdateProximity(i32),
    ChooseTree(Tree),
    ClearSelection(),
    // ListenTree(String),
    WriteMessageOnTree(String, Message),
    DeleteMessageFromTree(String, String),
}

pub struct MessageState {
    pub current_tree: Option<Tree>,
    pub trees: Vec<Tree>,
    pub current_tree_text: Option<String>,
    pub messages: Vec<Message>,
    pub proximity: i32,
    pub waiting: bool,
    pub inc: usize,
}

impl Default for MessageState {
    fn default() -> Self {
        Self {
            current_tree: None,
            messages: vec![],
            current_tree_text: None,
            trees: vec![],
            waiting: true,
            proximity: 0,
            inc: 0,
        }
    }
}

impl Reducible for MessageState {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut next_tree_text = None;
        let mut next_current_tree = self.current_tree.clone();
        let mut next_messages = self.messages.clone();
        let mut next_proximity = self.proximity;
        let mut next_trees = self.trees.clone();
        let mut next_waiting = self.waiting.clone();

        match action {
            Action::ShowMessages(messages) => {
                next_messages = messages;
            },
            Action::HideMessages() => {
                next_messages = vec![];
            },
            Action::WriteMessageOnTree(_tree_id, message) => {
                let mut messages = self.messages.clone();
                messages.push(message);
                next_messages = messages;
            }
            Action::DeleteMessageFromTree(_tree_id, message_id) => {
                let mut messages = self.messages.clone();
                messages.retain(|message| message.id != message_id);
                next_messages = messages;
            }
            Action::ListNearbyTrees(trees) => {
                if trees.len() == 1 {
                    next_current_tree = Some(trees.first().unwrap().clone());
                }
                next_trees = trees;
                next_waiting = false;
            }
            Action::UpdateProximity(proximity) => {
                next_proximity = proximity
            }
            Action::ChooseTree(tree) => {
                next_current_tree = Some(tree);
            }
            Action::ShowTreeText(text) => {
                next_tree_text = Some(text);
            }
            Action::ClearSelection() => {
                next_current_tree = None;
            }
        };

        Self {
            inc: (self.inc + 1) % 5,
            current_tree_text: next_tree_text,
            current_tree: next_current_tree,
            proximity: next_proximity,
            messages: next_messages,
            waiting: next_waiting,
            trees: next_trees,
        }.into()
    }
}
