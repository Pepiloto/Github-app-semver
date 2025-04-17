pub struct GithubAppConfig {
    pub app_id: String,
    pub private_key: String,
    pub hostname: String,
}

impl GithubAppConfig {
    pub fn new(app_id: &str, private_key: &str, hostname: &str) -> Self {
        Self {
            app_id: app_id.to_string(),
            private_key: private_key.to_string(),
            hostname: hostname.to_string(),
        }
    }
}
