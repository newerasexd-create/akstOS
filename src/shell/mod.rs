pub mod builtins;

use std::io::{self, Write};

pub fn run_college_shell() {
    let stdin = io::stdin();
    let mut line = String::new();

    loop {
        // Русскоязычный понятный терминал студента
        print!("\x1b[1;32m[akstOS]\x1b[0m:\x1b[1;34m[студент]\x1b[0m# ");
        io::stdout().flush().unwrap();

        line.clear();
        if stdin.read_line(&mut line).is_err() || line.is_empty() {
            println!();
            continue;
        }

        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        let cmd = parts[0];
        let args = &parts[1..];

        if !builtins::execute_builtin(cmd, args) {
            println!("\x1b[31makstOS: неизвестная команда '{}'. Введите 'справка' для списка команд.\x1b[0m", cmd);
        }
    }
}
