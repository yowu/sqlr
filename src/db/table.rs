use super::data::{Column, Row, Value};
use super::page::Page;
use std::fmt::Display;

#[derive(Debug)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
    pages: Vec<Page>,
}

impl Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Table: {}", self.name)?;
        for column in &self.columns {
            write!(f, "  {} ({:?}), ", column.name, column.data_type)?;
        }
        let total_rows = self.pages.iter().map(|page| page.num_rows()).sum::<usize>();
        writeln!(f, "\n  Total {:?} records.", total_rows)?;
        Ok(())
    }
}

impl Table {
    pub fn new(name: String, columns: Vec<Column>) -> Self {
        Self {
            name,
            columns,
            pages: Vec::new(),
        }
    }

    pub fn insert(&mut self, values: &Vec<String>) -> Result<(), String> {
        let row = self.convert_insert_values(values, None)?;
        let page: &mut Page = self.find_or_create_page();
        page.insert_row(Row { values: row })?;
        Ok(())
    }

    fn find_or_create_page(&mut self) -> &mut Page {
        let mut page_index = None;
        for (index, page) in self.pages.iter_mut().enumerate() {
            if !page.is_full() {
                page_index = Some(index);
                break;
            }
        }

        if !page_index.is_some() {
            self.pages.push(Page::new());
            page_index = Some(self.pages.len() - 1);
        }

        let page = self.pages.get_mut(page_index.unwrap()).unwrap();
        page
    }

    pub fn select(&self) -> Result<Vec<Row>, String> {
        // Implement the logic for selecting data from the table
        //
        // For now, we'll return all rows in the table
        let mut rows = Vec::new();
        for page in &self.pages {
            page.rows.iter().for_each(|row| {
                if let Some(row) = row {
                    rows.push(row.clone());
                }
            });
        }
        Ok(rows)
    }

    fn convert_insert_values(
        &self,
        values: &Vec<String>,
        columns: Option<Vec<String>>,
    ) -> Result<Vec<Value>, String> {
        let columns_to_use = match columns {
            Some(cols) => cols,
            None => self.columns.iter().map(|col| col.name.clone()).collect(),
        };

        if values.len() != columns_to_use.len() {
            return Err(format!(
                "Column count doesn't match value count. Expected {}, got {}.",
                columns_to_use.len(),
                values.len()
            ));
        }

        let mut transformed_values = Vec::new();

        for (value, column_name) in values.iter().zip(&columns_to_use) {
            let column = self
                .columns
                .iter()
                .find(|col| &col.name == column_name)
                .ok_or_else(|| format!("Column '{}' not found in table schema.", column_name))?;
            let transformed_value = Value::from_str(&column.data_type, value).map_err(|e| {
                format!("Error converting value for column '{}': {}", column.name, e)
            })?;
            transformed_values.push(transformed_value);
        }

        Ok(transformed_values)
    }
}
