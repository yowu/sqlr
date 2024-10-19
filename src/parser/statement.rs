use std::iter::Peekable;

use super::token::Token;
use crate::db::data::{Column, DataType};
use crate::db::statement::{CreateTable, Insert, Select, Statement};

pub fn parse_create<'a>(
    iter: &mut Peekable<impl Iterator<Item = &'a Token>>,
) -> Result<Statement, String> {
    iter.next(); // Consume "CREATE"
    match iter.peek() {
        Some(Token::Keyword(keyword)) if keyword.to_uppercase() == "TABLE" => {
            parse_create_table(iter)
        }
        _ => Err("Unknow create statement".to_string()),
    }
}

/*
```
<create_table> ::= "CREATE" "TABLE" <identifier> "(" <column_definitions> ")"
<column_definitions> ::= <column_definition> ("," <column_definition>)*
<column_definition> ::= <identifier> <data_type>
<data_type> ::= "INT" | "VARCHAR" "(" <number> ")" | "DATE" | "DOUBLE"
<identifier> ::= [a-zA-Z_][a-zA-Z0-9_]*
<number> ::= [0-9]+
```
*/

pub fn parse_create_table<'a>(
    iter: &mut Peekable<impl Iterator<Item = &'a Token>>,
) -> Result<Statement, String> {
    iter.next(); // Consume "TABLE"

    // Expect table name
    let table_name = match iter.next() {
        Some(Token::Identifier(name)) => name.clone(),
        _ => return Err("Expected table name".to_string()),
    };

    // Expect "("
    match iter.next() {
        Some(Token::Punctuation('(')) => {}
        _ => return Err("Expected '('".to_string()),
    }

    // Parse column definitions
    let mut columns = Vec::new();
    loop {
        // Expect column name
        let column_name = match iter.next() {
            Some(Token::Identifier(name)) => name.clone(),
            _ => return Err("Expected column name".to_string()),
        };

        // Expect data type
        let data_type = parse_data_type(iter)?;

        columns.push(Column {
            name: column_name,
            data_type,
        });

        // Check for "," or ")"
        match iter.peek() {
            Some(Token::Punctuation(',')) => {
                iter.next(); // Consume ","
                continue;
            }
            Some(Token::Punctuation(')')) => {
                iter.next(); // Consume ")"
                break;
            }
            _ => return Err("Expected ',' or ')'".to_string()),
        }
    }

    Ok(Statement::CreateTable(CreateTable {
        table_name,
        columns,
    }))
}

fn parse_data_type<'a>(
    iter: &mut Peekable<impl Iterator<Item = &'a Token>>,
) -> Result<DataType, String> {
    match iter.next() {
        Some(Token::DataType(data_type)) => match data_type.to_uppercase().as_str() {
            "INT" | "INTEGER" => Ok(DataType::Int),
            "DATE" => Ok(DataType::Date),
            "FLOAT" => Ok(DataType::Float),
            "CHAR" => Ok(DataType::Char),
            "BOOLEAN" | "BOOL" => Ok(DataType::Boolean),
            "VARCHAR" => {
                match iter.next() {
                    Some(Token::Punctuation('(')) => {}
                    _ => return Err("Expected '(' after VARCHAR".to_string()),
                }

                // Expect number
                let size = match iter.next() {
                    Some(Token::Numeric(size)) => size
                        .parse::<usize>()
                        .map_err(|_| "Invalid size in VARCHAR".to_string())?,
                    _ => return Err("Expected size in VARCHAR".to_string()),
                };

                // Expect ")"
                match iter.next() {
                    Some(Token::Punctuation(')')) => {}
                    _ => return Err("Expected ')' after VARCHAR size".to_string()),
                }

                Ok(DataType::Varchar(size))
            }
            _ => Err("Unknown data type".to_string()),
        },
        Some(_) => Err("Expected data type".to_string()),
        None => Err("Unexpected end of input".to_string()),
    }
}

pub fn parse_select<'a>(
    iter: &mut Peekable<impl Iterator<Item = &'a Token>>,
) -> Result<Statement, String> {
    iter.next(); // Consume "SELECT"

    let mut columns = Vec::new();
    loop {
        match iter.next() {
            Some(Token::Identifier(column)) => columns.push(column.clone()),
            Some(Token::Punctuation(',')) => continue,
            Some(Token::Keyword(keyword)) if keyword.to_uppercase() == "FROM" => break,
            _ => return Err("Expected column name or 'FROM'".to_string()),
        }
    }

    let table_name = match iter.next() {
        Some(Token::Identifier(name)) => name.clone(),
        _ => return Err("Expected table name".to_string()),
    };

    Ok(Statement::Select(Select {
        columns,
        table_name,
    }))
}

pub fn parse_insert<'a>(
    iter: &mut Peekable<impl Iterator<Item = &'a Token>>,
) -> Result<Statement, String> {
    iter.next(); // Consume "INSERT"

    match iter.next() {
        Some(Token::Keyword(keyword)) if keyword.to_uppercase() == "INTO" => {}
        _ => return Err("Expected 'INTO' keyword".to_string()),
    }

    let table_name = match iter.next() {
        Some(Token::Identifier(name)) => name.clone(),
        _ => return Err("Expected table name".to_string()),
    };

    match iter.next() {
        Some(Token::Punctuation('(')) => {}
        _ => return Err("Expected '('".to_string()),
    }

    let mut values = Vec::new();
    loop {
        match iter.next() {
            Some(Token::Literal(value)) => values.push(value.clone()),
            Some(Token::Numeric(value)) => values.push(value.clone()),
            Some(token) => return Err(format!("Expected value, got {:?}", token)),
            _ => return Err("Unexpected end for value".to_string()),
        }

        match iter.peek() {
            Some(Token::Punctuation(',')) => {
                iter.next(); // Consume ","
                continue;
            }
            Some(Token::Punctuation(')')) => {
                iter.next(); // Consume ")"
                break;
            }
            _ => return Err("Expected ',' or ')'".to_string()),
        }
    }

    Ok(Statement::Insert(Insert { table_name, values }))
}
