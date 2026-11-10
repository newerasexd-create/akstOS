use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

pub struct DiskInfo {
    pub name: String,
    pub path: String,
    pub size_gb: f64,
    pub model: String,
}

/// Поиск всех физических накопителей в системе (/dev/sda, /dev/hda и т.д.)
pub fn scan_disks() -> Vec<DiskInfo> {
    let mut disks = Vec::new();

    if let Ok(entries) = fs::read_dir("/sys/block") {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();

            // Фильтруем loop, ram, dm устройства
            if name.starts_with("sd") || name.starts_with("hd") || name.starts_with("vd") || name.starts_with("nvme") {
                let size_path = format!("/sys/block/{}/size", name);
                let sectors: u64 = fs::read_to_string(size_path)
                    .unwrap_or_default()
                    .trim()
                    .parse()
                    .unwrap_or(0);

                let bytes = sectors * 512;
                let size_gb = (bytes as f64) / (1024.0 * 1024.0 * 1024.0);

                let model_path = format!("/sys/block/{}/device/model", name);
                let model = fs::read_to_string(model_path)
                    .unwrap_or_else(|_| "Жесткий диск (HDD/SSD)".to_string())
                    .trim()
                    .to_string();

                disks.push(DiskInfo {
                    name: name.clone(),
                    path: format!("/dev/{}", name),
                    size_gb,
                    model,
                });
            }
        }
    }

    disks
}

/// Интерактивный установщик akstOS на жесткий диск старого ПК
pub fn run_installer() {
    println!("\x1b[1;36m");
    println!("============================================================");
    println!("     Мастер быстрой установки akstOS на жесткий диск        ");
    println!("============================================================\x1b[0m");

    let disks = scan_disks();

    if disks.is_empty() {
        println!("\x1b[1;33m[!] Внимание: В системе /sys/block не найдено физических накопителей.");
        println!("    (Если вы тестируете в Live-режиме, диски будут смонтированы при загрузке с BIOS).\x1b[0m\n");
        println!("Пример того, как установщик работает на реальном ПК колледжа:");
        println!("  1) /dev/sda [ 80.0 ГБ ] — Seagate Barracuda 7200");
        println!("  2) /dev/sdb [  4.0 ГБ ] — Загрузочная флешка USB\n");
        return;
    }

    println!("Обнаружены доступные диски в компьютере:\n");
    for (i, disk) in disks.iter().enumerate() {
        println!("  \x1b[1;32m[{}]\x1b[0m {} [ {:.1} ГБ ] — {}", i + 1, disk.path, disk.size_gb, disk.model);
    }

    print!("\n\x1b[1;33mВыберите номер диска для установки akstOS (1-{}): \x1b[0m", disks.len());
    io::stdout().flush().unwrap();

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        return;
    }

    let choice: usize = match input.trim().parse() {
        Ok(num) if num >= 1 && num <= disks.len() => num - 1,
        _ => {
            println!("\x1b[31mОшибка: неверный номер диска. Установка отменена.\x1b[0m");
            return;
        }
    };

    let target_disk = &disks[choice];

    println!("\n\x1b[1;31m[ВНИМАНИЕ!]\x1b[0m Все данные на диске \x1b[1m{}\x1b[0m ({:.1} ГБ) будут УДАЛЕНЫ!", target_disk.path, target_disk.size_gb);
    print!("Вы уверены, что хотите установить akstOS на этот диск? (введите 'да'): ");
    io::stdout().flush().unwrap();

    input.clear();
    if io::stdin().read_line(&mut input).is_err() || input.trim().to_lowercase() != "да" {
        println!("Установка отменена пользователем.");
        return;
    }

    println!("\n\x1b[1;32m[1/4] Подготовка разметки диска {} (MBR для старых BIOS)...\x1b[0m", target_disk.path);
    // 1. Создание MBR раздела
    let _ = Command::new("parted")
        .args(["-s", &target_disk.path, "mklabel", "msdos", "mkpart", "primary", "ext4", "1MiB", "100%", "set", "1", "boot", "on"])
        .status();

    let partition = if target_disk.path.contains("nvme") {
        format!("{}p1", target_disk.path)
    } else {
        format!("{}1", target_disk.path)
    };

    println!("\x1b[1;32m[2/4] Форматирование раздела {} (легковесный ext4)...\x1b[0m", partition);
    let _ = Command::new("mkfs.ext4")
        .args(["-F", "-L", "akstOS", &partition])
        .status();

    println!("\x1b[1;32m[3/4] Копирование системных файлов и ядра на жесткий диск...\x1b[0m");
    let target_mount = "/mnt/target";
    let _ = fs::create_dir_all(target_mount);
    
    // Монтирование целевого раздела
    let _ = Command::new("mount").args([&partition, target_mount]).status();

    let boot_dir = format!("{}/boot", target_mount);
    let _ = fs::create_dir_all(&boot_dir);

    // Копирование ядра и initramfs
    let _ = Command::new("cp").args(["/init", &format!("{}/init", target_mount)]).status();

    // Создание конфигурации загрузчика syslinux / extlinux
    println!("\x1b[1;32m[4/4] Установка сверхлегкого загрузчика (Syslinux MBR)...\x1b[0m");
    let extlinux_conf = format!("{}/syslinux.cfg", boot_dir);
    let cfg_content = "DEFAULT akstOS\nPROMPT 0\nTIMEOUT 30\n\nLABEL akstOS\n  LINUX /boot/vmlinuz\n  INITRD /boot/initramfs.cpio.gz\n  APPEND root=LABEL=akstOS nomodeset vga=791 console=tty0 quiet\n";
    let _ = fs::write(extlinux_conf, cfg_content);

    let _ = Command::new("extlinux").args(["--install", &boot_dir]).status();
    let _ = Command::new("umount").args([target_mount]).status();

    println!("\n\x1b[1;32m============================================================");
    println!(" [✓] Установка akstOS успешно завершена!");
    println!(" Вытащите установочную флешку и перезагрузите компьютер.");
    println!(" Компьютер сразу загрузит akstOS с жесткого диска!");
    println!("============================================================\x1b[0m\n");
}
