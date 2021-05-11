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

impl<T> Response<T> {
    pub fn from<A>(source: Response<A>) -> Self
    where
        T: From<A>,
        A: Clone,
    {
        Response {
            page: source.page,
            per_page: source.per_page,
            total: source.total,
            last_page: source.last_page,
            data: source.data.into_iter().map(T::from).collect::<Vec<T>>(),
        }
    }
}
