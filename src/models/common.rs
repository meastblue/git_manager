use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Label {
    pub name: String,
    pub color: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub version: String,
    pub milestones: Vec<Milestone>,
    pub issues: Vec<ProjectIssue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectFile {
    pub project: Project,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Milestone {
    pub name: String,
    pub version: String,
    pub deadline: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Issue {
    pub title: String,
    pub labels: Vec<String>,
    pub description: IssueDescription,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectIssue {
    pub title: String,
    pub milestone: String,
    pub estimate: String,
    pub sprint: u32,
    pub dependencies: Vec<String>,
    pub labels: Vec<String>,
    pub description: IssueDescription,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IssueDescription {
    pub sections: Vec<Section>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Section {
    pub title: String,
    pub content: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct IssueCreate {
    pub title: String,
    pub description: String,
    pub labels: Vec<String>,
}

impl IssueDescription {
    pub fn to_markdown(&self) -> String {
        self.sections
            .iter()
            .map(|section| format!("{}\n{}\n", section.title, section.content.join("\n")))
            .collect::<Vec<_>>()
            .join("\n")
    }
}
