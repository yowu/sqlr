use crate::db::{command::Command, database::Database, statement::Statement};
use crate::parser::{parse_command, parse_statement};

pub fn process_commands(command: &str) {
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

pub fn process_statement(db: &mut Database, statement: &str) {
    if statement.is_empty() {
        return;
    }
    match parse_statement(statement) {
        Ok(statement) => match statement {
            Statement::CreateTable(create_table) => {
                println!("Create table: {}", create_table);
                db.create_table(&create_table);
            }
            Statement::Select(select) => {
                println!("Select: {}", select);
                let rows = db.select_from_table(&select).unwrap();
                // print table header
                let columns = db.get_table_columns(&select.table_name).unwrap();
                for column in &columns {
                    print!("|{:<20}", column.name);
                }
                println!("|");
                println!("{}", "-".repeat(21 * columns.len()) + "-");

                // print table rows
                for row in rows {
                    for val in &row.values {
                        print!("|{:<20}", val.to_string());
                    }
                    println!("|");
                }
            }
            Statement::Insert(insert) => {
                println!("Insert: {}", insert);
                if let Err(e) = db.insert_into_table(&insert) {
                    println!("Error: {}", e);
                }
            }
        },
        Err(e) => println!("Error: {}", e),
    }
}
