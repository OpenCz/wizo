use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GitHubWorkflow {
    pub name: Option<String>,
    pub run_name: Option<String>,
    pub on: PolymorphicTrigger,
    pub permissions: Option<PolymorphicPermissions>,
    pub env: Option<HashMap<String, serde_yaml::Value>>,
    pub defaults: Option<serde_yaml::Value>,
    pub concurrency: Option<PolymorphicConcurrency>,
    pub jobs: HashMap<String, PolymorphicJob>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PolymorphicTrigger {
    // Case 1 : `on: push`
    Single(String),
    // Case 2 : `on: [push, pull_request]`
    Array(Vec<String>),
    // Case 3 : `on: { push: { branches: [main] }, schedule: [{cron: '0 0 *'}] }`
    Map(HashMap<String, serde_yaml::Value>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PolymorphicPermissions {
    // `permissions: read-all`
    Global(String),
    // `permissions: { contents: write, issues: read }`
    Detailed(HashMap<String, String>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PolymorphicConcurrency {
    Group(String),
    Detailed {
        group: String,
        #[serde(rename = "cancel-in-progress")]
        cancel_in_progress: Option<serde_yaml::Value>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PolymorphicJob {
    Standard(Box<StandardJob>),
    Reusable(ReusableWorkflowJob),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct StandardJob {
    pub name: Option<String>,

    #[serde(rename = "if")]
    pub condition: Option<String>,

    pub needs: Option<PolymorphicList>,
    pub runs_on: PolymorphicList,
    pub steps: Vec<Step>,
    pub strategy: Option<serde_yaml::Value>,
    pub permissions: Option<PolymorphicPermissions>,
    pub env: Option<HashMap<String, serde_yaml::Value>>,

    #[serde(flatten)]
    pub catch_all: HashMap<String, serde_yaml::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ReusableWorkflowJob {
    pub uses: String,
    pub name: Option<String>,

    #[serde(rename = "if")]
    pub condition: Option<String>,

    pub needs: Option<PolymorphicList>,
    pub with: Option<HashMap<String, serde_yaml::Value>>,
    pub secrets: Option<PolymorphicSecrets>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Step {
    pub id: Option<String>,
    pub name: Option<String>,

    #[serde(rename = "if")]
    pub condition: Option<String>,

    pub uses: Option<String>,
    pub run: Option<String>,
    pub with: Option<HashMap<String, serde_yaml::Value>>,
    pub env: Option<HashMap<String, serde_yaml::Value>>,
    pub timeout_minutes: Option<serde_yaml::Value>,

    #[serde(rename = "continue-on-error")]
    pub continue_on_error: Option<serde_yaml::Value>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PolymorphicList {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PolymorphicSecrets {
    // `secrets: inherit`
    Inherit(String),
    // `secrets: { TOKEN: ${{ secrets.MY_TOKEN }} }`
    Map(HashMap<String, String>),
}

pub fn workflow_file(filename: &str) -> Result<GitHubWorkflow, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(filename)?;
    let workflow = serde_yaml::from_str(&content)?;

    Ok(workflow)
}
