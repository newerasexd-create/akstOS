mod college;
mod shell;
mod sys;

fn print_welcome_banner() {
    println!("\x1b[1;35m");
    println!(r#"        _        _    ___  ____  "#);
    println!(r#"   __ _| | _____| |_ / _ \/ ___| "#);
    println!(r#"  / _` | |/ / __| __| | | \___ \ "#);
    println!(r#" | (_| |   <\__ \ |_| |_| |___) |"#);
    println!(r#"  \__,_|_|\_\___/\__|\___/|____/ "#);
    println!("\x1b[0m");
    println!("\x1b[1;36m=== akstOS: Операционная система для колледжа (v1.0) ===\x1b[0m");
    println!("Разработано для лабораторных работ и системного программирования на Rust.");
    println!("Пользовательское пространство: 100% Rust | Ядро: Linux");
    println!("------------------------------------------------------------\n");
}

fn main() {
    let pid = unsafe { libc::getpid() };

    if pid == 1 {
        println!("\x1b[1;32m[akst-init] Запущен как основной системный процесс PID 1\x1b[0m");
        // Монтирование ключевых файловых систем
        sys::mount::mount_essential_filesystems();
    } else {
        println!("[akstOS] Запуск в пользовательском режиме (PID: {})", pid);
    }

    print_welcome_banner();
    college::monitor::show_system_dashboard();
    println!();
    college::lab::list_labs();
    println!();

    // Запуск студенческой оболочки
    shell::run_college_shell();

    // Если оболочка завершилась, корректно гасим систему
    sys::power::poweroff();
}
