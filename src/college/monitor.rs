use std::fs;

/// Получение точного времени и даты в часовом поясе UTC+9 (без тяжелых внешних библиотек)
pub fn get_utc9_time() -> (u64, u64, u64, u64, u64, u64) {
    let total_secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() + (9 * 3600); // Смещение UTC+9 (+9 часов)

    let sec = total_secs % 60;
    let min = (total_secs / 60) % 60;
    let hour = (total_secs / 3600) % 24;

    // Алгоритм расчета григорианского календаря (Howard Hinnant)
    let mut days = (total_secs / 86400) as i64;
    days += 719468;
    let era = (if days >= 0 { days } else { days - 146096 }) / 146097;
    let doe = (days - era * 146097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = (yoe as i64) + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = if m <= 2 { y + 1 } else { y };

    (year as u64, m as u64, d as u64, hour, min, sec)
}

/// Вывод времени строкой
pub fn format_utc9_time() -> String {
    let (y, m, d, h, min, s) = get_utc9_time();
    format!("{:02}.{:02}.{:04} {:02}:{:02}:{:02} (UTC+9)", d, m, y, h, min, s)
}

/// Компактный монитор системы для древних мониторов и малого ОЗУ
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
    println!(r#"  / ____ \     Время: {}"#, format_utc9_time());
    println!(r#" /_/    \_\    Процессор: {} (ядер: {})"#, cpu_model, cpu_cores);
    println!(r#"               ОЗУ: {} МБ из {} МБ (занято: {}%)"#, mem_used / 1024, mem_total / 1024, mem_percent);
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
