// src/gitlab.rs
use crate::config::Label;
use crate::error::{AppError, AppResult};
use percent_encoding::{percent_encode, NON_ALPHANUMERIC};
use reqwest::{header, Client};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct GitLabLabel {
    name: String,
    color: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}

pub struct GitLabClient {
    client: Client,
    api_url: String,
    project_id: String,
}

impl GitLabClient {
    pub fn new(api_url: String, token: String, project_id: String) -> AppResult<Self> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "PRIVATE-TOKEN",
            header::HeaderValue::from_str(&token)
                .map_err(|e| AppError::Config(format!("Invalid token: {}", e)))?,
        );

        let client = Client::builder()
            .default_headers(headers)
            .build()
            .map_err(|e| AppError::GitLabApi(e))?;

        Ok(Self {
            client,
            api_url,
            project_id,
        })
    }

    pub async fn create_label(&self, label: &Label) -> AppResult<()> {
        let encoded_project_id = percent_encode(self.project_id.as_bytes(), NON_ALPHANUMERIC);
        let url = format!("{}/projects/{}/labels", self.api_url, encoded_project_id);

        let gitlab_label = GitLabLabel {
            name: label.name.clone(),
            color: label.color.clone(),
            description: label.description.clone(),
        };

        self.client
            .post(&url)
            .json(&gitlab_label)
            .send()
            .await
            .map_err(AppError::GitLabApi)?;

        Ok(())
    }
}
