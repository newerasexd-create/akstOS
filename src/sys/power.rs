/// Управление питанием системы akstOS

pub fn sync_filesystems() {
    println!("[akstOS] Синхронизация данных на диск...");
    unsafe {
        libc::sync();
    }
}

pub fn reboot() -> ! {
    sync_filesystems();
    println!("\x1b[1;33m[akstOS] Перезагрузка образовательной системы...\x1b[0m");
    unsafe {
        libc::reboot(libc::RB_AUTOBOOT);
    }
    loop {}
}

pub fn poweroff() -> ! {
    sync_filesystems();
    println!("\x1b[1;31m[akstOS] Завершение работы. До встречи на парах!\x1b[0m");
    unsafe {
        libc::reboot(libc::RB_POWER_OFF);
    }
    loop {}
}
