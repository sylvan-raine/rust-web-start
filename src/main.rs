mod logger;
mod app_config;
mod database;
mod entity;

use axum::{debug_handler, routing, Router};
use axum::extract::State;
use axum::response::IntoResponse;
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};
use tokio::net::TcpListener;
use entity::prelude::*;
use crate::entity::student;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logger::init();
    let db = database::init().await?;
    let router = Router::new()
        .route("/", routing::get(query_index))
        .route("/index", routing::get(query_index))
        .route("/student", routing::get(query_student)).with_state(db);
    
    let port = app_config::get_server().port();
    
    let listener = TcpListener::bind(("127.0.0.1", port)).await?;
    tracing::info!("listening on {}.", listener.local_addr()?);
    axum::serve(listener, router).await?;
    
    Ok(())
}

#[debug_handler]
async fn query_index() -> &'static str {
    "Hello world!"
}

#[debug_handler]
async fn query_student(State(db): State<DatabaseConnection>) -> impl IntoResponse {
    let stu = Student::find()
        .filter(
            Condition::all()
                .add(student::Column::Sex.eq("女"))
                .add(student::Column::Name.starts_with("钱"))
                .add(student::Column::Name.ends_with("多"))
        )
        .all(&db)
        .await
        .unwrap();

    axum::Json(stu)
}