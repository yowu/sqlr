use statement::{parse_create, parse_insert, parse_select};
use token::Token;
use tokenizer::tokenize;

use crate::db::{command::Command, statement::Statement};

mod statement;
mod token;
mod tokenizer;

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
        Some(Token::Keyword(keyword)) if keyword.to_uppercase() == "INSERT" => {
            parse_insert(&mut iter)
        }
        _ => Err("Unknown statement".to_string()),
    }
}

pub fn parse_command(input: &str) -> Command {
    match input {
        ".exit" => Command::Exit,
        _ => Command::Unknown(input.to_string()),
    }
}
