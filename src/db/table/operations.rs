use super::Insert;
use super::Row;
use super::Table;
use super::Value;

impl Table {
    pub fn insert(&mut self, insert: Insert) -> Result<(), String> {
        let row = self.convert_insert_values(insert.values, None)?;
        self.rows.push(Row { values: row });
        Ok(())
    }

    pub fn select(&self) -> Result<Vec<Row>, String> {
        // Implement the logic for selecting data from the table
        //
        // For now, we'll return all rows in the table
        Ok(self.rows.clone())
    }

    fn convert_insert_values(
        &self,
        values: Vec<String>,
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
