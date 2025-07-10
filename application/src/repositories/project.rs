use crate::prelude::*;

pub struct ProjectRepository;

impl ProjectRepository {
    pub async fn insert<'c, C: sqlx::postgres::PgExecutor<'c>, M: serde::Serialize>(name: &String, metadata: &M, priority: &i32, client: C) -> Result<()> {
        use sqlx::query;

        let metadata_serialized = serde_json::to_value(metadata)?;

        let sql = "INSERT INTO projects (name, metadata, priority) VALUES ($1, $2, $3);";
        let result = query(sql).bind(name).bind(metadata_serialized).bind(priority).execute(client).await;

        match result {
            Ok(_) => return Ok(()),
            Err(err) => return Err(Error::Sqlx(err))
        };
    }
    
    pub async fn retrieve<'c, C: sqlx::postgres::PgExecutor<'c>>(id: &i32, user_login: &String, client: C) -> Result<()> {
        use sqlx::query_as;

        let sql = "SELECT p.* FROM projects p INNER JOIN user_projects up ON p.id = up.project_id WHERE p.id = $1 AND up.user_login = $2;";
        let result = query_as(sql).bind(id).bind(user_login).fetch_one(client).await;

        match result {
            Ok(group) => return Ok(group),
            Err(err) => return Err(Error::Sqlx(err))
        };
    }
    
    pub async fn list<'c, C: sqlx::postgres::PgExecutor<'c>>(user_login: &String, client: C) -> Result<Vec<todoin_domain::Project>> {
        use sqlx::query_as;

        let sql = "SELECT p.* FROM projects p INNER JOIN user_projects up ON p.id = up.project_id WHERE up.user_login = $1;";
        let result = query_as(sql).bind(user_login).fetch_all(client).await;

        match result {
            Ok(groups) => return Ok(groups),
            Err(err) => return Err(Error::Sqlx(err))
        };
    }
    
    pub async fn delete<'c, C: sqlx::postgres::PgExecutor<'c>>(id: &i32, client: C) -> Result<()> {
        use sqlx::query;

        let sql = "DELETE FROM projects WHERE id = $1;";
        let result = query(sql).bind(id).execute(client).await;

        match result {
            Ok(_) => return Ok(()),
            Err(err) => return Err(Error::Sqlx(err))
        };
    }
    
    pub async fn set_active<'c, C: sqlx::postgres::PgExecutor<'c>>(id: &i32, active: &bool, client: C) -> Result<()> {
        use sqlx::query;

        let sql = "UPDATE projects SET active = $1 WHERE id = $2;";
        let result = query(sql).bind(active).bind(id).execute(client).await;

        match result {
            Ok(_) => return Ok(()),
            Err(err) => return Err(Error::Sqlx(err))
        };
    }
    
    pub async fn grant<'c, C: sqlx::postgres::PgExecutor<'c>>(id: &i32, user_login: &String, client: C) -> Result<()> {
        use sqlx::query;

        let sql = "INSERT INTO user_projects (user_login, project_id) VALUES ($1, $2);";
        let result = query(sql).bind(user_login).bind(id).execute(client).await;

        match result {
            Ok(_) => return Ok(()),
            Err(err) => return Err(Error::Sqlx(err))
        };
    }
    
    pub async fn revoke<'c, C: sqlx::postgres::PgExecutor<'c>>(id: &i32, user_login: &String, client: C) -> Result<()> {
        use sqlx::query;

        let sql = "DELETE FROM user_projects WHERE user_login = $1 AND project_id = $2;";
        let result = query(sql).bind(user_login).bind(id).execute(client).await;

        match result {
            Ok(_) => return Ok(()),
            Err(err) => return Err(Error::Sqlx(err))
        };
    }
}
