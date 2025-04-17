# Semver Rust Github App

This app allow the update following [semantic versioning](http://semver.org/) for Typescript and Java programs.

Code uses [octocrab](https://github.com/XAMPPRocky/octocrab).

## Requirements

- Rust installed
- A GitHub App subscribed to **Push** events and with the following permissions:
  - Content: Read & write
  - Metadata: Read-only
- (For local development) A tunnel to expose your local server to the internet (e.g. [smee](https://smee.io/), [ngrok](https://ngrok.com/) or [cloudflared](https://developers.cloudflare.com/cloudflare-one/connections/connect-apps/install-and-setup/tunnel-guide/local/))
- Your GitHub App Webhook must be configured to receive events at a URL that is accessible from the internet.

- ## Setup

1. Clone this repository.
2. Create a `.env` file similar to `.env.example` and set actual values. If you are using GitHub Enterprise Server, also include a `ENTERPRISE_HOSTNAME` variable and set the value to the name of your GitHub Enterprise Server instance.
3. Install dependencies with `cargo build`.
4. Start the server with `cargo run`.
5. Ensure your server is reachable from the internet.
    - If you're using `ngrok`, run `ngrok http 8080` once the initial setup is done.
6. Ensure your GitHub App includes at least one repository on its installations.

## Usage

Once the server is running, it will listen for events from the GitHub repositories you installed the app on.

When a push is made on a repository where the app is installed, the app will receive a webhook event containing information about the push.
If it matches the requirements to update a version (MAJOR, MINOR, PATCH) it will be done automatically.
