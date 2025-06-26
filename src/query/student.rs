use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
use axum::{debug_handler, routing, Router};
use axum::extract::State;
use axum::response::IntoResponse;
use sea_orm::{Condition, EntityTrait};
use crate::entity::prelude::Student;
use crate::entity::student;
use crate::query::response::QueryResult;
use crate::server::ServerState;

pub fn router() -> Router<ServerState> {
    Router::new()
        .route("/student", routing::get(query_student))
}

#[debug_handler]
async fn query_student(State(state): State<ServerState>) -> impl IntoResponse {
    tracing::debug!("Query student");
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

    QueryResult::Ok(stu)
}