use std::fs;
use std::path::Path;

pub struct LabTask {
    pub id: usize,
    pub title: &'static str,
    pub description: &'static str,
    pub instruction: &'static str,
}

pub const LABS: &[LabTask] = &[
    LabTask {
        id: 1,
        title: "Изучение виртуальной файловой системы /proc",
        description: "Освоение псевдо-файлов ядра Linux (/proc/cpuinfo, /proc/meminfo)",
        instruction: "1. Выполните 'cat /proc/cpuinfo' и найдите модель процессора.\n2. Выполните 'cat /proc/meminfo' и найдите объем памяти MemTotal.\n3. Создайте файл ответа: выполните 'echo lab1_done > /home/student/labs/lab1.txt'",
    },
    LabTask {
        id: 2,
        title: "Управление файлами и структурой каталогов",
        description: "Создание каталога проекта и структуры файлов студента",
        instruction: "1. Создайте папку: 'mkdir /home/student/projects/my_project'\n2. Создайте файл с кодом студента внутри проекта: 'echo \"fn main() {}\" > /home/student/projects/my_project/main.rs'",
    },
    LabTask {
        id: 3,
        title: "Анализ процессов и архитектуры ОС",
        description: "Исследование процесса инициализации PID 1 и системных метрик",
        instruction: "1. Вызовите команду 'sysinfo' для анализа использования памяти.\n2. Вызовите 'ps' и изучите, почему PID 1 принадлежит akst-init.",
    },
];

pub fn list_labs() {
    println!("\x1b[1;36m=== Список лабораторных работ akstOS College ===\x1b[0m");
    for lab in LABS {
        println!("\x1b[1;33m[Лаб #{}]\x1b[0m {}", lab.id, lab.title);
        println!("  Описание: {}", lab.description);
    }
    println!("\nДля деталей введите: \x1b[32mlab info <номер>\x1b[0m");
    println!("Для проверки решения: \x1b[32mlab check <номер>\x1b[0m");
}

pub fn show_lab_info(id: usize) {
    if let Some(lab) = LABS.iter().find(|l| l.id == id) {
        println!("\x1b[1;34m=== Лабораторная работа #{} ===\x1b[0m", lab.id);
        println!("\x1b[1mТема:\x1b[0m {}", lab.title);
        println!("\x1b[1mЦель:\x1b[0m {}", lab.description);
        println!("\x1b[1mЗадание:\x1b[0m\n{}", lab.instruction);
    } else {
        println!("Лабораторная работа #{} не найдена. Доступны: 1..{}", id, LABS.len());
    }
}

pub fn check_lab(id: usize) {
    match id {
        1 => {
            let path = Path::new("/home/student/labs/lab1.txt");
            if path.exists() {
                println!("\x1b[1;32m[✓ СДАНО]\x1b[0m Лабораторная #1 успешно зачтена! Файл отчета найден.");
            } else {
                println!("\x1b[1;31m[✗ НЕ СДАНО]\x1b[0m Файл /home/student/labs/lab1.txt не найден.");
                println!("Подсказка: выполните 'echo lab1_done > /home/student/labs/lab1.txt'");
            }
        }
        2 => {
            let path = Path::new("/home/student/projects/my_project/main.rs");
            if path.exists() {
                println!("\x1b[1;32m[✓ СДАНО]\x1b[0m Лабораторная #2 успешно зачтена! Структура проекта корректна.");
            } else {
                println!("\x1b[1;31m[✗ НЕ СДАНО]\x1b[0m Файл /home/student/projects/my_project/main.rs не найден.");
            }
        }
        3 => {
            println!("\x1b[1;32m[✓ СДАНО]\x1b[0m Теоретический модуль #3 пройден. Ответьте на контрольные вопросы преподавателю.");
        }
        _ => {
            println!("Лабораторная #{} не поддерживается автоматической проверкой.", id);
        }
    }
}
