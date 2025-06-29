use axum::{debug_handler, routing, Router};
use axum::extract::State;
use sea_orm::{ColumnTrait, EntityTrait, QueryTrait, QueryFilter, QueryOrder, PaginatorTrait, DeriveIntoActiveModel, IntoActiveModel, ActiveModelTrait, ModelTrait};
use serde::Deserialize;
use validator::Validate;
use crate::entity::prelude::Student;
use crate::entity::student;
use crate::entity::student::Model;
use crate::entity::student::ActiveModel;
use crate::error::AppError;
use crate::route::extract::{Path, ValidJson, ValidQuery};
use crate::route::not_found;
use crate::route::result::{Page, AppResult};
use crate::server::ServerState;
use crate::throw_err;

pub fn router() -> Router<ServerState> {
    Router::new()
        .route("/", routing::get(index))
        .route("/query", routing::get(query))
        .route("/insert", routing::post(insert))
        .route("/update/{id}", routing::put(update))
        .route("/delete/{id}", routing::delete(delete))
        .fallback(not_found)
}

/// 路由到 student 模块下的默认界面
async fn index() -> AppResult<&'static str> {
    AppResult::Ok("Welcome! This is the index page of student.")
}

/// 路由到 student 模块下的 insert 模块时所需的参数
#[derive(Deserialize, Validate, DeriveIntoActiveModel)]
struct InsertParams {
    #[validate(length(min = 1, max = 6))]
    id: String,

    #[validate(length(min = 1, max = 20))]
    name: String,

    #[validate(length(min = 1, max = 2))]
    sex: String,

    #[validate(range(min = 0))]
    age: i32,

    #[validate(email)]
    email: String,

    #[validate(length(max = 2))]
    department_id: String,
}

#[debug_handler]
async fn insert(
    State(state): State<ServerState>,
    ValidJson(params): ValidJson<InsertParams>
) -> AppResult<String> {
    let new_student = params.into_active_model();
    let _result = throw_err!(Student::insert(new_student).exec(state.db()).await);
    AppResult::Ok("Successfully inserted a student".to_string())
}

#[debug_handler]
async fn update(
    State(state): State<ServerState>,
    Path(id): Path<String>,
    ValidJson(params): ValidJson<InsertParams>
) -> AppResult<String> {
    let result = throw_err!(Student::find_by_id(&id).one(state.db()).await);
    if let None = result {
        AppResult::Err(AppError::Internal("No specified student found".to_string()))
    } else {
        let active_model = params.into_active_model();
        throw_err!(active_model.update(state.db()).await);
        AppResult::Ok(format!("Successfully updated {id}"))
    }
}

#[debug_handler]
async fn delete(
    State(state): State<ServerState>,
    Path(id): Path<String>,
) -> AppResult<String> {
    let target = throw_err!(Student::find_by_id(&id).one(state.db()).await);
    if let None = target {
        AppResult::Err(AppError::Internal("No specified student found".to_string()))
    } else {
        throw_err!(target.unwrap().delete(state.db()).await);
        tracing::info!("deleted student, id: {id}");
        AppResult::Ok(format!("Successfully deleted {id}"))
    }
}

/// 路由到 student 模块下的 query 板块时的所需的参数
#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
struct QueryParams {
    keyword: Option<String>,

    #[validate(range(min = 1, message = "Page index should be less than or equal to 1."))]
    #[serde(default = "QueryParams::default_page_index")]
    page_index: u64,

    #[validate(range(min = 5, max = 100, message = "The amount of items in one page should be at least 5 and at most 100."))]
    #[serde(default = "QueryParams::default_page_size")]
    page_size: u64
}

impl QueryParams {
    const DEFAULT_PAGE_SIZE: u64 = 20;
    const DEFAULT_PAGE_INDEX: u64 = 1;
    fn default_page_size() -> u64 {
        QueryParams::DEFAULT_PAGE_SIZE
    }

    fn default_page_index() -> u64 {
        QueryParams::DEFAULT_PAGE_INDEX
    }
}


/// 处理路由到 student 模块下的查询请求
#[debug_handler]
async fn query(
    State(state): State<ServerState>,
    ValidQuery(param): ValidQuery<QueryParams>
) -> AppResult<Page<Model>> {
    tracing::debug!("Query student");
    let stu_sql = Student::find()
        .apply_if(param.keyword.as_ref(), |q, k| {
            q.filter(student::Column::Name.contains(k))
        })
        .order_by_asc(student::Column::Id)
        .paginate(state.db(), param.page_size);

    let amount = throw_err!(stu_sql.num_items().await);
    let items = throw_err!(stu_sql.fetch_page(param.page_index - 1).await);

    AppResult::Ok(
        Page {
            page_index: param.page_index,
            page_size: param.page_size,
            total_pages: amount / param.page_size + 1,
            items
        }
    )
}