#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow, Clone, Debug)]
pub struct Project {
    id: i32,
    name: String,
    metadata: serde_json::Value,
    priority: i32,
    active: bool
}
