use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Pusher {
    name: String,
    email: String,
}

impl Pusher {
    pub fn name(&self) -> &str {
        &self.name
    }
}
