mod college;
mod shell;
mod sys;

fn setup_console() {
    unsafe {
        // Создаем каталог /dev и пробуем примонтировать devtmpfs
        let _ = std::fs::create_dir_all("/dev");
        let dev = std::ffi::CString::new("devtmpfs").unwrap();
        let target = std::ffi::CString::new("/dev").unwrap();
        let _ = libc::mount(
            dev.as_ptr(),
            target.as_ptr(),
            dev.as_ptr(),
            0,
            std::ptr::null(),
        );

        // Открываем системную консоль /dev/console для ввода и вывода на монитор
        let console_path = std::ffi::CString::new("/dev/console").unwrap();
        let fd = libc::open(console_path.as_ptr(), libc::O_RDWR);
        if fd >= 0 {
            libc::dup2(fd, 0); // stdin  -> клавиатура
            libc::dup2(fd, 1); // stdout -> экран монитора
            libc::dup2(fd, 2); // stderr -> экран монитора
            if fd > 2 {
                libc::close(fd);
            }
        }
    }
}

fn print_welcome_banner() {
    println!("\x1b[1;36m");
    println!(r#"     _    _  ______ _____ ___  ____  "#);
    println!(r#"    / \  | |/ / ___|_   _/ _ \/ ___| "#);
    println!(r#"   / _ \ | ' /\___ \ | || | | \___ \ "#);
    println!(r#"  / ___ \| . \ ___) || || |_| |___) |"#);
    println!(r#" /_/   \_\_|\_\____/ |_| \___/|____/ "#);
    println!("\x1b[0m");
    println!("\x1b[1;32m==============================================================\x1b[0m");
    println!("\x1b[1;33m akstOS: Облегченная операционная система для колледжа (v1.0)\x1b[0m");
    println!(" Специальная версия: для слабых и старых компьютеров.");
    println!(" Полностью на русском языке. Ядро: Linux | Окружение: Rust");
    println!("\x1b[1;32m==============================================================\x1b[0m");
    println!("\x1b[1;37;44m [ПОДСКАЗКА] Чтобы установить akstOS на жесткий диск, введите: \x1b[1;33mустановить\x1b[1;37;44m \x1b[0m\n");
}

fn main() {
    // 1. Привязываем ввод и вывод к экрану /dev/console
    setup_console();

    let pid = unsafe { libc::getpid() };

    if pid == 1 {
        println!("\x1b[1;32m[Загрузка] Запуск ядра akstOS (Главный процесс PID 1)...\x1b[0m");
        sys::mount::mount_essential_filesystems();
    } else {
        println!("[akstOS] Запуск в режиме тестирования (PID: {})", pid);
    }

    print_welcome_banner();

    // Диагностика железа и памяти старого ПК
    college::monitor::show_compact_dashboard();
    println!();
    college::lab::list_labs();
    println!();

    // Запуск русскоязычной студенческой оболочки
    shell::run_college_shell();

    // Безопасное завершение работы
    sys::power::poweroff();
}
