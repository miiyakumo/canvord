use apistos::ApiComponent;
use schemars::JsonSchema;

#[derive(Debug, Clone, serde::Serialize, JsonSchema, ApiComponent)]
pub struct PageResult<T: JsonSchema> {
    pub total: usize,   // 数据总量
    pub current: usize, // 当前页数（从 1 开始）
    pub size: usize,    // 每页大小
    pub data: Vec<T>,   // 当前页的数据
}