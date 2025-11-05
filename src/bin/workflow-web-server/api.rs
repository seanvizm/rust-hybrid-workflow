use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkflowInfo {
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub path: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkflowStep {
    pub step_number: usize,
    pub name: String,
    pub language: String,
    pub output: Option<String>,
    pub status: StepStatus,
    pub duration_ms: Option<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum StepStatus {
    Pending,
    Running,
    Success,
    Failed,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkflowExecution {
    pub workflow_name: String,
    pub status: ExecutionStatus,
    pub steps: Vec<WorkflowStep>,
    pub total_duration_ms: Option<u64>,
    pub error: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ExecutionStatus {
    NotStarted,
    Running,
    Completed,
    Failed,
}
