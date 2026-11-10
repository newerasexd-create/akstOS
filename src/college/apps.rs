use std::process::Command;
use std::path::Path;

pub struct CollegeApp {
    pub name: &'static str,
    pub command: &'static str,
    pub alias: &'static str,
    pub description: &'static str,
    pub binary_path: &'static str,
    pub url: &'static str,
}

pub const PREINSTALLED_APPS: &[CollegeApp] = &[
    CollegeApp {
        name: "Lapce (Код)",
        command: "lapce",
        alias: "код",
        description: "Легковесная среда разработки (быстрый аналог VS Code на чистом Rust)",
        binary_path: "/usr/bin/lapce",
        url: "https://github.com/lapce/lapce",
    },
    CollegeApp {
        name: "WordCraft",
        command: "wordcraft",
        alias: "ворд",
        description: "Офисный текстовый процессор для документов (чистый Rust)",
        binary_path: "/usr/bin/wordcraft",
        url: "https://github.com/storytold/wordcraft",
    },
    CollegeApp {
        name: "PdfCraft",
        command: "pdfcraft",
        alias: "пдф",
        description: "Просмотр, чтение и аннотирование PDF-файлов (чистый Rust)",
        binary_path: "/usr/bin/pdfcraft",
        url: "https://github.com/storytold/pdfcraft",
    },
    CollegeApp {
        name: "GridCraft",
        command: "gridcraft",
        alias: "таблицы",
        description: "Электронные таблицы и расчеты данных (чистый Rust)",
        binary_path: "/usr/bin/gridcraft",
        url: "https://github.com/storytold/gridcraft",
    },
    CollegeApp {
        name: "DeckCraft",
        command: "deckcraft",
        alias: "слайды",
        description: "Создание и показ презентаций (чистый Rust)",
        binary_path: "/usr/bin/deckcraft",
        url: "https://github.com/storytold/deckcraft",
    },
    CollegeApp {
        name: "ArtCraft",
        command: "artcraft",
        alias: "рисование",
        description: "Графический редактор и цифровое рисование (чистый Rust)",
        binary_path: "/usr/bin/artcraft",
        url: "https://github.com/storytold/artcraft",
    },
    CollegeApp {
        name: "Яндекс Браузер",
        command: "yandex-browser",
        alias: "браузер",
        description: "Современный веб-браузер для доступа в интернет и портала колледжа",
        binary_path: "/usr/bin/yandex-browser",
        url: "https://browser.yandex.ru",
    },
    CollegeApp {
        name: "Python 3",
        command: "python3",
        alias: "питон",
        description: "Язык программирования Python свежей версии для учебы и лабораторных",
        binary_path: "/usr/bin/python3",
        url: "https://www.python.org",
    },
];

/// Вывод списка всех предустановленных программ колледжа
pub fn list_apps() {
    println!("\x1b[1;36m=== Предустановленные программы akstOS College ===\x1b[0m");
    println!("Для запуска программы введите её русское название или команду.\n");

    for (i, app) in PREINSTALLED_APPS.iter().enumerate() {
        let installed = Path::new(app.binary_path).exists();
        let status = if installed {
            "\x1b[1;32m[УСТАНОВЛЕНО]\x1b[0m"
        } else {
            "\x1b[1;33m[В СОСТАВЕ ДИСТРИБУТИВА]\x1b[0m"
        };

        println!(" \x1b[1;32m{}.\x1b[0m \x1b[1m{}\x1b[0m {}", i + 1, app.name, status);
        println!("    Команда запуска: \x1b[1;33m{}\x1b[0m (или \x1b[36m{}\x1b[0m)", app.alias, app.command);
        println!("    Назначение: {}", app.description);
        println!("    Источник: {}\n", app.url);
    }
}

/// Запуск предустановленного приложения
pub fn launch_app(cmd: &str, args: &[&str]) -> bool {
    if let Some(app) = PREINSTALLED_APPS.iter().find(|a| a.command == cmd || a.alias == cmd || (cmd == "vscode" && a.command == "lapce")) {
        if Path::new(app.binary_path).exists() {
            println!("\x1b[1;32m[Запуск]\x1b[0m Запуск {}...", app.name);
            let mut process = Command::new(app.binary_path);
            process.args(args);
            match process.spawn() {
                Ok(mut child) => {
                    let _ = child.wait();
                }
                Err(e) => {
                    eprintln!("Ошибка запуска {}: {}", app.name, e);
                }
            }
        } else {
            println!("\x1b[1;33m[akstOS]\x1b[0m Программа \x1b[1m{}\x1b[0m готова к установке.", app.name);
            println!("  Она входит в стандартный предустановленный пакет на жесткий диск.");
            println!("  Чтобы установить всю систему с этими программами, введите команду: \x1b[1;32mустановить\x1b[0m\n");

            if app.command == "python3" {
                if let Ok(mut child) = Command::new("python").args(args).spawn() {
                    let _ = child.wait();
                    return true;
                }
            }
        }
        return true;
    }
    false
}
