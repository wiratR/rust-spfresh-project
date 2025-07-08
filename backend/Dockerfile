# Stage 1: Build stage
FROM rust:latest AS builder

RUN apt-get update && apt-get upgrade -y && \
    apt-get install -y cmake build-essential pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app/backend

# COPY ใช้ relative path จาก context ที่ส่งเข้ามา (project root)
COPY backend/Cargo.toml backend/Cargo.lock ./
COPY backend/fastembed-rs ./fastembed-rs
COPY backend/spfresh ./spfresh

# # สร้าง dummy main.rs เพื่อ build dependencies
# RUN mkdir src && echo "fn main() {}" > src/main.rs
# RUN cargo build --release
# RUN rm -rf src

# Copy source code จริง
COPY backend/src ./src

RUN cargo build --release

# Stage 2: Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get upgrade -y && \
    apt-get install -y libssl3 ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app/backend

COPY --from=builder /app/backend/target/release/backend .

EXPOSE 8000

CMD ["./backend"]
