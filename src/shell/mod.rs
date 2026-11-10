pub mod builtins;

use std::io::{self, Write};

pub fn run_college_shell() {
    let stdin = io::stdin();
    let mut line = String::new();

    loop {
        // Цветной брендированный промпт для колледжа
        print!("\x1b[1;32mстудент@akstOS\x1b[0m:\x1b[1;34m~\x1b[0m$ ");
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
            println!("\x1b[31makstOS: команда '{}' не найдена. Введите 'help' для справки.\x1b[0m", cmd);
        }
    }
}
