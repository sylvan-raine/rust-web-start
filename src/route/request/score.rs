use crate::entity::prelude::Score;
use crate::entity::score::{ActiveModel, Model};
use crate::entity::{course, student};
use crate::error::AppError;
use crate::route::extract::{ValidJson, ValidQuery};
use crate::route::page::{Page, PageParam};
use crate::route::result::AppResult;
use crate::server::ServerState;
use crate::throw_err;
use axum::extract::{Path, State};
use axum::{Router, debug_handler, routing};
use sea_orm::prelude::{Date, Expr};
use sea_orm::sea_query::IntoCondition;
use sea_orm::{
    ActiveModelTrait, DeriveIntoActiveModel, EntityTrait, IntoActiveModel, JoinType, ModelTrait,
    PaginatorTrait, QuerySelect, QueryTrait, RelationTrait,
};
use serde::Deserialize;
use validator::Validate;

pub fn router() -> Router<ServerState> {
    Router::new()
        .route("/", routing::get(index))
        .route("/query", routing::get(query))
        .route("/insert", routing::post(insert))
        .route("/update/{stu_id}/{course_id}", routing::put(update))
        .route("/delete/{stu_id}/{course_id}", routing::delete(delete))
}

/// 路由到 score 模块下的默认界面
#[debug_handler]
async fn index() -> AppResult<&'static str> {
    AppResult::Ok("欢迎!这是 Score 的首页!")
}

#[derive(Validate, Deserialize, DeriveIntoActiveModel)]
struct InsertParams {
    #[validate(length(max = 6))]
    stu_id: String,

    #[validate(length(max = 6))]
    course_id: String,

    score: Option<i32>,
    record_date: Option<Date>,
}

#[debug_handler]
async fn insert(
    State(state): State<ServerState>,
    ValidJson(json): ValidJson<InsertParams>,
) -> AppResult<String> {
    tracing::debug!("开始处理: 添加 Score");
    throw_err!(json.into_active_model().insert(state.db()).await);
    tracing::debug!("创建一条 Score 记录.");
    AppResult::Ok("成功添加 Score 记录!".to_string())
}

#[debug_handler]
async fn update(
    State(state): State<ServerState>,
    Path((stu_id, course_id)): Path<(String, String)>,
    ValidJson(json): ValidJson<InsertParams>,
) -> AppResult<Model> {
    tracing::debug!("开始处理: 更新 Score");
    let target = throw_err!(Score::find_by_id((stu_id, course_id)).one(state.db()).await);
    if let Some(score) = target {
        throw_err!(json.into_active_model().update(state.db()).await);
        tracing::debug!("成功更新了一条 Score 记录.");
        AppResult::Ok(score)
    } else {
        AppResult::Err(AppError::NotFound("没有相关的 Score 记录.".to_string()))
    }
}

#[debug_handler]
async fn delete(
    State(state): State<ServerState>,
    Path((stu_id, course_id)): Path<(String, String)>,
) -> AppResult<String> {
    tracing::debug!("开始处理: 删除 Score");
    let target = throw_err!(
        Score::find_by_id((stu_id.clone(), course_id.clone()))
            .one(state.db())
            .await
    );
    if let Some(score) = target {
        throw_err!(score.delete(state.db()).await);
        AppResult::Ok(format!(
            "成功删除一条 Score 记录, student_id 为 {stu_id}, course_id 为 {course_id}."
        ))
    } else {
        AppResult::Err(AppError::NotFound("没有相关的 Score 记录.".to_string()))
    }
}

#[derive(Validate, Deserialize)]
struct QueryParams {
    student: Option<String>,

    course: Option<String>,

    #[validate(nested)]
    #[serde(flatten)]
    page: PageParam,
}

/// 处理路由到 score 模块下的查询请求
#[debug_handler]
async fn query(
    State(state): State<ServerState>,
    ValidQuery(params): ValidQuery<QueryParams>,
) -> AppResult<Page<Model>> {
    tracing::debug!("开始处理: 查询 score");
    let pagination = Score::find()
        .apply_if(params.student, |rows, keyword| {
            rows.join(
                JoinType::InnerJoin,
                student::Relation::Score
                    .def()
                    .rev()
                    .on_condition(move |_score, student_name| {
                        Expr::col((student_name, student::Column::Name))
                            .like(format!("%{keyword}%"))
                            .into_condition()
                    }),
            )
        })
        .apply_if(params.course, |rows, keyword| {
            rows.join(
                JoinType::InnerJoin,
                course::Relation::Score
                    .def()
                    .rev()
                    .on_condition(move |_score, course_name| {
                        Expr::col((course_name, course::Column::Name))
                            .like(format!("%{keyword}%"))
                            .into_condition()
                    }),
            )
        })
        .paginate(state.db(), params.page.size);

    let total = throw_err!(pagination.num_pages().await);
    let items = throw_err!(pagination.fetch_page(params.page.index - 1).await);

    AppResult::Ok(Page {
        param: params.page,
        total,
        items,
    })
}
