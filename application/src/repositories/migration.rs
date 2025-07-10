const MIGRATIONS: [&str; 4] = [
"-- create type if not exists
DO $$
BEGIN
  CREATE TYPE task_stage AS ENUM('todo', 'in-progress', 'done');
EXCEPTION WHEN duplicate_object THEN
  NULL;
END $$;",

"CREATE TABLE IF NOT EXISTS projects (
  id SERIAL PRIMARY KEY,
  name VARCHAR(60),
  metadata JSON NOT NULL,
  priority INTEGER DEFAULT 1,
  active BOOL DEFAULT TRUE
);",

"CREATE TABLE IF NOT EXISTS tasks (
  id INTEGER PRIMARY KEY,
  project_id INTEGER NOT NULL, FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
  due_to TIMESTAMP, -- null means no due date
  description TEXT NOT NULL,
  stage task_stage DEFAULT 'todo'
);",

"CREATE TABLE IF NOT EXISTS user_projects (
  user_login TEXT NOT NULL, FOREIGN KEY (user_login) REFERENCES users(login) ON DELETE CASCADE,
  project_id INTEGER NOT NULL, FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);"
];

use crate::prelude::*;

pub struct MigrationRepository;

impl MigrationRepository {
    pub async fn migrate(client: &sqlx::postgres::PgPool) -> Result<()> {
        use sqlx::query;

        let mut tx = client.begin().await?;

        for sql in MIGRATIONS {
            let _ = query(sql).execute(&mut *tx).await?;
        }

        let _ = tx.commit().await?;
        
        return Ok(());
    }
}
