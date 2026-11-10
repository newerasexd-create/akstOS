#!/usr/bin/env bash
set -e

echo -e "\e[1;36m=== Сборка образовательного дистрибутива akstOS ===\e[0m"

# 1. Проверка Rust и таргета musl
if ! rustup target list | grep -q "x86_64-unknown-linux-musl (installed)"; then
    echo "[+] Установка musl target..."
    rustup target add x86_64-unknown-linux-musl
fi

# 2. Компиляция
echo "[+] Компиляция akstOS PID 1 (Release)..."
cargo build --release --target x86_64-unknown-linux-musl

# 3. Формирование файловой системы
echo "[+] Формирование дерева каталогов rootfs..."
rm -rf rootfs
mkdir -p rootfs/{bin,sbin,etc,proc,sys,dev,tmp,run,home/student/labs,home/student/projects}

cp target/x86_64-unknown-linux-musl/release/akst_os rootfs/init
chmod +x rootfs/init
cp config/os-release rootfs/etc/os-release

# 4. Упаковка в initramfs
echo "[+] Упаковка в initramfs.cpio.gz..."
cd rootfs
find . -print0 | cpio --null -ov --format=newc | gzip -9 > ../initramfs.cpio.gz
cd ..

echo -e "\e[1;32m[✓] initramfs.cpio.gz успешно собран!\e[0m"

# 5. Опциональный запуск в QEMU
if command -v qemu-system-x86_64 &> /dev/null; then
    KERNEL=$(ls -1 /boot/vmlinuz* 2>/dev/null | head -n 1 || true)
    if [ -n "$KERNEL" ]; then
        echo "[+] Запуск akstOS в QEMU с ядром $KERNEL..."
        qemu-system-x86_64 \
            -kernel "$KERNEL" \
            -initrd initramfs.cpio.gz \
            -append "console=ttyS0 quiet panic=1" \
            -m 256M \
            -nographic
    else
        echo "Ядро /boot/vmlinuz не найдено на хосте. Скопируйте ядро Linux и запустите через QEMU."
    fi
fi
