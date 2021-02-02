#[cfg(test)]
mod test {
    use crate::diesel::QueryDsl;
    use crate::model::User;
    use crate::schema::users;
    use crate::structs::Response;
    use crate::traits::LoadPaginated;
    use diesel::pg::PgConnection;
    use diesel::Connection;

    fn get_connection() -> diesel::pg::PgConnection {
        let database_url =
            dotenv::var("DATABASE_URL").expect("You have to provide DATABASE_URL to run tests");

        PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", &database_url))
    }

    #[test]
    fn test_orm_query_pagination() {
        let connection = get_connection();

        let response: Response<User> = users::table
            .into_boxed()
            .load_paginated(&connection, Some(1), Some(2))
            .unwrap();

        assert_eq!(response.page, 1);
    }

    // TODO: Figure out a way to make this happen...
    // #[test]
    // fn test_sql_query_pagination() {
    //     let connection = get_connection();

    //     let response: Response<User> = diesel::sql_query("SELECT * FROM users")
    //         .load_paginated(&connection, Some(1), Some(2))
    //         .unwrap();

    //     assert_eq!(response.page, 1);
    // }
}
