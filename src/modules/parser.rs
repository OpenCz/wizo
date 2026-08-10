use serde::de::{Deserializer, Error as _};
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
    pub defaults: Option<WorkflowDefaults>,
    pub concurrency: Option<PolymorphicConcurrency>,
    pub jobs: HashMap<String, PolymorphicJob>,

    #[serde(flatten)]
    pub catch_all: HashMap<String, serde_yaml::Value>,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum PolymorphicTrigger {
    // Case 1 : `on: push`
    Single(String),
    // Case 2 : `on: [push, pull_request]`
    Array(Vec<String>),
    // Case 3 : `on: { push: { branches: [main] }, schedule: [{cron: '0 0 *'}] }`
    Map(HashMap<String, PolymorphicTriggerConfig>),
}

impl<'de> Deserialize<'de> for PolymorphicTrigger {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = serde_yaml::Value::deserialize(deserializer)?;

        match value {
            serde_yaml::Value::String(trigger) => Ok(PolymorphicTrigger::Single(trigger)),
            serde_yaml::Value::Sequence(triggers) => {
                let mut events = Vec::with_capacity(triggers.len());

                for trigger in triggers {
                    match trigger {
                        serde_yaml::Value::String(trigger) => events.push(trigger),
                        other => {
                            return Err(D::Error::custom(format!(
                                "workflow trigger arrays must contain strings, got {other:?}"
                            )));
                        }
                    }
                }

                Ok(PolymorphicTrigger::Array(events))
            }
            serde_yaml::Value::Mapping(triggers) => {
                let mut events = HashMap::with_capacity(triggers.len());

                for (event_name, trigger_config) in triggers {
                    let event_name = event_name
                        .as_str()
                        .ok_or_else(|| D::Error::custom("workflow event names must be strings"))?
                        .to_string();

                    let trigger_config = match event_name.as_str() {
                        "schedule" => PolymorphicTriggerConfig::Schedule(
                            Vec::<ScheduleTrigger>::deserialize(trigger_config)
                                .map_err(D::Error::custom)?,
                        ),
                        "workflow_dispatch" => PolymorphicTriggerConfig::WorkflowDispatch(
                            WorkflowDispatchTrigger::deserialize(trigger_config)
                                .map_err(D::Error::custom)?,
                        ),
                        "workflow_call" => PolymorphicTriggerConfig::WorkflowCall(
                            WorkflowCallTrigger::deserialize(trigger_config)
                                .map_err(D::Error::custom)?,
                        ),
                        "workflow_run" => PolymorphicTriggerConfig::WorkflowRun(
                            WorkflowRunTrigger::deserialize(trigger_config)
                                .map_err(D::Error::custom)?,
                        ),
                        "push" | "pull_request" | "pull_request_target" | "release" => {
                            PolymorphicTriggerConfig::Filters(
                                TriggerFilters::deserialize(trigger_config)
                                    .map_err(D::Error::custom)?,
                            )
                        }
                        _ => PolymorphicTriggerConfig::Raw(trigger_config),
                    };

                    events.insert(event_name, trigger_config);
                }

                Ok(PolymorphicTrigger::Map(events))
            }
            other => Err(D::Error::custom(format!(
                "workflow triggers must be a string, array, or map, got {other:?}"
            ))),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PolymorphicTriggerConfig {
    WorkflowRun(WorkflowRunTrigger),
    Schedule(Vec<ScheduleTrigger>),
    WorkflowDispatch(WorkflowDispatchTrigger),
    WorkflowCall(WorkflowCallTrigger),
    Filters(TriggerFilters),
    Raw(serde_yaml::Value),
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct WorkflowDefaults {
    pub run: Option<RunDefaults>,

    #[serde(flatten)]
    pub catch_all: HashMap<String, serde_yaml::Value>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct RunDefaults {
    pub shell: Option<String>,

    #[serde(rename = "working-directory")]
    pub working_directory: Option<String>,

    #[serde(flatten)]
    pub catch_all: HashMap<String, serde_yaml::Value>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct TriggerFilters {
    pub branches: Option<Vec<String>>,

    #[serde(rename = "branches-ignore")]
    pub branches_ignore: Option<Vec<String>>,

    pub tags: Option<Vec<String>>,

    #[serde(rename = "tags-ignore")]
    pub tags_ignore: Option<Vec<String>>,

    pub paths: Option<Vec<String>>,

    #[serde(rename = "paths-ignore")]
    pub paths_ignore: Option<Vec<String>>,

    pub types: Option<Vec<String>>,

    #[serde(flatten)]
    pub catch_all: HashMap<String, serde_yaml::Value>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct WorkflowRunTrigger {
    #[serde(alias = "worlflows")]
    pub workflows: Option<Vec<String>>,

    pub branches: Option<Vec<String>>,

    #[serde(rename = "branches-ignore")]
    pub branches_ignore: Option<Vec<String>>,

    pub tags: Option<Vec<String>>,

    #[serde(rename = "tags-ignore")]
    pub tags_ignore: Option<Vec<String>>,

    pub types: Option<Vec<String>>,

    #[serde(flatten)]
    pub catch_all: HashMap<String, serde_yaml::Value>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct WorkflowDispatchTrigger {
    pub inputs: Option<HashMap<String, WorkflowInput>>,

    #[serde(flatten)]
    pub catch_all: HashMap<String, serde_yaml::Value>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct WorkflowCallTrigger {
    pub inputs: Option<HashMap<String, WorkflowInput>>,
    pub secrets: Option<HashMap<String, WorkflowInput>>,
    pub outputs: Option<HashMap<String, serde_yaml::Value>>,

    #[serde(flatten)]
    pub catch_all: HashMap<String, serde_yaml::Value>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct WorkflowInput {
    pub description: Option<String>,
    pub required: Option<bool>,
    pub default: Option<serde_yaml::Value>,

    #[serde(rename = "type")]
    pub input_type: Option<String>,

    pub options: Option<Vec<String>>,

    #[serde(flatten)]
    pub catch_all: HashMap<String, serde_yaml::Value>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ScheduleTrigger {
    pub cron: String,

    #[serde(flatten)]
    pub catch_all: HashMap<String, serde_yaml::Value>,
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
    pub strategy: Option<JobStrategy>,
    pub defaults: Option<WorkflowDefaults>,
    pub outputs: Option<HashMap<String, serde_yaml::Value>>,
    pub permissions: Option<PolymorphicPermissions>,
    pub env: Option<HashMap<String, serde_yaml::Value>>,
    pub concurrency: Option<PolymorphicConcurrency>,
    pub environment: Option<PolymorphicEnvironment>,
    #[serde(rename = "timeout-minutes")]
    pub timeout_minutes: Option<serde_yaml::Value>,
    #[serde(rename = "continue-on-error")]
    pub continue_on_error: Option<serde_yaml::Value>,
    pub container: Option<serde_yaml::Value>,
    pub services: Option<HashMap<String, serde_yaml::Value>>,

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
    pub outputs: Option<HashMap<String, serde_yaml::Value>>,
    pub concurrency: Option<PolymorphicConcurrency>,
    pub environment: Option<PolymorphicEnvironment>,
    pub container: Option<serde_yaml::Value>,
    pub services: Option<HashMap<String, serde_yaml::Value>>,

    #[serde(flatten)]
    pub catch_all: HashMap<String, serde_yaml::Value>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct JobStrategy {
    pub matrix: Option<serde_yaml::Value>,

    #[serde(rename = "fail-fast")]
    pub fail_fast: Option<bool>,

    #[serde(rename = "max-parallel")]
    pub max_parallel: Option<u32>,

    #[serde(flatten)]
    pub catch_all: HashMap<String, serde_yaml::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PolymorphicEnvironment {
    Name(String),
    Detailed {
        name: String,
        url: Option<serde_yaml::Value>,

        #[serde(flatten)]
        catch_all: HashMap<String, serde_yaml::Value>,
    },
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
    pub shell: Option<String>,
    #[serde(rename = "working-directory")]
    pub working_directory: Option<String>,
    pub with: Option<HashMap<String, serde_yaml::Value>>,
    pub env: Option<HashMap<String, serde_yaml::Value>>,
    pub timeout_minutes: Option<serde_yaml::Value>,

    #[serde(rename = "continue-on-error")]
    pub continue_on_error: Option<serde_yaml::Value>,

    #[serde(flatten)]
    pub catch_all: HashMap<String, serde_yaml::Value>,
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
