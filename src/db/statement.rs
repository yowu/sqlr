use std::fmt::Display;

use super::data::Column;

#[derive(Debug)]
pub enum Statement {
    CreateTable(CreateTable),
    Select(Select),
    Insert(Insert),
    // Add other statement types here
}

#[derive(Debug)]
pub struct CreateTable {
    pub table_name: String,
    pub columns: Vec<Column>,
}

#[derive(Debug)]
pub struct Select {
    pub table_name: String,
    pub columns: Vec<String>,
    // Add other SELECT statement components here
}

#[derive(Debug)]
pub struct Insert {
    pub table_name: String,
    pub values: Vec<String>,
}

impl Display for CreateTable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "CREATE TABLE {} (", self.table_name)?;
        for column in &self.columns {
            writeln!(f, "  {} {},", column.name, column.data_type)?;
        }
        write!(f, ")")
    }
}

impl Display for Select {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SELECT ")?;
        for column in &self.columns {
            write!(f, "{}, ", column)?;
        }
        write!(f, "FROM {}", self.table_name)
    }
}

impl Display for Insert {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "INSERT INTO {} VALUES ({})",
            self.table_name,
            self.values.join(", ")
        )
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::CreateTable(create_table) => write!(f, "{}", create_table),
            Statement::Select(select) => write!(f, "{}", select),
            Statement::Insert(insert) => write!(f, "{}", insert),
        }
    }
}
