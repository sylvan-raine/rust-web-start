use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Deserializer, Serialize};
use validator::Validate;

#[derive(Serialize)]
pub struct Page<T: Serialize> {
    #[serde(flatten)]
    pub param: PageParam,
    pub total: u64,
    pub items: Vec<T>
}

#[derive(Serialize, Deserialize, Validate, Clone, Copy)]
pub struct PageParam {
    /// 代表现在是第几页
    #[validate(range(min = 1, message = "页码应大于等于 1."))]
    #[serde(default = "PageParam::default_index", deserialize_with = "from_str")]
    pub index: u64,
    
    /// 代表一页能容纳多少条数据
    #[validate(range(min = 1, max = 100, message = "每页所含信息应在 1 条至 100 条之间."))]
    #[serde(default = "PageParam::default_size", deserialize_with = "from_str")]
    pub size: u64,
}

impl PageParam {
    const DEFAULT_PAGE_INDEX: u64 = 1;
    const DEFAULT_PAGE_SIZE: u64 = 50;

    fn default_index() -> u64 {
        PageParam::DEFAULT_PAGE_INDEX
    }

    fn default_size() -> u64 {
        PageParam::DEFAULT_PAGE_SIZE
    }
}

fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr,
    T::Err: Display,
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(serde::de::Error::custom)
}