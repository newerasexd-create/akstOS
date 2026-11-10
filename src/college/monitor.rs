use std::fs;

pub fn show_system_dashboard() {
    let hostname = fs::read_to_string("/etc/hostname")
        .unwrap_or_else(|_| "akstos-college".to_string());
    
    let kernel_ver = fs::read_to_string("/proc/version")
        .unwrap_or_else(|_| "Linux Kernel (Generic)".to_string());
    let short_kernel = kernel_ver.split_whitespace().take(3).collect::<Vec<_>>().join(" ");

    let uptime_str = fs::read_to_string("/proc/uptime")
        .unwrap_or_else(|_| "0.0 0.0".to_string());
    let uptime_secs: f64 = uptime_str.split_whitespace().next().and_then(|s| s.parse().ok()).unwrap_or(0.0);
    let uptime_mins = (uptime_secs / 60.0) as u64;

    // Чтение памяти
    let (mem_total, mem_free) = read_memory_info();
    let mem_used = mem_total.saturating_sub(mem_free);
    let mem_percent = if mem_total > 0 { (mem_used * 100) / mem_total } else { 0 };

    println!("\x1b[1;36m");
    println!(r#"        /\          akstOS Academic Edition"#);
    println!(r#"       /  \         -----------------------"#);
    println!(r#"      / /\ \        ОС: akstOS 1.0 (Rust Userland)"#);
    println!(r#"     / ____ \       Ядро: {}"#, short_kernel);
    println!(r#"    /_/    \_\      Хост: {}"#, hostname.trim());
    println!(r#"   [akst College]   Аптайм: {} мин."#, uptime_mins);
    println!(r#"                    ОЗУ: {} MB / {} MB [{}%]"#, mem_used / 1024, mem_total / 1024, mem_percent);
    println!("\x1b[0m");

    // Графический индикатор памяти для студентов
    let bar_width = 30;
    let filled = ((mem_percent as usize) * bar_width) / 100;
    let empty = bar_width.saturating_sub(filled);
    print!("ОЗУ: [");
    print!("\x1b[1;32m{}\x1b[0m", "#".repeat(filled));
    print!("{}", "-".repeat(empty));
    println!("] {}%", mem_percent);
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
