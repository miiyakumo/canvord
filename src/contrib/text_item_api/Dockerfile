FROM rust:latest as builder

WORKDIR /app

COPY . .

# 显式指定构建目标
RUN cargo build --release --bin text_item_api

RUN ls -lh /app/target/release

FROM debian:bullseye-slim

WORKDIR /app

COPY --from=builder /app/target/release/text_item_api /app/text_item_api

EXPOSE 8080

ENTRYPOINT ["/app/text_item_api"]