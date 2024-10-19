use std::fmt::Display;

use super::data::{Column, Row, Value};
use super::statement::Insert;

pub mod operations;

#[derive(Debug)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
    pub rows: Vec<Row>,
}

impl Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Table: {}", self.name)?;
        for column in &self.columns {
            write!(f, "  {} ({:?}), ", column.name, column.data_type)?;
        }
        writeln!(f, "\n  Total {:?} records.", self.rows.len())?;
        Ok(())
    }
}
