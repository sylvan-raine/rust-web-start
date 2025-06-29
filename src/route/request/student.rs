use axum::{debug_handler, routing, Router};
use axum::extract::State;
use sea_orm::{ColumnTrait, EntityTrait, QueryTrait, QueryFilter, QueryOrder, PaginatorTrait};
use serde::Deserialize;
use validator::Validate;
use crate::entity::prelude::Student;
use crate::entity::student;
use crate::entity::student::Model;
use crate::route::extract::ValidQuery;
use crate::route::not_found;
use crate::route::result::{Page, QueryResult};
use crate::server::ServerState;

pub fn router() -> Router<ServerState> {
    Router::new()
        .route("/", routing::get(index))
        .route("/query", routing::get(query))
        .fallback(not_found)
}

/// 路由到 student 模块下的默认界面
async fn index() -> QueryResult<&'static str> {
    QueryResult::Ok("Welcome! This is the index page of student.")
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
async fn query(State(state): State<ServerState>, ValidQuery(param): ValidQuery<QueryParams>) -> QueryResult<Page<Model>> {
    tracing::debug!("Query student");
    let stu_sql = Student::find()
        .apply_if(param.keyword.as_ref(), |q, k| {
            q.filter(student::Column::Name.contains(k))
        })
        .order_by_asc(student::Column::Id)
        .paginate(state.db(), param.page_size);

    let amount = match stu_sql.num_items().await {
        Ok(a) => a,
        Err(e) => return QueryResult::from(e),
    };
    let items = match stu_sql.fetch_page(param.page_index - 1).await {
        Ok(a) => a,
        Err(e) => return QueryResult::from(e),
    };

    QueryResult::Ok(
        Page {
            page_index: param.page_index,
            page_size: param.page_size,
            total_pages: amount / param.page_size + 1,
            items
        }
    )
}