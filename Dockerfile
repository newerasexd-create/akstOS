# Двухэтапная сборка akstOS без засорения хостовой системы
FROM rust:alpine as builder

RUN apk add --no-cache musl-dev

WORKDIR /app
COPY Cargo.toml Cargo.lock* ./
COPY src ./src

# Сборка статического бинарника PID 1
RUN cargo build --release --target x86_64-unknown-linux-musl

# Этап упаковки
FROM alpine:latest

RUN apk add --no-cache cpio gzip qemu-system-x86_64

WORKDIR /akstos
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/akst_os /akstos/init
COPY config/os-release /akstos/os-release

CMD ["echo", "akstOS образ собран. Запустите build.sh для создания загрузочного initramfs."]
