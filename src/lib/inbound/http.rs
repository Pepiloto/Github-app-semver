use crate::github::app::GithubAppConfig;

pub mod handlers;

/// The configuration of the HTTP server
#[derive(Debug, Clone)]
pub struct HttpServerConfig {
    pub port: String,
    // TODO: Add support for secret verification
    webhook_secret: Option<String>,
}

impl Default for HttpServerConfig {
    fn default() -> Self {
        Self {
            port: "8080".to_string(),
            webhook_secret: None,
        }
    }
}

impl HttpServerConfig {
    /// Create a new `HttpServer` instance
    pub fn new(port: &str, webhook_secret: Option<&str>) -> Self {
        Self {
            port: port.to_string(),
            webhook_secret: webhook_secret.map(|s| s.to_string()),
        }
    }
}

/// The global application state shared between all requests handlers
#[derive(Debug, Clone)]
pub struct AppState {
    github_app: octocrab::Octocrab,
}

/// The application's HTTP server
pub struct HttpServer {
    pub router: axum::Router,
    listener: tokio::net::TcpListener,
}

impl HttpServer {
    pub async fn new(
        router_config: HttpServerConfig,
        github_app_config: crate::github::app::GithubAppConfig,
    ) -> Result<Self, String> {
        let shared_state = std::sync::Arc::new(AppState {
            github_app: Self::create_github_app(github_app_config).await,
        });
        let router: axum::Router = axum::Router::new()
            .route(
                "/api/webhook",
                axum::routing::post(handlers::webhook_handler),
            )
            .with_state(shared_state);
        let listener = tokio::net::TcpListener::bind("0.0.0.0:".to_string() + &router_config.port)
            .await
            .map_err(|error| format!("Failed to bind to port: {:?}", error))?;
        Ok(Self { router, listener })
    }

    pub async fn run(self) -> Result<(), String> {
        axum::serve(self.listener, self.router)
            .await
            .map_err(|error| format!("Failed to run the server: {}", error))?;
        Ok(())
    }

    async fn create_github_app(github_app_config: GithubAppConfig) -> octocrab::Octocrab {
        // TODO: Add a way to handle the uri for self-hosted Github
        let octocrab = octocrab::Octocrab::builder() //.base_uri(format!("https://{}/api/v3", github_app_config.hostname)).unwrap()
            .app(
                octocrab::models::AppId(
                    github_app_config
                        .app_id
                        .parse::<u64>()
                        .expect("The APP_ID is not a valid number"),
                ),
                jsonwebtoken::EncodingKey::from_rsa_pem(github_app_config.private_key.as_bytes())
                    .expect("Invalid private key"),
            )
            .build()
            .expect("Error creating GitHub App");
        let github_app = octocrab
            .apps()
            .installations()
            .send()
            .await
            .expect("Github App installation failed");
        let github_app =
            octocrab.installation(github_app.items.first().expect("App is not installed").id);
        github_app.expect("Creation of the GitHub app failed")
    }
}
