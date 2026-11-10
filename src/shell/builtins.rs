use std::fs;
use std::path::Path;
use crate::college::{lab, monitor};
use crate::sys::power;

pub fn execute_builtin(cmd: &str, args: &[&str]) -> bool {
    match cmd {
        "справка" | "помощь" | "help" | "?" => {
            println!("\x1b[1;36m=== Список команд akstOS (на русском языке) ===\x1b[0m");
            println!("  \x1b[1;33mОбучение и колледж:\x1b[0m");
            println!("    лаб список          - показать все лабораторные работы");
            println!("    лаб инфо <номер>    - открыть задание работы");
            println!("    лаб проверить <номер>- сдать работу на автопроверку");
            println!("    создать_отчет <номер>- создать файл сдачи отчета");
            println!("    память (система)    - состояние ОЗУ и процессора старого ПК");
            println!("    тест                - замер скорости старого процессора");
            println!("    калькулятор <a> <+|-|*|/> <b> - вычисление выражения");
            println!();
            println!("  \x1b[1;33mФайлы и папки:\x1b[0m");
            println!("    файлы [каталог]     - список файлов (или 'ls')");
            println!("    читать <файл>       - открыть и прочесть файл (или 'cat')");
            println!("    папка <имя>         - создать новую папку (или 'mkdir')");
            println!("    записать <файл> <текст> - сохранить текст в файл");
            println!("    удалить <файл>      - удалить файл (или 'rm')");
            println!("    процессы            - список запущенных процессов (или 'ps')");
            println!("    очистить            - очистить экран (или 'clear')");
            println!("    перезагрузка        - перезагрузить компьютер (или 'reboot')");
            println!("    выключить           - выключить компьютер (или 'poweroff')");
            true
        }
        "лаб" | "lab" => {
            if args.is_empty() || args[0] == "список" || args[0] == "list" {
                lab::list_labs();
            } else if args[0] == "инфо" || args[0] == "info" {
                let id = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(1);
                lab::show_lab_info(id);
            } else if args[0] == "проверить" || args[0] == "check" {
                let id = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(1);
                lab::check_lab(id);
            } else {
                println!("Использование: лаб [список | инфо <№> | проверить <№>]");
            }
            true
        }
        "создать_отчет" => {
            let id = args.get(0).unwrap_or(&"1");
            let path = format!("/home/student/лабораторная{}.txt", id);
            let content = format!("Отчет по лабораторной работе №{} студента колледжа в akstOS.\nВыполнено успешно!\n", id);
            match fs::write(&path, content) {
                Ok(_) => println!("\x1b[1;32m[✓]\x1b[0m Файл отчета создан: {path}"),
                Err(e) => eprintln!("Ошибка создания отчета: {e}"),
            }
            true
        }
        "память" | "система" | "sysinfo" | "neofetch" => {
            monitor::show_compact_dashboard();
            true
        }
        "тест" | "bench" => {
            println!("\x1b[1;33m[Тест] Измерение производительности процессора...\x1b[0m");
            let start = std::time::Instant::now();
            let mut sum: u32 = 0;
            // Уменьшенное количество итераций, чтобы древний Celeron не зависал
            for i in 0..2_000_000u32 {
                sum = sum.wrapping_add(i.wrapping_mul(7));
            }
            let duration = start.elapsed();
            println!("\x1b[1;32m[✓]\x1b[0m 2 000 000 тактовых операций выполнены за: {:?}", duration);
            println!("Контрольная сумма: {}", sum);
            true
        }
        "калькулятор" | "calc" => {
            if args.len() == 3 {
                let a: Result<f64, _> = args[0].parse();
                let op = args[1];
                let b: Result<f64, _> = args[2].parse();

                match (a, b) {
                    (Ok(num1), Ok(num2)) => match op {
                        "+" => println!("Результат = {}", num1 + num2),
                        "-" => println!("Результат = {}", num1 - num2),
                        "*" => println!("Результат = {}", num1 * num2),
                        "/" => {
                            if num2 == 0.0 {
                                println!("Ошибка: деление на ноль невозможно!");
                            } else {
                                println!("Результат = {}", num1 / num2);
                            }
                        }
                        _ => println!("Неизвестное действие: {}. Допустимы: +, -, *, /", op),
                    },
                    _ => println!("Ошибка: вводите только числа"),
                }
            } else {
                println!("Использование: калькулятор 5 + 3");
            }
            true
        }
        "файлы" | "список" | "ls" => {
            let target = args.get(0).unwrap_or(&"/");
            match fs::read_dir(target) {
                Ok(entries) => {
                    for entry in entries.flatten() {
                        let name = entry.file_name();
                        let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);
                        if is_dir {
                            println!("\x1b[1;34m[папка] {}/\x1b[0m", name.to_string_lossy());
                        } else {
                            println!("        {}", name.to_string_lossy());
                        }
                    }
                }
                Err(e) => eprintln!("Ошибка чтения каталога '{}': {}", target, e),
            }
            true
        }
        "читать" | "cat" => {
            if let Some(file) = args.get(0) {
                match fs::read_to_string(file) {
                    Ok(data) => println!("{data}"),
                    Err(e) => eprintln!("Не удалось открыть {file}: {e}"),
                }
            } else {
                println!("Использование: читать <путь_к_файлу>");
            }
            true
        }
        "записать" => {
            if args.len() >= 2 {
                let file = args[0];
                let text = args[1..].join(" ");
                match fs::write(file, format!("{}\n", text)) {
                    Ok(_) => println!("\x1b[1;32m[✓]\x1b[0m Записано в {}", file),
                    Err(e) => eprintln!("Ошибка записи в {file}: {e}"),
                }
            } else {
                println!("Использование: записать <файл> <текст>");
            }
            true
        }
        "папка" | "mkdir" => {
            if let Some(dir) = args.get(0) {
                if let Err(e) = fs::create_dir_all(dir) {
                    eprintln!("Не удалось создать папку {dir}: {e}");
                } else {
                    println!("\x1b[1;32m[✓]\x1b[0m Папка создана: {}", dir);
                }
            } else {
                println!("Использование: папка <имя_папки>");
            }
            true
        }
        "удалить" | "rm" => {
            if let Some(target) = args.get(0) {
                let p = Path::new(target);
                let res = if p.is_dir() {
                    fs::remove_dir_all(p)
                } else {
                    fs::remove_file(p)
                };
                if let Err(e) = res {
                    eprintln!("Ошибка удаления {target}: {e}");
                } else {
                    println!("\x1b[1;32m[✓]\x1b[0m Удалено: {}", target);
                }
            } else {
                println!("Использование: удалить <файл>");
            }
            true
        }
        "процессы" | "ps" => {
            println!("\x1b[1m НОМЕР(PID)  ПРОЦЕСС\x1b[0m");
            if let Ok(entries) = fs::read_dir("/proc") {
                for entry in entries.flatten() {
                    if let Ok(name) = entry.file_name().into_string() {
                        if name.chars().all(|c| c.is_ascii_digit()) {
                            let cmdline_path = format!("/proc/{}/cmdline", name);
                            let cmd = fs::read_to_string(cmdline_path)
                                .unwrap_or_else(|_| "система".to_string())
                                .replace('\0', " ");
                            let display = if cmd.trim().is_empty() { "[ядро linux]" } else { cmd.trim() };
                            println!("{:>8}   {}", name, display);
                        }
                    }
                }
            }
            true
        }
        "очистить" | "clear" => {
            print!("\x1b[2J\x1b[1;1H");
            true
        }
        "перезагрузка" | "reboot" => {
            power::reboot();
        }
        "выключить" | "poweroff" | "exit" | "выход" => {
            power::poweroff();
        }
        _ => false,
    }
}
