use crate::prelude::*;

pub struct TaskRepository;

impl TaskRepository {
    pub async fn insert<'c, C: sqlx::postgres::PgExecutor<'c>, M: serde::Serialize>(project_id: &i32, due_to: &Option<chrono::DateTime<chrono::Utc>>, description: &String, stage: todoin_domain::TaskStage, client: C) -> Result<()> {
        use sqlx::query;

        let sql = "INSERT INTO tasks (project_id, due_to, description, stage) VALUES ($1, $2, $3, $4);";
        let result = query(sql).bind(project_id).bind(due_to).bind(description).bind(stage).execute(client).await;

        match result {
            Ok(_) => return Ok(()),
            Err(err) => return Err(Error::Sqlx(err))
        };
    }
    
    pub async fn list<'c, C: sqlx::postgres::PgExecutor<'c>>(user_login: &String, project_id: &i32, client: C) -> Result<Vec<todoin_domain::Task>> {
        use sqlx::query_as;

        let sql = "SELECT p.* FROM projects p INNER JOIN user_projects up ON p.id = up.project_id INNER JOIN tasks t ON p.id = t.project_id WHERE up.user_login = $1 AND p.id = $2;";
        let result = query_as(sql).bind(user_login).bind(project_id).fetch_all(client).await;

        match result {
            Ok(result) => return Ok(result),
            Err(err) => return Err(Error::Sqlx(err))
        };
    }
    
    pub async fn delete<'c, C: sqlx::postgres::PgExecutor<'c>>(id: &i32, client: C) -> Result<()> {
        use sqlx::query;

        let sql = "DELETE FROM tasks WHERE id = $1;";
        let result = query(sql).bind(id).execute(client).await;

        match result {
            Ok(_) => return Ok(()),
            Err(err) => return Err(Error::Sqlx(err))
        };
    }

    pub async fn update_stage<'c, C: sqlx::postgres::PgExecutor<'c>>(id: &i32, new_stage: todoin_domain::TaskStage, client: C) -> Result<()> {
        use sqlx::query;

        let sql = "UPDATE tasks SET stage = $1 WHERE id = $2;";
        let result = query(sql).bind(new_stage).bind(id).execute(client).await;

        match result {
            Ok(_) => return Ok(()),
            Err(err) => return Err(Error::Sqlx(err))
        };
    }
}
