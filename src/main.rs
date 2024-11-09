// src/main.rs
mod config;
mod error;
mod gitlab;

use clap::Parser;
use config::Config;
use error::AppResult;
use gitlab::GitLabClient;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// GitLab API URL (e.g., https://gitlab.com/api/v4)
    #[arg(long, env = "GITLAB_API_URL")]
    api_url: String,

    /// GitLab API Token
    #[arg(long, env = "GITLAB_API_TOKEN")]
    token: String,

    /// Project ID or path (e.g., username/project-name)
    #[arg(long, env = "GITLAB_PROJECT_ID")]
    project_id: String,

    /// Path to labels configuration file
    #[arg(long, default_value = "labels.json")]
    config: String,
}

#[tokio::main]
async fn main() -> AppResult<()> {
    dotenv::dotenv().ok();
    let cli = Cli::parse();

    // Charger la configuration
    let config = Config::from_file(&cli.config)?;

    // Initialiser le client GitLab
    let gitlab = GitLabClient::new(cli.api_url, cli.token, cli.project_id)?;

    // Créer les labels
    for label in config.labels {
        match gitlab.create_label(&label).await {
            Ok(_) => println!("✅ Created label: {}", label.name),
            Err(e) => eprintln!("❌ Failed to create label {}: {:?}", label.name, e),
        }
    }

    Ok(())
}
