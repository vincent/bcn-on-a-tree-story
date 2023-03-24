use std::rc::Rc;

use yew::Reducible;

use crate::models::{Message, Tree};

pub enum Action {
    ShowMessages(Vec<Message>),
    HideMessages(),
    ListNearbyTrees(Vec<Tree>),
    ChooseTree(Tree),
    ClearSelection(),
    ListenTree(String),
    WriteMessageOnTree(String, Message),
    DeleteMessageFromTree(String, String),
}

pub struct MessageState {
    pub current_tree: Option<Tree>,
    pub trees: Vec<Tree>,
    pub messages: Vec<Message>,
}

impl Default for MessageState {
    fn default() -> Self {
        Self {
            current_tree: None,
            messages: vec![],
            trees: vec![]
        }
    }
}

impl Reducible for MessageState {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let next_current_tree;
        let next_messages;
        let next_trees;

        match action {
            Action::ShowMessages(messages) => {
                next_current_tree = self.current_tree.clone();
                next_messages = messages;
                next_trees = self.trees.clone();
            },
            Action::HideMessages() => {
                next_current_tree = self.current_tree.clone();
                next_messages = vec![];
                next_trees = self.trees.clone();
            },
            Action::WriteMessageOnTree(tree_id, message) => {
                let mut messages = self.messages.clone();
                messages.push(message);

                next_current_tree = self.current_tree.clone();
                next_messages = messages;
                next_trees = self.trees.clone();
            }
            Action::DeleteMessageFromTree(tree_id, message_id) => {
                let mut messages = self.messages.clone();
                messages.retain(|message| message.id != message_id);

                next_current_tree = self.current_tree.clone();
                next_messages = messages;
                next_trees = self.trees.clone();
            }
            Action::ListNearbyTrees(trees) => {
                if trees.len() == 1 {
                    next_current_tree = Some(trees.first().unwrap().clone());
                } else {
                    next_current_tree = self.current_tree.clone();
                }
                next_messages = self.messages.clone();
                next_trees = trees;
            }
            Action::ChooseTree(tree) => {
                next_current_tree = Some(tree);
                next_messages = self.messages.clone();
                next_trees = self.trees.clone();
            }
            Action::ClearSelection() => {
                next_current_tree = None;
                next_messages = self.messages.clone();
                next_trees = self.trees.clone();
            }

            _ => {
                next_current_tree = self.current_tree.clone();
                next_messages = self.messages.clone();
                next_trees = self.trees.clone();
            }
        };

        Self {
            current_tree: next_current_tree,
            messages: next_messages,
            trees: next_trees,
        }.into()
    }
}
