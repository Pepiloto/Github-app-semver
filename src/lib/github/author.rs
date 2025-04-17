use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Author {
    name: String,
    email: String,
    username: Option<String>,
}
