use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct User {
    id: String,
    email: String,
    first_name: String,
    last_name: String,
    password: String,
}
