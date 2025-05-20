FROM rust:latest as builder

WORKDIR /build

COPY . .

RUN apt-get update && apt-get install -y musl-tools

WORKDIR /build/src/contrib/text_item_api

RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl -p text_item_api

FROM debian:bullseye-slim

WORKDIR /app

COPY --from=builder /build/src/contrib/text_item_api/target/x86_64-unknown-linux-musl/release/text_item_api /app/text_item_api

ENTRYPOINT ["/app/text_item_api"]