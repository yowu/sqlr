use std::iter::Peekable;

use crate::ast::{ColumnDefinition, CreateTable, DataType, Select, Statement};
use crate::command::Command;
use crate::token::Token;
use crate::tokenizer::tokenize;

pub fn parse_statement(statement: &str) -> Result<Statement, String> {
    let tokens = tokenize(statement);
    let mut iter = tokens.iter().peekable();

    match iter.peek() {
        Some(Token::Keyword(keyword)) if keyword.to_uppercase() == "CREATE" => {
            parse_create(&mut iter)
        }
        Some(Token::Keyword(keyword)) if keyword.to_uppercase() == "SELECT" => {
            parse_select(&mut iter)
        }
        _ => Err("Unknown statement".to_string()),
    }
}

fn parse_create<'a>(
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

fn parse_create_table<'a>(
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

        columns.push(ColumnDefinition {
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
            "INT" => Ok(DataType::Int),
            "DATE" => Ok(DataType::Date),
            "FLOAT" => Ok(DataType::Float),
            "CHAR" => Ok(DataType::Char),
            "BOOLEAN" => Ok(DataType::Boolean),
            "VARCHAR" => {
                match iter.next() {
                    Some(Token::Punctuation('(')) => {}
                    _ => return Err("Expected '(' after VARCHAR".to_string()),
                }

                // Expect number
                let size = match iter.next() {
                    Some(Token::Number(size)) => size
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

fn parse_select<'a>(
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

pub fn parse_command(input: &str) -> Command {
    match input {
        ".exit" => Command::Exit,
        _ => Command::Unknown(input.to_string()),
    }
}
