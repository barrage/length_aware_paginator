[![Crates.io](https://img.shields.io/crates/v/lenght_aware_paginator.svg)](https://crates.io/crates/lenght_aware_paginator)

# lenght_aware_paginator

```toml
lenght_aware_paginator = "0.1.0"
```

Lenght aware paginator enables you to paginate Diesel queries and have information about the lenght
of data being paginated. It will give you total number of items, and last page that you can navigate to
and still get some kind of data.

You will only have to provide page and per_page parameters.

```rust
use diesel::pg::PgConnection;
use diesel::Connection;
use diesel::QueryDsl;
use lenght_aware_paginator::{LoadPaginated, Response};
use serde::{Deserialize, Serialize};

// user.rs : your model for the table represented in schema.rs
#[derive(Queryable, Deserialize, Serialize)]
pub struct User {
    id: i32,
    email: String,
    first_name: String,
    last_name: String,
    password: String,
}

fn get_connection() -> PgConnection {
    let database_url =
        dotenv::var("DATABASE_URL").expect("You have to provide DATABASE_URL to run tests");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", &database_url))
}

#[test]
fn test_orm_query_pagination() {
    let connection = get_connection();

    // Use `lenght_aware_paginator::LoadPaginated` trait to enable
    // using the `load_paginated` method on your query.
    // Your query will return `lenght_aware_paginator::Response<T>` struct
    let response: Response<User> = schema::users::table
        .into_boxed()
        .load_paginated(connection, page, per_page)
        .unwrap();

    assert_eq!(response.page, 1);
    assert_eq!(response.per_page, 10);
    assert_eq!(response.total, 15);
    assert_eq!(response.last_page, 2);
    assert_eq!(response.data.len(), 10);
}
```

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
