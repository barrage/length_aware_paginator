use serde::{Deserialize, Serialize};

#[derive(QueryId)]
pub struct PaginatedQuery<T> {
    pub query: T,
    pub page: i64,
    pub per_page: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Response<T> {
    pub page: i64,
    pub per_page: i64,
    pub total: i64,
    pub last_page: i64,
    pub data: Vec<T>,
}
