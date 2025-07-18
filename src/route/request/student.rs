use crate::entity::prelude::Student;
use crate::entity::student::ActiveModel;
use crate::entity::student::Model;
use crate::entity::{department, student};
use crate::error::AppError;
use crate::route::extract::{Path, ValidJson, ValidQuery};
use crate::route::page::{Page, PageParam};
use crate::route::result::AppResult;
use crate::server::ServerState;
use crate::throw_err;
use axum::extract::State;
use axum::{Router, debug_handler, routing};
use sea_orm::prelude::Expr;
use sea_orm::sea_query::IntoCondition;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DeriveIntoActiveModel, EntityTrait, IntoActiveModel, JoinType,
    ModelTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, QueryTrait, RelationTrait,
};
use serde::Deserialize;
use validator::Validate;

pub fn router() -> Router<ServerState> {
    Router::new()
        .route("/", routing::get(index))
        .route("/query", routing::get(query))
        .route("/insert", routing::post(insert))
        .route("/update/{id}", routing::put(update))
        .route("/delete/{id}", routing::delete(delete))
}

/// 路由到 student 模块下的默认界面
#[debug_handler]
async fn index() -> AppResult<&'static str> {
    AppResult::Ok("欢迎! 这是 Student 的首页.")
}

/// 路由到 student 模块下的 insert 模块时所需的参数
#[derive(Deserialize, Validate, DeriveIntoActiveModel)]
struct InsertParams {
    #[validate(length(min = 1, max = 6))]
    id: String,

    #[validate(length(min = 1, max = 20))]
    name: String,

    #[validate(length(min = 1, max = 2))]
    sex: Option<String>,

    #[validate(range(min = 0))]
    age: Option<i32>,

    #[validate(email)]
    email: Option<String>,

    #[validate(length(max = 2))]
    department_id: Option<String>,
}

#[debug_handler]
async fn insert(
    State(state): State<ServerState>,
    ValidJson(params): ValidJson<InsertParams>,
) -> AppResult<String> {
    tracing::debug!("开始处理: 添加 Student");
    let new_student = params.into_active_model();
    let _result = throw_err!(Student::insert(new_student).exec(state.db()).await);
    AppResult::Ok("成功添加一条 Student 记录!".to_string())
}

#[debug_handler]
async fn update(
    State(state): State<ServerState>,
    Path(id): Path<String>,
    ValidJson(params): ValidJson<InsertParams>,
) -> AppResult<String> {
    tracing::debug!("开始处理: 更新 Student");
    let target = throw_err!(Student::find_by_id(&id).one(state.db()).await);
    if target.is_some() {
        throw_err!(params.into_active_model().update(state.db()).await);
        AppResult::Ok(format!("成功更新一条 id 为 {id} 的 Student 记录!"))
    } else {
        AppResult::Err(AppError::NotFound("没有相关的 Student 记录".to_string()))
    }
}

#[debug_handler]
async fn delete(State(state): State<ServerState>, Path(id): Path<String>) -> AppResult<String> {
    tracing::debug!("开始处理: 删除 student");
    let target = throw_err!(Student::find_by_id(&id).one(state.db()).await);
    if let Some(student) = target {
        throw_err!(student.delete(state.db()).await);
        tracing::info!("已删除 id 为 {id} 的 Student");
        AppResult::Ok(format!("成功删除 id 为 {id} 的学生!"))
    } else {
        AppResult::Err(AppError::NotFound("没有相关的 Student 记录".to_string()))
    }
}

/// 路由到 student 模块下的 query 板块时的所需的参数
#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
struct QueryParams {
    keyword: Option<String>,
    department: Option<String>,

    #[validate(email)]
    email: Option<String>,

    #[validate(length(max = 2))]
    sex: Option<String>,

    #[validate(range(min = 0))]
    age: Option<i32>,

    #[validate(nested)]
    #[serde(flatten)]
    page: PageParam,
}

/// 处理路由到 student 模块下的查询请求
#[debug_handler]
async fn query(
    State(state): State<ServerState>,
    ValidQuery(params): ValidQuery<QueryParams>,
) -> AppResult<Page<Model>> {
    tracing::debug!("开始处理: Query student");
    let pagination = Student::find()
        .apply_if(params.department, |rows, keyword| {
            rows.join(
                JoinType::InnerJoin,
                department::Relation::Student.def().rev().on_condition(
                    move |_student, department_name| {
                        Expr::col((department_name, department::Column::Name))
                            .like(format!("%{keyword}%"))
                            .into_condition()
                    },
                ),
            )
        })
        .apply_if(params.keyword.as_ref(), |rows, keyword| {
            rows.filter(student::Column::Name.contains(keyword))
        })
        .apply_if(params.email.as_ref(), |rows, keyword| {
            rows.filter(student::Column::Email.contains(keyword))
        })
        .apply_if(params.sex.as_ref(), |rows, keyword| {
            rows.filter(student::Column::Sex.eq(keyword))
        })
        .apply_if(params.age, |rows, keyword| {
            rows.filter(student::Column::Age.eq(keyword))
        })
        .order_by_asc(student::Column::Id)
        .paginate(state.db(), params.page.size);

    let total = throw_err!(pagination.num_pages().await);
    let items = throw_err!(pagination.fetch_page(params.page.index - 1).await);

    AppResult::Ok(Page {
        param: params.page,
        total,
        items,
    })
}
