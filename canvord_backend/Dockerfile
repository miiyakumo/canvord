FROM rust:1.88.0-trixie as builder

WORKDIR /app/backend

# 只复制后端代码目录
COPY canvord_backend .

RUN cargo build --release

FROM debian:trixie-slim
WORKDIR /app

COPY --from=builder /app/backend/target/release/canvord_backend .

EXPOSE 8000

CMD ["./canvord_backend"]