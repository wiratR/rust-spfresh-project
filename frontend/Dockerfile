# 1. ใช้ official Rust image เป็น base image
FROM rust:latest AS builder

# 2. ติดตั้ง wasm32 target และ trunk
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk wasm-bindgen-cli

# 3. ตั้ง working directory
WORKDIR /app

# 4. คัดลอกไฟล์ manifest ก่อนเพื่อให้ build เร็วขึ้น
COPY Cargo.toml Cargo.lock ./

# 5. คัดลอกไฟล์ index.html และ source code
COPY index.html ./
COPY src ./src

# 6. ตั้ง ENV เพื่อให้ trunk เห็นตอน compile
ARG BACKEND_URL
ENV BACKEND_URL=${BACKEND_URL}

RUN echo "🔧 BACKEND_URL = $BACKEND_URL" \
    && trunk build --release --public-url /

# 7. ใช้ nginx serve ไฟล์ frontend
FROM nginx:alpine

COPY --from=builder /app/dist /usr/share/nginx/html

EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
