use sqlx::{Error, PgPool};

use crate::Task;

pub struct Storage {
    pool: PgPool,
}

impl Storage {
    pub async fn new(conn: &str) -> sqlx::Result<Self, Error> {
        let pool = PgPool::connect(conn).await?;
        Ok(Self { pool })
    }

    pub async fn query_tasks(&self) -> sqlx::Result<Vec<Task>> {
        let tasks = sqlx::query_as!(Task, "SELECT * FROM tasks")
            .fetch_all(&self.pool)
            .await?;
        Ok(tasks)
    }

    pub async fn insert_task(&self, task: &Task) -> sqlx::Result<i32> {
        let id = sqlx::query_scalar!(
            "INSERT INTO tasks (title, description) VALUES ($1, $2) RETURNING id",
            task.title,
            task.description
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(id)
    }

    pub(crate) async fn query_task(&self, id: i32) -> sqlx::Result<Option<Task>> {
        let task = sqlx::query_as!(Task, "SELECT * FROM tasks WHERE id = $1", id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(task)
    }

    pub(crate) async fn update_task(&self, id: i32, task: &Task) -> sqlx::Result<Option<i32>> {
        let id = sqlx::query_scalar!(
            "UPDATE tasks SET title = $1, description = $2 WHERE id = $3 RETURNING id",
            task.title,
            task.description,
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(id)
    }

    // pub(crate) async fn delete_task(&self, id: i32) -> sqlx::Result<bool> {
    //     let count = sqlx::query_scalar!("DELETE FROM tasks WHERE id = $1 RETURNING id", id)
    //         .fetch_one(&self.pool)
    //         .await?;
    //     Ok(count > 0)
    // }

    pub(crate) async fn delete_task(&self, id: i32) -> sqlx::Result<Option<Task>> {
        let task = sqlx::query_as!(Task, "DELETE FROM tasks WHERE id = $1 RETURNING *", id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(task)
    }
}
