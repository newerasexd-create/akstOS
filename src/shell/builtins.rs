use std::fs;
use std::path::Path;
use crate::college::{lab, monitor};
use crate::sys::power;

pub fn execute_builtin(cmd: &str, args: &[&str]) -> bool {
    match cmd {
        "help" => {
            println!("\x1b[1;36m=== Справочник команд akstOS College ===\x1b[0m");
            println!("  \x1b[1;32mОбучение и лаборатория:\x1b[0m");
            println!("    lab list            - список лабораторных работ колледжа");
            println!("    lab info <номер>    - задание к лабораторной работе");
            println!("    lab check <номер>   - автоматическая проверка выполнения");
            println!("    sysinfo (neofetch)  - состояние системы, память, ядро");
            println!("    bench               - замер быстродействия процессора (бенчмарк)");
            println!("    calc <a +|-|*|/ b>  - учебный калькулятор");
            println!();
            println!("  \x1b[1;32mФайлы и система:\x1b[0m");
            println!("    ls [путь]           - список файлов и каталогов");
            println!("    cat <файл>          - просмотр текстового файла");
            println!("    mkdir <папка>       - создание каталога");
            println!("    rm <файл>           - удаление файла");
            println!("    echo <текст>        - печать текста");
            println!("    ps                  - просмотр активных процессов");
            println!("    clear               - очистка терминала");
            println!("    uname               - версия ядра Linux");
            println!("    reboot              - перезагрузка операционной системы");
            println!("    poweroff            - выключение компьютера");
            true
        }
        "lab" => {
            if args.is_empty() || args[0] == "list" {
                lab::list_labs();
            } else if args[0] == "info" {
                let id = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(1);
                lab::show_lab_info(id);
            } else if args[0] == "check" {
                let id = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(1);
                lab::check_lab(id);
            } else {
                println!("Использование: lab [list | info <N> | check <N>]");
            }
            true
        }
        "sysinfo" | "neofetch" | "fetch" => {
            monitor::show_system_dashboard();
            true
        }
        "bench" => {
            println!("[akstOS Bench] Запуск академического бенчмарка вычислений...");
            let start = std::time::Instant::now();
            let mut sum: u64 = 0;
            for i in 0..10_000_000u64 {
                sum = sum.wrapping_add(i.wrapping_mul(3));
            }
            let elapsed = start.elapsed();
            println!("[✓] 10,000,000 математических операций завершены за: {:?}", elapsed);
            println!("Результат контрольной суммы: {}", sum);
            true
        }
        "calc" => {
            if args.len() == 3 {
                let a: Result<f64, _> = args[0].parse();
                let op = args[1];
                let b: Result<f64, _> = args[2].parse();

                match (a, b) {
                    (Ok(num1), Ok(num2)) => match op {
                        "+" => println!("= {}", num1 + num2),
                        "-" => println!("= {}", num1 - num2),
                        "*" => println!("= {}", num1 * num2),
                        "/" => {
                            if num2 == 0.0 {
                                println!("Ошибка: деление на ноль!");
                            } else {
                                println!("= {}", num1 / num2);
                            }
                        }
                        _ => println!("Неизвестный оператор: {}. Поддерживаются: +, -, *, /", op),
                    },
                    _ => println!("Ошибка: аргументы должны быть числами"),
                }
            } else {
                println!("Использование: calc <число1> <+|-|*|/> <число2>");
            }
            true
        }
        "uname" => {
            if let Ok(content) = fs::read_to_string("/proc/version") {
                println!("{}", content.trim());
            } else {
                println!("Linux akstOS 1.0 (Rust Userland)");
            }
            true
        }
        "ls" => {
            let target = args.get(0).unwrap_or(&"/");
            match fs::read_dir(target) {
                Ok(entries) => {
                    for entry in entries.flatten() {
                        let name = entry.file_name();
                        let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);
                        if is_dir {
                            println!("\x1b[1;34m{}/\x1b[0m", name.to_string_lossy());
                        } else {
                            println!("{}", name.to_string_lossy());
                        }
                    }
                }
                Err(e) => eprintln!("ls: не удалось прочитать '{}': {}", target, e),
            }
            true
        }
        "cat" => {
            if let Some(file) = args.get(0) {
                match fs::read_to_string(file) {
                    Ok(data) => print!("{data}"),
                    Err(e) => eprintln!("cat: {file}: {e}"),
                }
            } else {
                println!("Использование: cat <путь_к_файлу>");
            }
            true
        }
        "mkdir" => {
            if let Some(dir) = args.get(0) {
                if let Err(e) = fs::create_dir_all(dir) {
                    eprintln!("mkdir: {dir}: {e}");
                }
            } else {
                println!("Использование: mkdir <каталог>");
            }
            true
        }
        "rm" => {
            if let Some(target) = args.get(0) {
                let p = Path::new(target);
                let res = if p.is_dir() {
                    fs::remove_dir_all(p)
                } else {
                    fs::remove_file(p)
                };
                if let Err(e) = res {
                    eprintln!("rm: {target}: {e}");
                }
            } else {
                println!("Использование: rm <файл>");
            }
            true
        }
        "echo" => {
            println!("{}", args.join(" "));
            true
        }
        "clear" => {
            print!("\x1b[2J\x1b[1;1H");
            true
        }
        "ps" => {
            println!("\x1b[1m PID   CMD\x1b[0m");
            if let Ok(entries) = fs::read_dir("/proc") {
                for entry in entries.flatten() {
                    if let Ok(name) = entry.file_name().into_string() {
                        if name.chars().all(|c| c.is_ascii_digit()) {
                            let cmdline_path = format!("/proc/{}/cmdline", name);
                            let cmd = fs::read_to_string(cmdline_path)
                                .unwrap_or_else(|_| "unknown".to_string())
                                .replace('\0', " ");
                            let display_cmd = if cmd.trim().is_empty() { "[kernel]" } else { cmd.trim() };
                            println!("{:>4}   {}", name, display_cmd);
                        }
                    }
                }
            }
            true
        }
        "reboot" => {
            power::reboot();
        }
        "poweroff" => {
            power::poweroff();
        }
        _ => false,
    }
}
