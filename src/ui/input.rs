use std::io::{self, Write};

pub fn print_prompt() {
    print!("sqlr> ");
    io::stdout().flush().unwrap();
}

pub fn read_user_input() -> String {
    let mut statement = String::new();

    loop {
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        // Remove the newline character from the input
        let user_input = user_input.trim_end();

        match user_input.ends_with('\\') {
            true => {
                // Remove the backslash and continue reading the next line
                statement.push_str(&user_input[..user_input.len() - 1]);
                statement.push(' ');
            }
            false => {
                statement.push_str(user_input);
                statement.push(' ');
                break;
            }
        }
    }

    statement.trim().to_string()
}
