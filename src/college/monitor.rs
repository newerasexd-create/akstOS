use std::fs;

/// Компактный монитор системы для древних мониторов (80x25 символов) и малого ОЗУ
pub fn show_compact_dashboard() {
    let hostname = fs::read_to_string("/etc/hostname")
        .unwrap_or_else(|_| "akstos-college".to_string());

    let (cpu_model, cpu_cores) = read_cpu_info();
    let (mem_total, mem_free) = read_memory_info();
    let mem_used = mem_total.saturating_sub(mem_free);
    let mem_percent = if mem_total > 0 { (mem_used * 100) / mem_total } else { 0 };

    println!("\x1b[1;36m");
    println!(r#"     /\        akstOS Легковесная Редакция"#);
    println!(r#"    /  \       --------------------------------"#);
    println!(r#"   / /\ \      Хост: {}"#, hostname.trim());
    println!(r#"  / ____ \     Процессор: {} (ядер: {})"#, cpu_model, cpu_cores);
    println!(r#" /_/    \_\    ОЗУ: {} МБ из {} МБ (занято: {}%)"#, mem_used / 1024, mem_total / 1024, mem_percent);
    println!("\x1b[0m");

    // Индикатор памяти
    let width = 24;
    let filled = ((mem_percent as usize) * width) / 100;
    let empty = width.saturating_sub(filled);
    print!(" Память: [");
    print!("\x1b[1;32m{}\x1b[0m", "#".repeat(filled));
    print!("{}", "-".repeat(empty));
    println!("] {} МБ свободно", mem_free / 1024);

    if mem_total / 1024 < 512 {
        println!("\x1b[1;33m [!] Режим экономии ОЗУ: система оптимизирована под малый объем памяти.\x1b[0m");
    }
}

fn read_cpu_info() -> (String, usize) {
    let mut model = "x86 Процессор".to_string();
    let mut cores = 0;

    if let Ok(content) = fs::read_to_string("/proc/cpuinfo") {
        for line in content.lines() {
            if line.starts_with("model name") {
                if let Some(val) = line.split(':').nth(1) {
                    model = val.trim().to_string();
                }
            } else if line.starts_with("processor") {
                cores += 1;
            }
        }
    }
    if cores == 0 { cores = 1; }
    (model, cores)
}

fn read_memory_info() -> (u64, u64) {
    let mut total = 0;
    let mut free = 0;

    if let Ok(content) = fs::read_to_string("/proc/meminfo") {
        for line in content.lines() {
            if line.starts_with("MemTotal:") {
                total = parse_kb(line);
            } else if line.starts_with("MemAvailable:") || line.starts_with("MemFree:") {
                if free == 0 {
                    free = parse_kb(line);
                }
            }
        }
    }
    (total, free)
}

fn parse_kb(line: &str) -> u64 {
    line.split_whitespace()
        .nth(1)
        .and_then(|val| val.parse::<u64>().ok())
        .unwrap_or(0)
}
