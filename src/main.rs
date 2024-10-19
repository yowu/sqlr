mod db;
mod parser;
mod ui;

use db::Database;
use ui::run_ui_loop;

fn main() {
    println!("Welcome to the sqlr!");
    let mut db = Database::new();
    run_ui_loop(&mut db);
}
