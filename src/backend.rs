mod table;
mod pager;

use crate::backend::pager::Pager;
pub use crate::backend::table::Table;

pub fn db_open(file_name: Option<&str>) -> Table {
    let pager = Pager::new(
        file_name.unwrap_or_else(|| "./database"),
    );

    Table::new(pager)
}
