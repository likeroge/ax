# Этап сборки
FROM rust:1.88-alpine AS builder
RUN apk add --no-cache musl-dev openssl-dev
WORKDIR /app
COPY . .
RUN cargo vendor /vendor && cargo fetch --locked
RUN cargo build --release
# Финальный образ
FROM alpine:latest
RUN apk add --no-cache ca-certificates
COPY --from=builder /app/target/release/ruapi /usr/local/bin/
EXPOSE 3000
CMD ["ruapi"]
