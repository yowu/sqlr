use std::fmt::Display;

#[derive(Debug)]
pub enum DataType {
    Int,
    Char,
    Boolean,
    Float,
    Varchar(usize),
    Date,
}

#[derive(Debug)]
pub enum Statement {
    CreateTable(CreateTable),
    Select(Select),
    // Add other statement types here
}

#[derive(Debug)]
pub struct CreateTable {
    pub table_name: String,
    pub columns: Vec<ColumnDefinition>,
}

#[derive(Debug)]
pub struct ColumnDefinition {
    pub name: String,
    pub data_type: DataType,
}

#[derive(Debug)]
pub struct Select {
    pub columns: Vec<String>,
    pub table_name: String,
    // Add other SELECT statement components here
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

impl Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataType::Int => write!(f, "INT"),
            DataType::Char => write!(f, "CHAR"),
            DataType::Boolean => write!(f, "BOOLEAN"),
            DataType::Float => write!(f, "FLOAT"),
            DataType::Varchar(size) => write!(f, "VARCHAR({})", size),
            DataType::Date => write!(f, "DATE"),
        }
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

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::CreateTable(create_table) => write!(f, "{}", create_table),
            Statement::Select(select) => write!(f, "{}", select),
        }
    }
}
