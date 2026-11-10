/// Управление питанием системы akstOS для старых компьютеров

pub fn sync_filesystems() {
    println!("[akstOS] Сохранение данных на диск...");
    unsafe {
        libc::sync();
    }
}

pub fn reboot() -> ! {
    sync_filesystems();
    println!("\x1b[1;33m[akstOS] Перезагрузка компьютера...\x1b[0m");
    unsafe {
        libc::reboot(libc::RB_AUTOBOOT);
    }
    loop {}
}

pub fn poweroff() -> ! {
    sync_filesystems();
    println!("\x1b[1;32m[akstOS] Работа завершена. Теперь компьютер можно безопасно выключить.\x1b[0m");
    unsafe {
        libc::reboot(libc::RB_POWER_OFF);
    }
    loop {}
}
