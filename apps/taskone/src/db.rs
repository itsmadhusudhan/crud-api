use crate::task::Task;
use async_trait::async_trait;
use sqlx::SqlitePool;
use std::fmt::Error;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[allow(dead_code)]
#[allow(unsafe_code)]
#[async_trait]
pub trait DB {
    async fn get_tasks(&self) -> Vec<Task>;
    async fn get_task(&self, id: Uuid) -> Option<Task>;
    async fn create_task(&self, task: Task) -> Result<Task, Error>;
    async fn update_task(&self, id: Uuid, task: Task) -> Option<Task>;
    async fn delete_task(&self, id: Uuid) -> Option<bool>;
}

#[derive(Debug, Clone)]
pub struct MemoryDB {
    tasks: Arc<Mutex<Vec<Task>>>,
}
impl MemoryDB {
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(Mutex::new(vec![])),
        }
    }
}

#[async_trait]
impl DB for MemoryDB {
    async fn get_tasks(&self) -> Vec<Task> {
        let tasks = self.tasks.lock().unwrap();
        tasks.clone()
    }

    async fn get_task(&self, id: Uuid) -> Option<Task> {
        let tasks = self.tasks.lock().unwrap();
        tasks.iter().find(|task| task.id == id).cloned()
    }

    async fn create_task(&self, task: Task) -> Result<Task, Error> {
        println!("create_task");
        let mut tasks = self.tasks.lock().unwrap();
        tasks.push(task.clone());
        Ok(task)
    }

    async fn update_task(&self, id: Uuid, task: Task) -> Option<Task> {
        let mut tasks = self.tasks.lock().unwrap();
        let index = tasks.iter().position(|task| task.id == id)?;
        tasks[index] = task.clone();
        Some(task)
    }

    async fn delete_task(&self, id: Uuid) -> Option<bool> {
        let mut tasks = self.tasks.lock().unwrap();
        let index = tasks.iter().position(|task| task.id == id)?;
        tasks.remove(index);
        Some(true)
    }
}

#[derive(Debug, Clone)]
pub struct SqliteDB {
    db: Arc<SqlitePool>,
}

impl SqliteDB {
    pub async fn new() -> Self {
        Self {
            db: Arc::new(SqlitePool::connect("todo.db").await.unwrap()),
        }
    }
}

#[async_trait]
impl DB for SqliteDB {
    async fn get_tasks(&self) -> Vec<Task> {
        let tasks = sqlx::query_as::<_, Task>("SELECT * FROM tasks")
            .fetch_all(&*self.db)
            .await
            .unwrap();
        tasks
    }

    async fn get_task(&self, id: Uuid) -> Option<Task> {
        let task = sqlx::query_as::<_, Task>("SELECT * FROM tasks WHERE id = ?")
            .bind(id)
            .fetch_one(&*self.db)
            .await
            .ok();
        task
    }

    async fn create_task(&self, task: Task) -> Result<Task, Error> {
        println!("create_task");
        println!("{:?}", task);

        let result_task = task.clone();

        let result = sqlx::query("INSERT INTO tasks (id, name, done, due_date, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?)")
            .bind(task.id)
            .bind(task.name)
            .bind(task.done)
            .bind(task.due_date)
            .bind(task.created_at)
            .bind(task.updated_at).execute(&*self.db)
            .await;

        if result.is_err() {
            println!("Error: {:?}", result.err());

            return Err(Error::default());
        }

        Ok(result_task)
    }

    async fn update_task(&self, id: Uuid, task: Task) -> Option<Task> {
        let task = sqlx::query_as::<_, Task>(
            "UPDATE tasks SET name = ?, done = ?, due_date = ?, updated_at = ? WHERE id = ?",
        )
        .bind(task.name)
        .bind(task.done)
        .bind(task.due_date)
        .bind(task.updated_at)
        .bind(id)
        .fetch_one(&*self.db)
        .await
        .ok();
        task
    }

    async fn delete_task(&self, id: Uuid) -> Option<bool> {
        let task = sqlx::query("DELETE FROM tasks WHERE id = ?")
            .bind(id)
            .execute(&*self.db)
            .await
            .ok();

        if task.is_some() {
            return Some(true);
        }

        Some(false)
    }
}

pub enum Database {
    MemoryDB(MemoryDB),
    SqliteDB(SqliteDB),
}

#[async_trait]
impl DB for Database {
    async fn get_tasks(&self) -> Vec<Task> {
        match self {
            Database::MemoryDB(db) => db.get_tasks().await,
            Database::SqliteDB(db) => db.get_tasks().await,
        }
    }

    async fn get_task(&self, id: Uuid) -> Option<Task> {
        match self {
            Database::MemoryDB(db) => db.get_task(id).await,
            Database::SqliteDB(db) => db.get_task(id).await,
        }
    }

    async fn create_task(&self, task: Task) -> Result<Task, Error> {
        match self {
            Database::MemoryDB(db) => db.create_task(task).await,
            Database::SqliteDB(db) => db.create_task(task).await,
        }
    }

    async fn update_task(&self, id: Uuid, task: Task) -> Option<Task> {
        match self {
            Database::MemoryDB(db) => db.update_task(id, task).await,
            Database::SqliteDB(db) => db.update_task(id, task).await,
        }
    }

    async fn delete_task(&self, id: Uuid) -> Option<bool> {
        match self {
            Database::MemoryDB(db) => db.delete_task(id).await,
            Database::SqliteDB(db) => db.delete_task(id).await,
        }
    }
}
