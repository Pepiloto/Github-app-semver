use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Sender {
    name: Option<String>,
    email: Option<serde_json::Value>,
    login: String,
    id: i64,
    node_id: String,
    avatar_url: String,
    gravatar_id: String,
    url: String,
    html_url: String,
    followers_url: String,
    following_url: String,
    gists_url: String,
    starred_url: String,
    subscriptions_url: String,
    organizations_url: String,
    repos_url: String,
    events_url: String,
    received_events_url: String,
    #[serde(rename = "type")]
    sender_type: String,
    site_admin: bool,
}

impl Sender {
    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }
}
