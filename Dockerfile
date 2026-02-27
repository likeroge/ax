# Этап сборки
FROM rust:1.88-alpine AS builder
RUN apk add --no-cache musl-dev openssl-dev
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN cargo vendor /vendor && cargo fetch --locked
COPY src ./src
RUN cargo build --release
# Финальный образ
FROM alpine:latest
RUN apk add --no-cache ca-certificates
COPY --from=builder /app/target/release/ruapi /usr/local/bin/
EXPOSE 3000
CMD ["ruapi"]
