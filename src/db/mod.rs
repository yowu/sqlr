pub mod command;
pub mod data;
pub mod operations;
pub mod statement;
pub mod table;

use std::collections::HashMap;

#[derive(Debug)]
pub struct Database {
    pub tables: HashMap<String, table::Table>,
}
