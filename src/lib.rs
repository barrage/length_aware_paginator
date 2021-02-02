//! Lenght aware pagination package that will enable you to easily paginate diesel queries.
//! With given page and per_page size it will automatically calculate last page
//! and will return response with counted data and data for given page.

#[macro_use]
extern crate diesel;

mod model;
mod schema;
mod structs;
mod tests;
mod traits;

const DEFAULT_ITEMS_PER_PAGE: i64 = 15;

pub use structs::Response;
pub use traits::{LoadPaginated, Paginate};
