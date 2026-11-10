use std::fs;
use std::path::Path;

/// Монтирование виртуальных файловых систем с ограничением памяти под старые ПК
pub fn mount_essential_filesystems() {
    println!("\x1b[1;36m[Инициализация] Подключение файловой системы...\x1b[0m");

    let targets = [
        ("proc", "/proc", "proc", ""),
        ("sysfs", "/sys", "sysfs", ""),
        ("devtmpfs", "/dev", "devtmpfs", ""),
        // Ограничиваем tmpfs 16 Мегабайтами, чтобы не забить оперативную память старого ПК
        ("tmpfs", "/tmp", "tmpfs", "size=16M"),
        ("tmpfs", "/run", "tmpfs", "size=8M"),
    ];

    for (source, target, fstype, opts) in targets {
        if !Path::new(target).exists() {
            let _ = fs::create_dir_all(target);
        }

        let c_src = std::ffi::CString::new(source).unwrap();
        let c_target = std::ffi::CString::new(target).unwrap();
        let c_type = std::ffi::CString::new(fstype).unwrap();
        let c_opts = if opts.is_empty() {
            None
        } else {
            Some(std::ffi::CString::new(opts).unwrap())
        };

        unsafe {
            let data_ptr = match &c_opts {
                Some(o) => o.as_ptr() as *const libc::c_void,
                None => std::ptr::null(),
            };

            let res = libc::mount(
                c_src.as_ptr(),
                c_target.as_ptr(),
                c_type.as_ptr(),
                0,
                data_ptr,
            );

            if res != 0 {
                eprintln!("\x1b[33m[Замечание] Каталог {} не удалось примонтировать\x1b[0m", target);
            }
        }
    }

    // Создаем папки студента для занятий
    let student_dirs = [
        "/home/student",
        "/home/student/лабораторные",
        "/home/student/проекты",
        "/etc",
        "/var/log",
    ];

    for dir in student_dirs {
        let _ = fs::create_dir_all(dir);
    }

    // Имя компьютера в сети колледжа
    let _ = fs::write("/etc/hostname", "akstos-college\n");
}
