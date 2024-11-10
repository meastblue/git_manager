mod github;
mod gitlab;

pub use self::github::GitHubProvider;
pub use self::gitlab::GitLabProvider;

use crate::error::ProviderError;
use crate::traits::repository::DynProvider;

#[derive(Debug, Clone, Copy)]
pub enum ProviderType {
    GitHub,
    GitLab,
}

#[derive(Debug, Clone)]
pub struct ProviderConfig {
    pub api_url: String,
    pub token: String,
    pub repository: String,
}

pub fn create_provider(
    provider_type: ProviderType,
    config: ProviderConfig,
) -> Result<DynProvider, ProviderError> {
    match provider_type {
        ProviderType::GitHub => {
            println!(
                "Creating GitHub provider for repository: {}",
                config.repository
            );
            let provider = GitHubProvider::new(config)?;
            Ok(Box::new(provider))
        }
        ProviderType::GitLab => {
            println!(
                "Creating GitLab provider for repository: {}",
                config.repository
            );
            let provider = GitLabProvider::new(config)?;
            Ok(Box::new(provider))
        }
    }
}
