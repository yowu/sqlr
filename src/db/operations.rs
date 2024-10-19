use super::data::{Column, Row};
use super::statement::{CreateTable, Insert, Select};
use super::Database;
use std::collections::HashMap;

use super::table::Table;

impl Database {
    pub fn new() -> Self {
        Database {
            tables: HashMap::new(),
        }
    }

    pub fn create_table(&mut self, create_table: CreateTable) {
        let table = Table {
            name: create_table.table_name.clone(),
            columns: create_table.columns,
            rows: Vec::new(),
        };
        self.tables.insert(create_table.table_name, table);
    }

    /*
    pub fn drop_table(&mut self, table_name: &str) -> Result<(), String> {
        if self.tables.remove(table_name).is_some() {
            Ok(())
        } else {
            Err(format!("Table '{}' does not exist.", table_name))
        }
    }
    */

    pub fn insert_into_table(&mut self, insert: Insert) -> Result<(), String> {
        if let Some(table) = self.tables.get_mut(&insert.table_name) {
            table.insert(insert)
        } else {
            Err(format!("Table '{}' does not exist.", insert.table_name))
        }
    }

    pub fn select_from_table(&self, select: Select) -> Result<Vec<Row>, String> {
        if let Some(table) = self.tables.get(&select.table_name) {
            table.select()
        } else {
            Err(format!("Table '{}' does not exist.", select.table_name))
        }
    }

    pub fn get_table_columns(&self, table_name: &str) -> Result<Vec<Column>, String> {
        self.tables
            .get(table_name)
            .map(|table| table.columns.clone())
            .ok_or_else(|| format!("Table '{}' does not exist.", table_name))
    }
}
