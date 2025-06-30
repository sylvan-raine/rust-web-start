use axum::{debug_handler, routing, Router};
use axum::extract::{Path, State};
use sea_orm::prelude::{Date, Expr};
use sea_orm::sea_query::IntoCondition;
use sea_orm::{
    ActiveModelTrait, DeriveIntoActiveModel, EntityTrait, IntoActiveModel,
    JoinType, ModelTrait, PaginatorTrait, QuerySelect, QueryTrait, RelationTrait
};
use serde::Deserialize;
use validator::Validate;
use crate::entity::prelude::Score;
use crate::entity::score::{ActiveModel, Model};
use crate::entity::{course, student};
use crate::error::AppError;
use crate::route::extract::{ValidJson, ValidQuery};
use crate::route::not_found;
use crate::route::page::{Page, PageParam};
use crate::route::result::AppResult;
use crate::server::ServerState;
use crate::throw_err;

pub fn router() -> Router<ServerState> {
    Router::new()
        .route("/", routing::get(index))
        .route("/query", routing::get(query))
        .route("/insert", routing::post(insert))
        .route("/update/{stu_id}/{course_id}", routing::put(update))
        .route("/delete/{stu_id}/{course_id}", routing::delete(delete))
        .fallback(not_found)
}

/// 路由到 score 模块下的默认界面
async fn index() -> AppResult<&'static str> {
    AppResult::Ok("Welcome! This is the index page of score.")
}

#[derive(Validate, Deserialize, DeriveIntoActiveModel)]
struct InsertParams {
    #[validate(length(max = 6))]
    stu_id: String,

    #[validate(length(max = 6))]
    course_id: String,

    score: Option<i32>,
    record_date: Option<Date>
}

async fn insert(
    State(state): State<ServerState>,
    ValidJson(json): ValidJson<InsertParams>
) -> AppResult<String> {
    throw_err!(json.into_active_model().insert(state.db()).await);
    tracing::debug!("Created score");
    AppResult::Ok("Succesfully inserted score!".to_string())
}

async fn update(
    State(state): State<ServerState>,
    Path((stu_id, course_id)): Path<(String, String)>,
    ValidJson(json): ValidJson<InsertParams>
) -> AppResult<Model> {
    let target = throw_err!(Score::find_by_id((stu_id, course_id)).one(state.db()).await);
    if let Some(score) = target {
        throw_err!(json.into_active_model().update(state.db()).await);
        tracing::debug!("Updated score");
        AppResult::Ok(score)
    } else {
        AppResult::Err(AppError::NotFound("No such score specified.".to_string()))
    }
}

async fn delete(
    State(state): State<ServerState>,
    Path((stu_id, course_id)): Path<(String, String)>
) -> AppResult<String> {
    let target = throw_err!(Score::find_by_id((stu_id.clone(), course_id.clone())).one(state.db()).await);
    if let Some(score) = target {
        throw_err!(score.delete(state.db()).await);
        AppResult::Ok(format!("Successfully deleted score, student id: {stu_id}, course_id: {course_id}."))    
    } else {
        AppResult::Err(AppError::NotFound("No such score specified.".to_string()))
    }
}

#[derive(Validate, Deserialize)]
struct QueryParams {
    student: Option<String>,

    course: Option<String>,

    #[serde(flatten)]
    page: PageParam
}

/// 处理路由到 score 模块下的查询请求
#[debug_handler]
async fn query(
    State(state): State<ServerState>,
    ValidQuery(params): ValidQuery<QueryParams>
) -> AppResult<Page<Model>> {
    tracing::debug!("Begin to handle: Query score");
    let pagination = Score::find()
        .apply_if(params.student, |rows, keyword| {
            rows.join(JoinType::InnerJoin, student::Relation::Score.def().rev()
                .on_condition(move |_score, student_name| {
                    Expr::col((student_name, student::Column::Name)).like(format!("%{keyword}%")).into_condition()
                })
            )
        })
        .apply_if(params.course, |rows, keyword| {
            rows.join(JoinType::InnerJoin, course::Relation::Score.def().rev()
                .on_condition(move |_score, course_name| {
                    Expr::col((course_name, course::Column::Name)).like(format!("%{keyword}%")).into_condition()
                })
            )
        })
        .paginate(state.db(), params.page.size);

    let total = throw_err!(pagination.num_pages().await);
    let items = throw_err!(pagination.fetch_page(params.page.index - 1).await);
    
    AppResult::Ok(Page { param: params.page, total, items })
}