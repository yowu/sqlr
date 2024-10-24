use handlers::{process_commands, process_statement};
use input::{print_prompt, read_user_input};

use crate::db::database::Database;
mod handlers;
mod input;

pub fn run_ui_loop(db: &mut Database) {
    loop {
        print_prompt();
        let user_input = read_user_input();

        match user_input.starts_with('.') {
            true => process_commands(&user_input),
            false => process_statement(db, &user_input),
        }
    }
}
