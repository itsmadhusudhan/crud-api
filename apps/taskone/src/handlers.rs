use crate::api_error::CustomError;
use crate::db::{Database, DB};
use crate::task::{CreateTask, Task};
use crate::AppState;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use validator::Validate;

// routes
#[post("/task")]
pub async fn create_task(
    app_state: web::Data<AppState<Database>>,
    task: web::Json<CreateTask>,
) -> Result<HttpResponse, CustomError> {
    match task.validate() {
        Ok(_) => {
            let task = Task::new(task.name.clone(), task.due_date);

            let result = app_state.db.create_task(task).await;

            match result {
                Ok(created_task) => Ok(HttpResponse::Ok().json(created_task)),
                Err(_e) => Err(CustomError::NameEmpty),
            }
        }
        Err(_e) => match _e.field_errors().get("name") {
            Some(_) => Err(CustomError::NameEmpty),
            None => Err(CustomError::NameEmpty),
        },
    }
}

// get tasks
#[get("/tasks")]
pub async fn get_tasks(app_state: web::Data<AppState<Database>>) -> impl Responder {
    let tasks = app_state.db.get_tasks().await;

    HttpResponse::Ok().json(tasks)
}

// get task
#[get("/task/{id}")]
pub async fn get_task(
    app_state: web::Data<AppState<Database>>,
    id: web::Path<String>,
) -> Result<HttpResponse, CustomError> {
    let uuid = id.parse::<uuid::Uuid>();

    if uuid.is_err() {
        return Err(CustomError::NotFound);
    }

    let task = app_state
        .db
        .get_task(uuid.unwrap())
        .await
        .ok_or(CustomError::NotFound)?;

    Ok(HttpResponse::Ok().json(task))
}

// update task
#[put("/task/{id}")]
pub async fn update_task(
    app_state: web::Data<AppState<Database>>,
    id: web::Path<String>,
    task: web::Json<CreateTask>,
) -> Result<HttpResponse, CustomError> {
    let uuid = id.parse::<uuid::Uuid>();

    if uuid.is_err() {
        return Err(CustomError::NotFound);
    }

    let task = Task::new(task.name.clone(), task.due_date);

    let updated_task = app_state
        .db
        .update_task(uuid.unwrap(), task)
        .await
        .ok_or(CustomError::NotFound)?;

    Ok(HttpResponse::Ok().json(updated_task))
}

// delete task
#[delete("/task/{id}")]
pub async fn delete_task(
    app_state: web::Data<AppState<Database>>,
    id: web::Path<String>,
) -> Result<HttpResponse, CustomError> {
    let uuid = id.parse::<uuid::Uuid>();

    println!("uuid: {:?}", uuid);

    if uuid.is_err() {
        return Err(CustomError::NotFound);
    }

    let deleted_task = app_state
        .db
        .delete_task(uuid.unwrap())
        .await
        .ok_or(CustomError::NotFound)?;

    Ok(HttpResponse::Ok().json(deleted_task))
}
