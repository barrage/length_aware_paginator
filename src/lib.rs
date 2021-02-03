//! Lenght aware paginator enables you to paginate Diesel queries and have information about the lenght
//! of data being paginated. It will give you total number of items, and last page that you can navigate to
//! and still get some kind of data.
//!
//! You will only have to provide page and per_page parameters.
//!
//! ```ignore
//! use diesel::pg::PgConnection;
//! use diesel::Connection;
//! use diesel::QueryDsl;
//! use lenght_aware_paginator::{LoadPaginated, Response};
//! use serde::{Deserialize, Serialize};
//!
//! /// Get the database connection
//! /// *panics* if no DATABASE_URL is defined in the env or if the db is unreachable
//! fn get_connection() -> PgConnection {
//!     let database_url =
//!         dotenv::var("DATABASE_URL").expect("You have to provide DATABASE_URL to run tests");
//!
//!     PgConnection::establish(&database_url)
//!         .unwrap_or_else(|_| panic!("Error connecting to {}", &database_url))
//! }
//!
//! // schema.rs : autogenerated by diesel after running migration
//! table! {
//!     users (id) {
//!         id -> Int4,
//!         email -> Varchar,
//!         first_name -> Varchar,
//!         last_name -> Varchar,
//!         password -> Varchar,
//!     }
//! }
//!
//! // user.rs : your model for the table represented in schema.rs
//! #[derive(Queryable, Deserialize, Serialize)]
//! pub struct User {
//!     id: i32,
//!     email: String,
//!     first_name: String,
//!     last_name: String,
//!     password: String,
//! }
//!
//! #[test]
//! fn test_orm_query_pagination() {
//!     let connection = get_connection();
//!
//!     // Use `lenght_aware_paginator::LoadPaginated` trait to enable
//!     // using the `load_paginated` method on your query.
//!     // Your query will return `lenght_aware_paginator::Response<T>` struct
//!     let response: Response<User> = schema::users::table
//!         .into_boxed()
//!         .load_paginated(connection, page, per_page)
//!         .unwrap();
//!
//!     assert_eq!(response.page, 1);
//!     assert_eq!(response.per_page, 10);
//!     assert_eq!(response.total, 15);
//!     assert_eq!(response.last_page, 2);
//!     assert_eq!(response.data.len(), 10);
//! }
//! ```
//!
//! ### Limitations
//!
//! Unfortunatelly this is still not implemented to work with `sql_query()` due to its own limitations.
//! Maybe in the future I'll update this package to enable this.

#[macro_use]
extern crate diesel;

mod structs;
mod traits;

pub use structs::Response;
pub use traits::{LoadPaginated, Paginate};