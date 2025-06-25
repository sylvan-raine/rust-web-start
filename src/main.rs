mod logger;
mod app_config;
mod database;
mod entity;
mod server;
mod app;

use axum::{debug_handler, routing, Router};
use axum::extract::State;
use axum::response::IntoResponse;
use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter};
use entity::prelude::*;
use crate::entity::student;
use crate::server::ServerState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let router = Router::new()
        .route("/", routing::get(query_index))
        .route("/index", routing::get(query_index))
        .route("/student", routing::get(query_student));
    
    app::run(router).await?;
    
    Ok(())
}

#[debug_handler]
async fn query_index() -> &'static str {
    "Hello world!"
}

#[debug_handler]
async fn query_student(State(state): State<ServerState>) -> impl IntoResponse {
    let stu = Student::find()
        .filter(
            Condition::all()
                .add(student::Column::Sex.eq("女"))
                .add(student::Column::Name.starts_with("钱"))
                .add(student::Column::Name.ends_with("多"))
        )
        .all(state.db())
        .await
        .unwrap();

    axum::Json(stu)
}