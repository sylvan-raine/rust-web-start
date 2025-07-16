use axum::{debug_handler, routing, Router};
use axum::extract::State;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DeriveIntoActiveModel, EntityTrait, 
    IntoActiveModel, ModelTrait, PaginatorTrait, QueryFilter, QueryOrder, QueryTrait
};
use serde::Deserialize;
use validator::Validate;
use crate::entity::department;
use crate::entity::department::{Model, ActiveModel};
use crate::entity::prelude::Department;
use crate::error::AppError;
use crate::route::extract::{Path, ValidJson, ValidQuery};
use crate::route::page::{Page, PageParam};
use crate::route::result::AppResult;
use crate::server::ServerState;
use crate::throw_err;

pub fn router() -> Router<ServerState> {
    Router::new()
        .route("/", routing::get(index))
        .route("/query", routing::get(query))
        .route("/insert", routing::post(insert))
        .route("/update/{id}", routing::put(update))
        .route("/delete/{id}", routing::delete(delete))
}

/// 路由到 department 模块下的默认界面
#[debug_handler]
async fn index() -> AppResult<&'static str> {
    AppResult::Ok("欢迎! 这是 Department 的首页.")
}

/// 对 department 进行更改所需的参数
#[derive(Deserialize, Validate, DeriveIntoActiveModel)]
struct InsertParams {
    #[validate(length(min = 1, max = 2))]
    id: String,

    #[validate(length(min = 1, max = 20))]
    name: Option<String>,

    #[validate(length(max = 40))]
    office_room: Option<String>,

    #[validate(length(max = 80))]
    home_page: Option<String>,
}

/// 路由到 department 模块下的 insert 界面
#[debug_handler]
async fn insert(
    State(state): State<ServerState>,
    ValidJson(params): ValidJson<InsertParams>
) -> AppResult<String> {
    tracing::debug!("开始处理: 添加 Department");
    throw_err!(params.into_active_model().insert(state.db()).await);
    AppResult::Ok("成功添加一条 Department 记录!".to_string())
}

/// 路由到 department 模块下的 update 界面
#[debug_handler]
async fn update(
    State(state): State<ServerState>,
    Path(id): Path<String>,
    ValidJson(params): ValidJson<InsertParams>
) -> AppResult<String> {
    tracing::debug!("开始处理: 修改 Department");
    let target = throw_err!(Department::find_by_id(&id).one(state.db()).await);
    if let Some(_) = target {
        throw_err!(params.into_active_model().update(state.db()).await);
        AppResult::Ok("成功修改一条 Department 数据!".to_string())
    } else {
        AppResult::Err(AppError::NotFound("没有相关的 Department 记录!".to_string()))
    }
}

/// 路由到 department 模块下的 delete 页面
#[debug_handler]
async fn delete(
    State(state): State<ServerState>,
    Path(id): Path<String>,
) -> AppResult<String> {
    tracing::debug!("开始处理: 删除 Department");
    let target = throw_err!(Department::find_by_id(&id).one(state.db()).await);
    if let Some(department) = target {
        throw_err!(department.delete(state.db()).await);
        tracing::info!("已删除一条 id 为 {id} 的 Department 记录");
        AppResult::Ok(format!("成功删除一条 id 为 {id} 的 Department 记录!"))
    } else {
        AppResult::Err(AppError::NotFound("没有相关的 Department 记录!".to_string()))
    }
}

/// 路由到 department 模块下的 query 板块时的所需的参数
#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
struct QueryParams {
    keyword: Option<String>,

    #[validate(length(max = 40))]
    office_room: Option<String>,

    #[validate(length(max = 80))]
    home_page: Option<String>,

    #[validate(nested)]
    #[serde(flatten)]
    page: PageParam
}

/// 处理路由到 department 模块下的查询请求
#[debug_handler]
async fn query(
    State(state): State<ServerState>,
    ValidQuery(params): ValidQuery<QueryParams>
) -> AppResult<Page<Model>> {
    tracing::debug!("开始处理: 查询 Department");
    let pagination = Department::find()
        .apply_if(params.keyword.as_ref(), |rows, keyword| {
            rows.filter(department::Column::Name.contains(keyword))
        })
        .apply_if(params.office_room.as_ref(), |rows, keyword| {
            rows.filter(department::Column::OfficeRoom.eq(keyword))
        })
        .apply_if(params.home_page.as_ref(), |rows, keyword| {
            rows.filter(department::Column::HomePage.eq(keyword))
        })
        .order_by_asc(department::Column::Id)
        .paginate(state.db(), params.page.size);

    let total = throw_err!(pagination.num_pages().await);
    let items = throw_err!(pagination.fetch_page(params.page.index - 1).await);

    AppResult::Ok(Page { param: params.page, total, items })
}