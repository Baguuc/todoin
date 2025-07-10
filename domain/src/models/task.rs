#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow, Clone, Debug)]
pub struct Task {
    id: i32,
    project_id: i32,
    due_to: Option<chrono::DateTime<chrono::Utc>>,
    description: String,
    stage: TaskStage
}

#[derive(serde::Serialize, serde::Deserialize, sqlx::Type, Clone, Debug)]
#[sqlx(type_name = "task_stage", rename_all = "lowercase")]
pub enum TaskStage {
    Todo,
    #[sqlx(rename = "in-progress")]
    InProgress,
    Done
}
