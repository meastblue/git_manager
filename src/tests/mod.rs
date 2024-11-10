#[cfg(test)]
mod tests {
    use crate::models::common::{
        IssueDescription, Label, Milestone, Project, ProjectIssue, Section,
    };
    use crate::providers::{create_provider, ProviderConfig, ProviderType};
    use std::env;

    // Helper pour créer une configuration de test
    fn create_test_config() -> ProviderConfig {
        ProviderConfig {
            api_url: "https://api.github.com".to_string(),
            token: "test_token".to_string(),
            repository: "test/repo".to_string(),
        }
    }

    #[test]
    fn test_create_provider() {
        let config = create_test_config();

        let github_provider = create_provider(ProviderType::GitHub, config.clone());
        assert!(github_provider.is_ok(), "Should create GitHub provider");

        let gitlab_provider = create_provider(ProviderType::GitLab, config);
        assert!(gitlab_provider.is_ok(), "Should create GitLab provider");
    }

    #[test]
    fn test_label_creation() {
        let label = Label {
            name: "test".to_string(),
            color: "#ff0000".to_string(),
            description: Some("Test label".to_string()),
        };

        assert_eq!(label.name, "test");
        assert_eq!(label.color, "#ff0000");
        assert_eq!(label.description, Some("Test label".to_string()));
    }

    #[test]
    fn test_issue_description_to_markdown() {
        let description = IssueDescription {
            sections: vec![Section {
                title: "## Test".to_string(),
                content: vec!["Line 1".to_string(), "Line 2".to_string()],
            }],
        };

        let markdown = description.to_markdown();
        assert!(markdown.contains("## Test"));
        assert!(markdown.contains("Line 1"));
        assert!(markdown.contains("Line 2"));
    }

    #[tokio::test]
    #[ignore] // Ignorer par défaut car nécessite des tokens valides
    async fn test_github_integration() {
        let token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN must be set");
        let config = ProviderConfig {
            api_url: "https://api.github.com".to_string(),
            token,
            repository: env::var("GITHUB_REPO").expect("GITHUB_REPO must be set"),
        };

        let provider = create_provider(ProviderType::GitHub, config).unwrap();

        // Test label creation
        let test_label = Label {
            name: "test_label".to_string(),
            color: "#ff0000".to_string(),
            description: Some("Test label".to_string()),
        };

        let result = provider.create_label(&test_label).await;
        assert!(result.is_ok(), "Should create label successfully");
    }
}
