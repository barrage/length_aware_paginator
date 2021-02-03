use super::structs::{PaginatedQuery, Response};
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::query_builder::*;
use diesel::query_dsl::methods::LoadQuery;
use diesel::sql_types::{BigInt, HasSqlType};

pub trait Paginate: Sized {
    fn page(self, page: i64) -> PaginatedQuery<Self>;
}

impl<T> Paginate for T {
    fn page(self, page: i64) -> PaginatedQuery<Self> {
        PaginatedQuery {
            query: self,
            per_page: 15,
            page,
        }
    }
}

impl<T> QueryFragment<Pg> for PaginatedQuery<T>
where
    T: QueryFragment<Pg>,
{
    fn walk_ast(&self, mut out: AstPass<Pg>) -> QueryResult<()> {
        out.push_sql("SELECT *, COUNT(*) OVER () FROM (");
        self.query.walk_ast(out.reborrow())?;
        out.push_sql(") t LIMIT ");
        out.push_bind_param::<BigInt, _>(&self.per_page)?;
        out.push_sql(" OFFSET ");
        let offset = (self.page - 1) * self.per_page;
        out.push_bind_param::<BigInt, _>(&offset)?;
        Ok(())
    }
}

impl<T: Query> Query for PaginatedQuery<T> {
    type SqlType = (T::SqlType, BigInt);
}

impl<T> RunQueryDsl<PgConnection> for PaginatedQuery<T> {}

impl<T> PaginatedQuery<T> {
    /// Set per page size on the query
    pub fn per_page(self, per_page: i64) -> Self {
        PaginatedQuery { per_page, ..self }
    }

    /// Load paginated data
    pub fn load_paginated<U>(self, conn: &PgConnection) -> QueryResult<Response<U>>
    where
        Self: LoadQuery<PgConnection, (U, i64)>,
    {
        let page = self.page;
        let per_page = self.per_page;
        let results = self.load::<(U, i64)>(conn)?;
        let total = results.get(0).map(|x| x.1).unwrap_or(0);
        let data = results.into_iter().map(|x| x.0).collect();
        let mut last_page = 1;

        if total > 0 {
            last_page = (total as f64 / per_page as f64).ceil() as i64;
        }

        Ok(Response {
            page,
            per_page,
            total,
            last_page,
            data,
        })
    }
}

pub trait LoadPaginated<U>:
    Query + QueryId + QueryFragment<Pg> + LoadQuery<PgConnection, U>
{
    fn load_paginated(
        self,
        conn: &PgConnection,
        page: Option<i64>,
        per_page: Option<i64>,
    ) -> QueryResult<Response<U>>;
}

impl<T, U> LoadPaginated<U> for T
where
    Self: Query + QueryId + QueryFragment<Pg> + LoadQuery<PgConnection, U>,
    U: Queryable<Self::SqlType, Pg>,
    Pg: HasSqlType<Self::SqlType>,
{
    fn load_paginated(
        self,
        conn: &PgConnection,
        page: Option<i64>,
        per_page: Option<i64>,
    ) -> QueryResult<Response<U>> {
        let page = page.unwrap_or(1);

        let mut query = self.page(page);

        if let Some(per_page) = per_page {
            query = query.per_page(per_page);
        }

        query.load_paginated::<U>(conn)
    }
}
