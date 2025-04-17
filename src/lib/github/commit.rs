use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Commit {
    id: String,
    tree_id: String,
    distinct: bool,
    message: String,
    timestamp: String,
    url: String,
    author: crate::github::author::Author,
    committer: crate::github::author::Author,
    added: Vec<Option<serde_json::Value>>,
    removed: Vec<Option<serde_json::Value>>,
    modified: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HeadCommit {
    id: String,
    tree_id: String,
    distinct: bool,
    message: String,
    timestamp: String,
    url: String,
    author: crate::github::author::Author,
    committer: crate::github::pusher::Pusher,
    added: Vec<Option<serde_json::Value>>,
    removed: Vec<Option<serde_json::Value>>,
    modified: Vec<String>,
}

impl HeadCommit {
    pub fn committer(&self) -> &crate::github::pusher::Pusher {
        &self.committer
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}
