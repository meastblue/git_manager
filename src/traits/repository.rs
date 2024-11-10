use crate::error::ProviderError;
use crate::models::common::{IssueCreate, Label, Milestone, Project};
use async_trait::async_trait;

#[async_trait]
pub trait RepositoryProvider: Send + Sync {
    /// Crée un nouveau label dans le repository
    async fn create_label(&self, label: &Label) -> Result<(), ProviderError>;

    /// Crée une nouvelle issue
    async fn create_issue(&self, issue: &IssueCreate) -> Result<(), ProviderError>;

    /// Crée un nouveau milestone et retourne son ID
    async fn create_milestone(&self, milestone: &Milestone) -> Result<i64, ProviderError>;

    /// Crée un lien entre deux issues
    async fn create_issue_link(&self, from_id: i64, to_id: i64) -> Result<(), ProviderError>;

    /// Configure un projet complet avec milestones, issues et leurs relations
    async fn setup_project(&self, project: &Project) -> Result<(), ProviderError>;
}

pub type DynProvider = Box<dyn RepositoryProvider>;
