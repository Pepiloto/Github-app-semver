use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Payload {
    #[serde(rename = "ref")]
    payload_ref: String,
    before: String,
    after: String,
    repository: crate::github::repository::Repository,
    pusher: crate::github::pusher::Pusher,
    organization: crate::github::organisation::Organization,
    enterprise: crate::github::enterprise::Enterprise,
    sender: crate::github::sender::Sender,
    installation: Installation,
    created: bool,
    deleted: bool,
    forced: bool,
    base_ref: Option<serde_json::Value>,
    compare: String,
    commits: Vec<crate::github::commit::Commit>,
    head_commit: Option<crate::github::commit::HeadCommit>,
}

impl Payload {
    pub fn head_commit(&self) -> &Option<crate::github::commit::HeadCommit> {
        &self.head_commit
    }

    pub fn repository(&self) -> &crate::github::repository::Repository {
        &self.repository
    }

    pub fn pusher(&self) -> &crate::github::pusher::Pusher {
        &self.pusher
    }

    pub fn payload_ref(&self) -> &str {
        &self.payload_ref
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Installation {
    id: i64,
    node_id: String,
}
