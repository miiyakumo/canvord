# ===== Stage 1: Build using trunk =====
FROM rust:1.88.0-trixie AS builder

# 安装构建依赖
RUN apt-get update && apt-get install -y curl pkg-config libssl-dev

# 安装 trunk 和 wasm target
RUN rustup target add wasm32-unknown-unknown \
    && cargo install trunk

WORKDIR /app

# 复制项目文件（建议使用 .dockerignore 排除 target/ 等无用文件）
COPY canvord_frontend ./frontend
COPY logo.svg .

# 构建前端资源，输出到 /app/dist
WORKDIR /app/frontend
RUN trunk build --release --dist /app/dist

# ===== Stage 2: Nginx 部署静态资源 =====
FROM nginx:stable-alpine AS deploy

# 清理默认页面
RUN rm -rf /usr/share/nginx/html/*

# 拷贝构建产物到 nginx html 目录
COPY --from=builder /app/dist /usr/share/nginx/html

# 添加自定义 nginx.conf
# COPY nginx.conf /etc/nginx/nginx.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
