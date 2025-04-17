use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Enterprise {
    id: i64,
    slug: String,
    name: String,
    node_id: String,
    avatar_url: String,
    description: String,
    website_url: String,
    html_url: String,
    created_at: String,
    updated_at: String,
}
