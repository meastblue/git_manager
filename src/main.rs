mod error;
mod models;
mod providers;
mod traits;

use clap::{Parser, Subcommand, ValueEnum};
use error::ProviderError;
use models::common::{Issue, IssueCreate, ProjectFile};
use models::config::Config;
use providers::{create_provider, ProviderConfig, ProviderType};
use std::path::PathBuf;

#[derive(Debug, Copy, Clone, ValueEnum)]
enum Provider {
    #[value(name = "github")]
    GitHub,
    #[value(name = "gitlab")]
    GitLab,
}

impl From<Provider> for ProviderType {
    fn from(provider: Provider) -> Self {
        match provider {
            Provider::GitHub => ProviderType::GitHub,
            Provider::GitLab => ProviderType::GitLab,
        }
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Provider type (github or gitlab)
    #[arg(long, value_enum)]
    provider: Provider,

    /// API URL (e.g., https://api.github.com or https://gitlab.com/api/v4)
    #[arg(long, env = "REPO_API_URL")]
    api_url: String,

    /// Authentication token
    #[arg(long, env = "REPO_TOKEN")]
    token: String,

    /// Repository identifier (e.g., "owner/repo" for GitHub or "group/project" for GitLab)
    #[arg(long, env = "REPO_PATH")]
    repository: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create repository labels
    Labels {
        /// Path to labels configuration file
        #[arg(long, default_value = "labels.json")]
        config: PathBuf,
    },
    /// Create individual issues
    Issues {
        /// Path to issues configuration file
        #[arg(long, default_value = "tasks.json")]
        tasks: PathBuf,
    },
    /// Setup complete project (milestones, issues, and links)
    Setup {
        /// Path to project configuration file
        #[arg(long, default_value = "project.json")]
        config: PathBuf,
    },
}

#[tokio::main]
async fn main() -> Result<(), ProviderError> {
    // Charger les variables d'environnement
    dotenv::dotenv().ok();

    // Parser les arguments
    let cli = Cli::parse();

    println!(
        "Using {} provider",
        match cli.provider {
            Provider::GitHub => "GitHub",
            Provider::GitLab => "GitLab",
        }
    );

    // Créer la configuration du provider
    let config = ProviderConfig {
        api_url: cli.api_url,
        token: cli.token,
        repository: cli.repository,
    };

    // Créer le provider
    let provider = create_provider(cli.provider.into(), config)?;

    // Exécuter la commande appropriée
    match cli.command {
        Commands::Labels { config } => {
            println!("Loading labels from: {}", config.display());
            let config = Config::from_file(config.to_str().unwrap())?;

            for label in config.labels {
                match provider.create_label(&label).await {
                    Ok(_) => println!("✅ Created label: {}", label.name),
                    Err(e) => eprintln!("❌ Failed to create label {}: {:?}", label.name, e),
                }
            }
        }
        Commands::Issues { tasks } => {
            println!("Loading issues from: {}", tasks.display());
            let content = std::fs::read_to_string(tasks)?;
            let issues: Vec<Issue> = serde_json::from_str(&content)?;

            for issue in issues {
                let description = issue.description.to_markdown();
                let create_issue = IssueCreate {
                    title: issue.title.clone(),
                    description,
                    labels: issue.labels,
                };

                match provider.create_issue(&create_issue).await {
                    Ok(_) => println!("✅ Created issue: {}", create_issue.title),
                    Err(e) => {
                        eprintln!("❌ Failed to create issue {}: {:?}", create_issue.title, e)
                    }
                }
            }
        }
        Commands::Setup { config } => {
            println!("Loading project from: {}", config.display());
            let content = std::fs::read_to_string(config)?;
            let project_file: ProjectFile = serde_json::from_str(&content)?;

            match provider.setup_project(&project_file.project).await {
                Ok(_) => println!("✅ Project setup completed successfully!"),
                Err(e) => eprintln!("❌ Failed to setup project: {:?}", e),
            }
        }
    }

    Ok(())
}
