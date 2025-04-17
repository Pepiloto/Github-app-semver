#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let server_config = github_app_lib::inbound::http::HttpServerConfig::new("8080", None);
    let github_app_config = github_app_lib::github::app::GithubAppConfig::new(
        &std::env::var("APP_ID").expect("APP_ID environment variable not set"),
        &std::env::var("PRIVATE_KEY").expect("PRIVATE_KEY environment variable not set"),
        &std::env::var("HOSTNAME").expect("HOSTNAME environment variable not set"),
    );
    let http_server =
        github_app_lib::inbound::http::HttpServer::new(server_config, github_app_config)
            .await
            .expect("Failed to create the server");
    let _ = http_server.run().await;
}
