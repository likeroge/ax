# Этап сборки
FROM rust:1.88-alpine AS builder
RUN apk add --no-cache musl-dev openssl-dev
WORKDIR /app
# Копируем только файлы зависимостей
COPY Cargo.toml Cargo.lock ./
# Создаем фиктивный main.rs для кэширования зависимостей
RUN mkdir -p src && \
  echo "fn main() {}" > src/main.rs
# Скачиваем и компилируем зависимости
RUN cargo build --release
# Копируем реальный исходный код
# COPY src ./src
COPY . .
# Пересобираем с реальным кодом
RUN cargo build --release

#COPY . .
#RUN cargo vendor /vendor && cargo fetch --locked
#RUN cargo build --release

# Финальный образ
FROM alpine:latest
RUN apk add --no-cache ca-certificates
COPY --from=builder /app/target/release/ruapi /usr/local/bin/ruapi

RUN mkdir -p /usr/local/bin/static
RUN mkdir -p /usr/local/bin/templates

EXPOSE 3000
CMD ["ruapi"]
