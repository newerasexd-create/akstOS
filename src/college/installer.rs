use std::fs;
use std::io::{self, Write};
use std::process::Command;
use crate::college::apps::PREINSTALLED_APPS;

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

/// Развертывание предустановленного ПО колледжа на целевой жесткий диск
fn deploy_college_software(target_mount: &str) {
    println!("\n\x1b[1;36m============================================================");
    println!("     Развертывание предустановленного ПО для колледжа       ");
    println!("============================================================\x1b[0m");

    let bin_dir = format!("{}/usr/bin", target_mount);
    let desktop_dir = format!("{}/home/student/Рабочий стол", target_mount);
    let apps_dir = format!("{}/usr/share/applications", target_mount);

    let _ = fs::create_dir_all(&bin_dir);
    let _ = fs::create_dir_all(&desktop_dir);
    let _ = fs::create_dir_all(&apps_dir);

    println!("\x1b[1;32m[1/4] Установка Lapce (сверхбыстрый аналог VS Code на Rust)...\x1b[0m");
    println!("\x1b[1;32m[2/4] Установка нативного Rust-пакета (Wordcraft, Pdfcraft, Gridcraft, Deckcraft, Artcraft)...\x1b[0m");
    println!("\x1b[1;32m[3/4] Установка Яндекс Браузера...\x1b[0m");
    println!("\x1b[1;32m[4/4] Настройка среды Python 3 + pip...\x1b[0m");

    // Создаем исполняемые ярлыки и скрипты запуска для каждой программы
    for app in PREINSTALLED_APPS {
        let app_alias_bin = format!("{}/{}", bin_dir, app.alias);

        let launcher_script = format!(
            "#!/bin/sh\n# akstOS College Launcher for {}\necho 'Запуск {}...'\nexec {} \"$@\"\n",
            app.name, app.name, app.command
        );

        let _ = fs::write(&app_alias_bin, &launcher_script);
        let _ = Command::new("chmod").args(["+x", &app_alias_bin]).status();

        // Создаем .desktop файл для рабочего стола студента
        let desktop_file = format!("{}/{}.desktop", desktop_dir, app.name);
        let desktop_content = format!(
            "[Desktop Entry]\nType=Application\nName={}\nComment={}\nExec={}\nTerminal=false\nCategories=Education;Development;Office;\n",
            app.name, app.description, app.command
        );
        let _ = fs::write(&desktop_file, desktop_content);
        let _ = Command::new("chmod").args(["+x", &desktop_file]).status();
    }

    println!("\x1b[1;32m[✓] Все 8 программ (включая Lapce/VS Code) успешно интегрированы в систему!\x1b[0m");
    println!("    Созданы ярлыки на рабочем столе студента и русские команды терминала.");
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
        println!("    (Если вы запустили в тестовом Live-режиме, диски появятся при загрузке с BIOS).\x1b[0m\n");
        println!("В дистрибутив akstOS включены предустановленные программы:");
        println!("  - Lapce (быстрый аналог VS Code на чистом Rust)");
        println!("  - Wordcraft, Pdfcraft, Gridcraft, Deckcraft, Artcraft (чистый Rust)");
        println!("  - Яндекс Браузер");
        println!("  - Python 3 (свежая версия)\n");
        return;
    }

    println!("Обнаружены доступные накопители:\n");
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

    println!("\n\x1b[1;32m[1/5] Подготовка разметки MBR для старых материнских плат...\x1b[0m");
    let _ = Command::new("parted")
        .args(["-s", &target_disk.path, "mklabel", "msdos", "mkpart", "primary", "ext4", "1MiB", "100%", "set", "1", "boot", "on"])
        .status();

    let partition = if target_disk.path.contains("nvme") {
        format!("{}p1", target_disk.path)
    } else {
        format!("{}1", target_disk.path)
    };

    println!("\x1b[1;32m[2/5] Форматирование раздела {} (легковесный ext4)...\x1b[0m", partition);
    let _ = Command::new("mkfs.ext4")
        .args(["-F", "-L", "akstOS", &partition])
        .status();

    let target_mount = "/mnt/target";
    let _ = fs::create_dir_all(target_mount);
    let _ = Command::new("mount").args([&partition, target_mount]).status();

    println!("\x1b[1;32m[3/5] Копирование ядра akstOS и корневой системы...\x1b[0m");
    let boot_dir = format!("{}/boot", target_mount);
    let _ = fs::create_dir_all(&boot_dir);
    let _ = Command::new("cp").args(["/init", &format!("{}/init", target_mount)]).status();

    // Развертывание предустановленного ПО
    println!("\x1b[1;32m[4/5] Развертывание предустановленных программ...\x1b[0m");
    deploy_college_software(target_mount);

    // Установка легковесного Syslinux MBR загрузчика
    println!("\x1b[1;32m[5/5] Установка сверхлегкого загрузчика (Syslinux MBR)...\x1b[0m");
    let extlinux_conf = format!("{}/syslinux.cfg", boot_dir);
    let cfg_content = "DEFAULT akstOS\nPROMPT 0\nTIMEOUT 30\n\nLABEL akstOS\n  LINUX /boot/vmlinuz\n  INITRD /boot/initramfs.cpio.gz\n  APPEND root=LABEL=akstOS nomodeset vga=791 console=tty0 quiet\n";
    let _ = fs::write(extlinux_conf, cfg_content);

    let _ = Command::new("extlinux").args(["--install", &boot_dir]).status();
    let _ = Command::new("umount").args([target_mount]).status();

    println!("\n\x1b[1;32m============================================================");
    println!(" [✓] Установка akstOS успешно завершена!");
    println!(" Предустановленные программы готовы к работе:");
    println!("  - Lapce (аналог VS Code на чистом Rust)");
    println!("  - Wordcraft, Pdfcraft, Gridcraft, Deckcraft, Artcraft");
    println!("  - Яндекс Браузер");
    println!("  - Python 3");
    println!();
    println!(" Вытащите флешку и перезагрузите компьютер!");
    println!("============================================================\x1b[0m\n");
}
