mod ast;
mod command;
mod input;
mod parser;
mod token;
mod tokenizer;

use command::Command;
use input::{print_prompt, read_user_input};
use parser::{parse_command, parse_statement};

fn process_commands(command: &str) {
    match parse_command(command) {
        Command::Exit => {
            println!("Exiting the application.");
            std::process::exit(0);
        }
        Command::Unknown(cmd) => {
            println!("Unknown command: {}", cmd);
        }
    }
}

fn process_statement(statement: &str) {
    if statement.is_empty() {
        return;
    }
    match parse_statement(statement) {
        Ok(ast) => println!("{:#?}", ast),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    println!("Welcome to the sqlr!");
    loop {
        print_prompt();
        let user_input = read_user_input();

        match user_input.starts_with('.') {
            true => process_commands(&user_input),
            false => process_statement(&user_input),
        }
    }
}
