use repo_manager::{
    error::ProviderError,
    models::common::*,
    providers::{create_provider, ProviderConfig, ProviderType},
};
use std::env;

async fn setup_test_provider(
) -> Result<Box<dyn repo_manager::traits::repository::RepositoryProvider>, ProviderError> {
    dotenv::dotenv().ok();

    let config = ProviderConfig {
        api_url: env::var("REPO_API_URL").expect("REPO_API_URL must be set"),
        token: env::var("REPO_TOKEN").expect("REPO_TOKEN must be set"),
        repository: env::var("REPO_PATH").expect("REPO_PATH must be set"),
    };

    create_provider(ProviderType::GitHub, config)
}

#[tokio::test]
#[ignore]
async fn test_full_project_setup() {
    let provider = setup_test_provider()
        .await
        .expect("Failed to create provider");

    let project = Project {
        name: "Test Project".to_string(),
        version: "0.1.0".to_string(),
        milestones: vec![Milestone {
            name: "v0.1.0".to_string(),
            version: "0.1.0".to_string(),
            deadline: "2024-12-31".to_string(),
            description: "Test milestone".to_string(),
        }],
        issues: vec![ProjectIssue {
            title: "Test Issue".to_string(),
            milestone: "0.1.0".to_string(),
            estimate: "1d".to_string(),
            sprint: 1,
            dependencies: vec![],
            labels: vec!["test".to_string()],
            description: IssueDescription {
                sections: vec![Section {
                    title: "## Objectif".to_string(),
                    content: vec!["Test objective".to_string()],
                }],
            },
        }],
    };

    let result = provider.setup_project(&project).await;
    assert!(result.is_ok(), "Project setup should succeed");
}

#[tokio::test]
#[ignore]
async fn test_label_operations() {
    let provider = setup_test_provider()
        .await
        .expect("Failed to create provider");

    let label = Label {
        name: "test_label".to_string(),
        color: "#ff0000".to_string(),
        description: Some("Test label".to_string()),
    };

    let result = provider.create_label(&label).await;
    assert!(result.is_ok(), "Label creation should succeed");
}
