use chrono::NaiveDate;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
    Int,
    Char,
    Boolean,
    Float,
    Varchar(usize),
    Date,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i32),
    Char(char),
    Boolean(bool),
    Float(f32),
    Varchar(String),
    Date(NaiveDate),
    // Add other value types as needed
}

#[derive(Debug, Clone)]
pub struct Column {
    pub name: String,
    pub data_type: DataType,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Row {
    pub values: Vec<Value>,
}

impl Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataType::Int => write!(f, "INT"),
            DataType::Char => write!(f, "CHAR"),
            DataType::Boolean => write!(f, "BOOLEAN"),
            DataType::Float => write!(f, "FLOAT"),
            DataType::Varchar(len) => write!(f, "VARCHAR({})", len),
            DataType::Date => write!(f, "DATE"),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(value) => write!(f, "{}", value),
            Value::Char(value) => write!(f, "{}", value),
            Value::Boolean(value) => write!(f, "{}", value),
            Value::Float(value) => write!(f, "{}", value),
            Value::Varchar(value) => write!(f, "{}", value),
            Value::Date(value) => write!(f, "{}", value),
        }
    }
}

impl Value {
    pub fn from_str(data_type: &DataType, value: &str) -> Result<Value, String> {
        match data_type {
            DataType::Int => value
                .parse::<i32>()
                .map(Value::Int)
                .map_err(|_| format!("Expected integer, got '{}'.", value)),
            DataType::Char => {
                if value.len() != 1 {
                    Err(format!("Expected char, got '{}'.", value))
                } else {
                    Ok(Value::Char(value.chars().next().unwrap()))
                }
            }
            DataType::Boolean => match value.to_lowercase().as_str() {
                "true" | "1" => Ok(Value::Boolean(true)),
                "false" | "0" => Ok(Value::Boolean(false)),
                _ => Err(format!("Expected boolean, got '{}'.", value)),
            },
            DataType::Float => value
                .parse::<f32>()
                .map(Value::Float)
                .map_err(|_| format!(" Expected float, got '{}'.", value)),
            DataType::Date => NaiveDate::parse_from_str(value, "%Y-%m-%d")
                .map(Value::Date)
                .map_err(|_| format!("Expected date (YYYY-MM-DD), got '{}'.", value)),
            DataType::Varchar(len) => {
                if value.len() > *len {
                    Err(format!("Value exceeds maximum length of {}.", len))
                } else {
                    Ok(Value::Varchar(value.to_string()))
                }
            } // Add other data types as needed
        }
    }
}
