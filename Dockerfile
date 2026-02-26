FROM rust:1.88-alpine as builder
RUN apk add --no-cache musl-dev openssl-dev
WORKDIR /app
COPY . .
RUN cargo build --release

FROM alpine:latest
RUN apk add --no-cache ca-certificates
COPY --from=builder /app/target/release/ruapi /usr/local/bin/
EXPOSE 3000
CMD ["ruapi"]
