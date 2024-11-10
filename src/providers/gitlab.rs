use async_trait::async_trait;
use percent_encoding::{percent_encode, NON_ALPHANUMERIC};
use reqwest::{header, Client};
use std::collections::HashMap;

use crate::error::ProviderError;
use crate::models::common::{IssueCreate, Label, Milestone, Project};
use crate::providers::ProviderConfig;
use crate::traits::repository::RepositoryProvider;

pub struct GitLabProvider {
    client: Client,
    api_url: String,
    project_id: String,
}

impl GitLabProvider {
    pub fn new(config: ProviderConfig) -> Result<Self, ProviderError> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "PRIVATE-TOKEN",
            header::HeaderValue::from_str(&config.token)
                .map_err(|e| ProviderError::Config(format!("Invalid token: {}", e)))?,
        );

        let client = Client::builder()
            .default_headers(headers)
            .build()
            .map_err(|e| ProviderError::Network(e.to_string()))?;

        Ok(Self {
            client,
            api_url: config.api_url,
            project_id: config.repository,
        })
    }

    fn encode_project_id(&self) -> String {
        percent_encode(self.project_id.as_bytes(), NON_ALPHANUMERIC).to_string()
    }
}

#[async_trait]
impl RepositoryProvider for GitLabProvider {
    async fn create_label(&self, label: &Label) -> Result<(), ProviderError> {
        let url = format!(
            "{}/projects/{}/labels",
            self.api_url,
            self.encode_project_id()
        );

        let response = self
            .client
            .post(&url)
            .json(&label)
            .send()
            .await
            .map_err(|e| ProviderError::Network(e.to_string()))?;

        if !response.status().is_success() {
            return Err(ProviderError::Api(format!(
                "Failed to create label: {}",
                response.status()
            )));
        }

        Ok(())
    }

    async fn create_issue(&self, issue: &IssueCreate) -> Result<(), ProviderError> {
        let url = format!(
            "{}/projects/{}/issues",
            self.api_url,
            self.encode_project_id()
        );

        let response = self
            .client
            .post(&url)
            .json(&issue)
            .send()
            .await
            .map_err(|e| ProviderError::Network(e.to_string()))?;

        if !response.status().is_success() {
            return Err(ProviderError::Api(format!(
                "Failed to create issue: {}",
                response.status()
            )));
        }

        Ok(())
    }

    async fn create_milestone(&self, milestone: &Milestone) -> Result<i64, ProviderError> {
        let url = format!(
            "{}/projects/{}/milestones",
            self.api_url,
            self.encode_project_id()
        );

        #[derive(serde::Serialize)]
        struct GitLabMilestone<'a> {
            title: &'a str,
            description: &'a str,
            due_date: &'a str,
        }

        let gitlab_milestone = GitLabMilestone {
            title: &milestone.name,
            description: &milestone.description,
            due_date: &milestone.deadline,
        };

        #[derive(serde::Deserialize)]
        struct MilestoneResponse {
            id: i64,
        }

        let response = self
            .client
            .post(&url)
            .json(&gitlab_milestone)
            .send()
            .await
            .map_err(|e| ProviderError::Network(e.to_string()))?;

        if !response.status().is_success() {
            return Err(ProviderError::Api(format!(
                "Failed to create milestone: {}",
                response.status()
            )));
        }

        let milestone_response = response
            .json::<MilestoneResponse>()
            .await
            .map_err(|e| ProviderError::Api(e.to_string()))?;

        Ok(milestone_response.id)
    }

    async fn create_issue_link(&self, from_id: i64, to_id: i64) -> Result<(), ProviderError> {
        let url = format!(
            "{}/projects/{}/issues/{}/links?target_issue_id={}",
            self.api_url,
            self.encode_project_id(),
            from_id,
            to_id
        );

        let response = self
            .client
            .post(&url)
            .send()
            .await
            .map_err(|e| ProviderError::Network(e.to_string()))?;

        if !response.status().is_success() {
            return Err(ProviderError::Api(format!(
                "Failed to create issue link: {}",
                response.status()
            )));
        }

        Ok(())
    }

    async fn setup_project(&self, project: &Project) -> Result<(), ProviderError> {
        let mut milestone_ids = HashMap::new();

        // Créer les milestones
        for milestone in &project.milestones {
            let id = self.create_milestone(milestone).await?;
            milestone_ids.insert(&milestone.version, id);
        }

        // Créer les issues avec leurs milestones
        let mut issue_ids = HashMap::new();
        for issue in &project.issues {
            let milestone_id = milestone_ids.get(&issue.milestone).ok_or_else(|| {
                ProviderError::NotFound(format!("Milestone not found: {}", issue.milestone))
            })?;

            let description = issue
                .description
                .sections
                .iter()
                .map(|s| format!("{}\n{}", s.title, s.content.join("\n")))
                .collect::<Vec<_>>()
                .join("\n\n");

            #[derive(serde::Serialize)]
            struct GitLabIssue<'a> {
                title: &'a str,
                description: String,
                milestone_id: i64,
                labels: &'a [String],
            }

            let url = format!(
                "{}/projects/{}/issues",
                self.api_url,
                self.encode_project_id()
            );

            let gitlab_issue = GitLabIssue {
                title: &issue.title,
                description,
                milestone_id: *milestone_id,
                labels: &issue.labels,
            };

            let response = self
                .client
                .post(&url)
                .json(&gitlab_issue)
                .send()
                .await
                .map_err(|e| ProviderError::Network(e.to_string()))?;

            if !response.status().is_success() {
                return Err(ProviderError::Api(format!(
                    "Failed to create issue: {}",
                    response.status()
                )));
            }

            #[derive(serde::Deserialize)]
            struct IssueResponse {
                iid: i64,
            }

            let issue_response = response
                .json::<IssueResponse>()
                .await
                .map_err(|e| ProviderError::Api(e.to_string()))?;

            issue_ids.insert(&issue.title, issue_response.iid);
        }

        // Créer les liens entre les issues
        for issue in &project.issues {
            if let Some(&from_id) = issue_ids.get(&issue.title) {
                for dep in &issue.dependencies {
                    if let Some(&to_id) = issue_ids.get(dep) {
                        self.create_issue_link(from_id, to_id).await?;
                    }
                }
            }
        }

        Ok(())
    }
}
