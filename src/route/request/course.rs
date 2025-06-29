use crate::entity::course;
use crate::entity::course::ActiveModel;
use crate::entity::course::Model;
use crate::entity::prelude::Course;
use crate::error::AppError;
use crate::route::extract::{Path, ValidJson, ValidQuery};
use crate::route::not_found;
use crate::route::page::{Page, PageParam};
use crate::route::result::AppResult;
use crate::server::ServerState;
use crate::throw_err;
use axum::extract::State;
use axum::{Router, debug_handler, routing};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DeriveIntoActiveModel, EntityTrait, IntoActiveModel, ModelTrait,
    PaginatorTrait, QueryFilter, QueryOrder, QueryTrait,
};
use serde::Deserialize;
use validator::Validate;

pub fn router() -> Router<ServerState> {
    Router::new()
        .route("/", routing::get(index))
        .route("/query", routing::get(query))
        .route("/insert", routing::post(insert))
        .route("/update", routing::put(update))
        .route("/delete", routing::delete(delete))
        .fallback(not_found)
}

/// 路由到 course 模块下的默认界面
async fn index() -> AppResult<&'static str> {
    AppResult::Ok("Welcome! This is the index page of course.")
}

/// 插入新的课程数据所需要的参数
#[derive(Deserialize, Validate, DeriveIntoActiveModel)]
struct InsertParam {
    #[validate(length(max = 6))]
    id: String,

    #[validate(length(max = 20))]
    name: String,

    #[validate(length(max = 6))]
    pre_course: Option<String>,

    #[validate(range(min = 0))]
    credit: Option<i32>,

    #[validate(length(max = 2))]
    department_id: Option<String>,
}

/// 处理路由到 course 模块下的 insert 界面
async fn insert(
    State(state): State<ServerState>,
    ValidJson(json): ValidJson<InsertParam>,
) -> AppResult<String> {
    let new_course = json.into_active_model();
    throw_err!(new_course.insert(state.db()).await);
    AppResult::Ok("Successfully inserted course!".to_string())
}

/// 处理路由到 course 模块下的 update 请求
async fn update(
    State(state): State<ServerState>,
    Path(id): Path<String>,
    ValidJson(json): ValidJson<InsertParam>,
) -> AppResult<String> {
    let target = throw_err!(Course::find_by_id(&id).one(state.db()).await);
    if let Some(_) = target {
        throw_err!(json.into_active_model().update(state.db()).await);
        AppResult::Ok("Successfully updated course!".to_string())
    } else {
        AppResult::Err(AppError::Internal("No specified course found!".to_string()))
    }
}

/// 处理路由到 course 模块下的 delete 请求
async fn delete(State(state): State<ServerState>, Path(id): Path<String>) -> AppResult<String> {
    let target = throw_err!(Course::find_by_id(&id).one(state.db()).await);
    if let Some(course) = target {
        throw_err!(course.delete(state.db()).await);
        tracing::info!("Deleted course, {id}");
        AppResult::Ok("Successfully deleted course!".to_string())
    } else {
        AppResult::Err(AppError::Internal("No specified course found!".to_string()))
    }
}

#[derive(Deserialize, Validate)]
struct QueryParam {
    #[validate(length(max = 6))]
    name: Option<String>,

    #[validate(length(max = 6))]
    pre_course: Option<String>,

    #[validate(range(min = 0))]
    credit: Option<u32>,

    #[validate(nested)]
    #[serde(flatten)]
    page: PageParam,
}

/// 处理路由到 course 模块下的查询请求
#[debug_handler]
async fn query(
    State(state): State<ServerState>,
    ValidQuery(params): ValidQuery<QueryParam>,
) -> AppResult<Page<Model>> {
    tracing::debug!("Query course");
    let course = Course::find()
        .apply_if(params.name.as_ref(), |rows, key| {
            rows.filter(course::Column::Name.contains(key))
        })
        .apply_if(params.pre_course.as_ref(), |rows, course| {
            rows.filter(course::Column::PreCourse.eq(course))
        })
        .apply_if(params.credit, |rows, course| {
            rows.filter(course::Column::Credit.eq(course))
        })
        .order_by_asc(course::Column::Id)
        .paginate(state.db(), params.page.size);

    let amount = throw_err!(course.num_items().await);
    let items = throw_err!(course.fetch_page(params.page.index - 1).await);

    AppResult::Ok(Page {
        param: params.page,
        total: amount / params.page.size + 1,
        items,
    })
}
