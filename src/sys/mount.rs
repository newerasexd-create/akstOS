use std::fs;
use std::path::Path;

/// Монтирование псевдо-файловых систем Linux
pub fn mount_essential_filesystems() {
    println!("\x1b[1;36m[akst-init] Монтирование виртуальных файловых систем Linux...\x1b[0m");

    let targets = [
        ("proc", "/proc", "proc", 0),
        ("sysfs", "/sys", "sysfs", 0),
        ("devtmpfs", "/dev", "devtmpfs", 0),
        ("tmpfs", "/tmp", "tmpfs", 0),
        ("tmpfs", "/run", "tmpfs", 0),
    ];

    for (source, target, fstype, flags) in targets {
        if !Path::new(target).exists() {
            let _ = fs::create_dir_all(target);
        }

        let c_src = std::ffi::CString::new(source).unwrap();
        let c_target = std::ffi::CString::new(target).unwrap();
        let c_type = std::ffi::CString::new(fstype).unwrap();

        unsafe {
            let res = libc::mount(
                c_src.as_ptr(),
                c_target.as_ptr(),
                c_type.as_ptr(),
                flags,
                std::ptr::null(),
            );
            if res != 0 {
                // Если не получилось (например, запущено в непривилегированном контейнере), логируем
                eprintln!("\x1b[33m[akst-init] Замечание: {} не удалось примонтировать (код: {})\x1b[0m", target, res);
            }
        }
    }

    // Создание каталогов для колледжа
    let college_dirs = [
        "/home/student",
        "/home/student/labs",
        "/home/student/projects",
        "/etc",
        "/var/log",
    ];

    for dir in college_dirs {
        let _ = fs::create_dir_all(dir);
    }

    // Создание /etc/hostname
    let _ = fs::write("/etc/hostname", "akstos-college\n");
}
