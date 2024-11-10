use reqwest::{header, Client};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::error::ProviderError;
use crate::models::common::{IssueCreate, Label, Milestone, Project};
use crate::providers::ProviderConfig;
use crate::traits::repository::RepositoryProvider;

const API_VERSION: &str = "2022-11-28";

#[derive(Debug, Serialize)]
struct GitHubLabel<'a> {
    name: &'a str,
    color: String, // Sans le #
    description: Option<&'a str>,
}

#[derive(Debug, Serialize)]
struct GitHubMilestone<'a> {
    title: &'a str,
    description: &'a str,
    #[serde(rename = "due_on")]
    due_on: &'a str,
    state: &'a str,
}

#[derive(Debug, Deserialize)]
struct GitHubMilestoneResponse {
    number: i64, // GitHub utilise 'number' plutôt que 'id'
}

#[derive(Debug, Serialize)]
struct GitHubIssue {
    title: String,
    body: String,
    milestone: Option<i64>,
    labels: Vec<String>,
    assignees: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct GitHubIssueResponse {
    number: i64,
}

pub struct GitHubProvider {
    client: Client,
    api_url: String,
    repo: String,
}

impl GitHubProvider {
    pub fn new(config: ProviderConfig) -> Result<Self, ProviderError> {
        let mut headers = header::HeaderMap::new();

        // Authorization header
        let auth_value = format!("Bearer {}", config.token);
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&auth_value)
                .map_err(|e| ProviderError::Config(format!("Invalid token: {}", e)))?,
        );

        // API Version header
        headers.insert(
            "X-GitHub-Api-Version",
            header::HeaderValue::from_static(API_VERSION),
        );

        // Accept header
        headers.insert(
            header::ACCEPT,
            header::HeaderValue::from_static("application/vnd.github+json"),
        );

        // User-Agent header
        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_static("gitlab_label_manager/0.1.0"), // Remplace avec le nom de ton application et version
        );

        let client = Client::builder()
            .default_headers(headers)
            .build()
            .map_err(|e| ProviderError::Network(e.to_string()))?;

        Ok(Self {
            client,
            api_url: config.api_url,
            repo: config.repository,
        })
    }

    fn format_date(due_on: &str) -> String {
        format!("{}T00:00:00Z", due_on)
    }

    fn strip_hash_from_color(color: &str) -> String {
        color.trim_start_matches('#').to_string()
    }
}

#[async_trait::async_trait]
impl RepositoryProvider for GitHubProvider {
    async fn create_label(&self, label: &Label) -> Result<(), ProviderError> {
        let url = format!("{}/repos/{}/labels", self.api_url, self.repo,);

        let github_label = GitHubLabel {
            name: &label.name,
            color: Self::strip_hash_from_color(&label.color),
            description: label.description.as_deref(),
        };

        let response = self
            .client
            .post(&url)
            .json(&github_label)
            .send()
            .await
            .map_err(|e| ProviderError::Network(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_body = response
                .text()
                .await
                .unwrap_or_else(|_| String::from("Unable to read error response"));

            return Err(ProviderError::Api(format!(
                "Failed to create label. Status: {}, Body: {}",
                status, error_body
            )));
        }

        Ok(())
    }

    async fn create_milestone(&self, milestone: &Milestone) -> Result<i64, ProviderError> {
        let url = format!("{}/repos/{}/milestones", self.api_url, self.repo,);

        let github_milestone = GitHubMilestone {
            title: &milestone.name,
            description: &milestone.description,
            // due_on: &milestone.deadline,
            due_on: &Self::format_date(&milestone.deadline),
            state: "open",
        };

        let response = self
            .client
            .post(&url)
            .json(&github_milestone)
            .send()
            .await
            .map_err(|e| ProviderError::Network(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status(); // Récupérer le statut avant d'appeler `.text()`
            let error_body = response
                .text()
                .await
                .unwrap_or_else(|_| String::from("Unable to read error response"));

            return Err(ProviderError::Api(format!(
                "Failed to create label. Status: {}, Body: {}",
                status, // Utiliser `status` ici
                error_body
            )));
        }

        let milestone_response = response
            .json::<GitHubMilestoneResponse>()
            .await
            .map_err(|e| {
                ProviderError::Api(format!("Failed to parse milestone response: {}", e))
            })?;

        Ok(milestone_response.number)
    }

    async fn create_issue(&self, issue: &IssueCreate) -> Result<(), ProviderError> {
        let url = format!("{}/repos/{}/issues", self.api_url, self.repo,);

        let github_issue = GitHubIssue {
            title: issue.title.clone(),
            body: issue.description.clone(),
            milestone: None,
            labels: issue.labels.clone(),
            assignees: Vec::new(), // Optionnel, pourrait être ajouté plus tard
        };

        let response = self
            .client
            .post(&url)
            .json(&github_issue)
            .send()
            .await
            .map_err(|e| ProviderError::Network(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status(); // Récupérer le statut avant d'appeler `.text()`
            let error_body = response
                .text()
                .await
                .unwrap_or_else(|_| String::from("Unable to read error response"));

            return Err(ProviderError::Api(format!(
                "Failed to create label. Status: {}, Body: {}",
                status, // Utiliser `status` ici
                error_body
            )));
        }

        Ok(())
    }

    async fn create_issue_link(&self, from_id: i64, to_id: i64) -> Result<(), ProviderError> {
        // GitHub n'a pas d'API native pour les liens entre issues
        // On ajoute un commentaire pour montrer la dépendance
        let url = format!(
            "{}/repos/{}/issues/{}/comments",
            self.api_url, self.repo, from_id
        );

        let comment = format!("Depends on #{}", to_id);

        #[derive(Serialize)]
        struct CommentBody<'a> {
            body: &'a str,
        }

        let response = self
            .client
            .post(&url)
            .json(&CommentBody { body: &comment })
            .send()
            .await
            .map_err(|e| ProviderError::Network(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status(); // Récupérer le statut avant d'appeler `.text()`
            let error_body = response
                .text()
                .await
                .unwrap_or_else(|_| String::from("Unable to read error response"));

            return Err(ProviderError::Api(format!(
                "Failed to create label. Status: {}, Body: {}",
                status, // Utiliser `status` ici
                error_body
            )));
        }

        Ok(())
    }

    async fn setup_project(&self, project: &Project) -> Result<(), ProviderError> {
        println!("Setting up project in GitHub: {}", project.name);

        // 1. Créer les milestones
        let mut milestone_ids = HashMap::new();
        for milestone in &project.milestones {
            println!("Creating milestone: {}", milestone.name);
            let id = self.create_milestone(milestone).await?;
            milestone_ids.insert(&milestone.version, id);
        }

        // 2. Créer les issues
        let mut issue_ids = HashMap::new();
        for issue in &project.issues {
            println!("Creating issue: {}", issue.title);

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

            let github_issue = GitHubIssue {
                title: issue.title.clone(),
                body: description,
                milestone: Some(*milestone_id),
                labels: issue.labels.clone(),
                assignees: Vec::new(),
            };

            let url = format!("{}/repos/{}/issues", self.api_url, self.repo,);

            let response = self
                .client
                .post(&url)
                .json(&github_issue)
                .send()
                .await
                .map_err(|e| ProviderError::Network(e.to_string()))?;

            if !response.status().is_success() {
                let status = response.status(); // Récupérer le statut avant d'appeler `.text()`
                let error_body = response
                    .text()
                    .await
                    .unwrap_or_else(|_| String::from("Unable to read error response"));

                return Err(ProviderError::Api(format!(
                    "Failed to create label. Status: {}, Body: {}",
                    status, // Utiliser `status` ici
                    error_body
                )));
            }

            let issue_response = response
                .json::<GitHubIssueResponse>()
                .await
                .map_err(|e| ProviderError::Api(e.to_string()))?;

            issue_ids.insert(&issue.title, issue_response.number);
        }

        // 3. Créer les liens entre les issues
        for issue in &project.issues {
            if let Some(&from_id) = issue_ids.get(&issue.title) {
                for dep in &issue.dependencies {
                    if let Some(&to_id) = issue_ids.get(dep) {
                        println!("Creating link from #{} to #{}", from_id, to_id);
                        self.create_issue_link(from_id, to_id).await?;
                    }
                }
            }
        }

        println!("Project setup completed successfully!");
        Ok(())
    }
}
