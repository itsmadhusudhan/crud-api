mod api_error;
mod db;
mod handlers;
mod task;

use std::sync::Arc;

#[allow(unused_imports)]
use crate::db::{Database, MemoryDB, SqliteDB, DB};
use crate::handlers::{create_task, delete_task, get_tasks, update_task};
use actix_web::{middleware::Logger, web, App, HttpServer};

#[derive(Debug, Clone)]
pub struct AppState<T: DB> {
    pub db: Arc<T>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    // let memory_db = MemoryDB::new();

    // create sqlite instance

    let sqlite_db = SqliteDB::new().await;
    let server = HttpServer::new(move || {
        // Wrap the MemoryDB instance in the Database enum
        // let database = Database::MemoryDB(memory_db.clone());
        let database = Database::SqliteDB(sqlite_db.clone());

        // Use the Database enum instance in the AppState struct
        let app_state = web::Data::new(AppState {
            db: Arc::new(database),
        });

        let logger = Logger::default();

        App::new().wrap(logger).app_data(app_state).service(
            web::scope("/api/v1")
                .service(create_task)
                .service(get_tasks)
                .service(update_task)
                .service(delete_task),
        )
    })
    .bind(("127.0.0.1", 5412))?
    .run();

    log::info!("Server running at http://localhost:5412/");

    server.await
}

#[cfg(test)]
mod api_tests {
    use super::*;
    use crate::task::CreateTask;
    use actix_web::test;

    #[actix_rt::test]
    async fn test_create_task() {
        let memory_db = MemoryDB::new();
        let database = Database::MemoryDB(memory_db.clone());
        let app_state = web::Data::new(AppState {
            db: Arc::new(database),
        });

        let mut app = test::init_service(
            App::new()
                .app_data(app_state)
                .service(web::scope("/api/v1").service(create_task)),
        )
        .await;

        let task = CreateTask {
            name: "test".to_string(),
            due_date: None,
        };

        let req = test::TestRequest::post()
            .uri("/api/v1/task")
            .set_json(&task)
            .to_request();

        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), actix_web::http::StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_create_task_empty_name() {
        let memory_db = MemoryDB::new();
        let database = Database::MemoryDB(memory_db.clone());
        let app_state = web::Data::new(AppState {
            db: Arc::new(database),
        });

        let mut app = test::init_service(
            App::new()
                .app_data(app_state)
                .service(web::scope("/api/v1").service(create_task)),
        )
        .await;

        let task = CreateTask {
            name: "".to_string(),
            due_date: None,
        };

        let req = test::TestRequest::post()
            .uri("/api/v1/task")
            .set_json(&task)
            .to_request();

        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), actix_web::http::StatusCode::BAD_REQUEST);
    }

    #[actix_rt::test]
    async fn test_create_task_empty_name2() {
        let memory_db = MemoryDB::new();
        let database = Database::MemoryDB(memory_db.clone());
        let app_state = web::Data::new(AppState {
            db: Arc::new(database),
        });

        let mut app = test::init_service(
            App::new()
                .app_data(app_state)
                .service(web::scope("/api/v1").service(create_task)),
        )
        .await;

        let task = CreateTask {
            name: "".to_string(),
            due_date: None,
        };

        let req = test::TestRequest::post()
            .uri("/api/v1/task")
            .set_json(&task)
            .to_request();

        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), actix_web::http::StatusCode::BAD_REQUEST);
    }
}
