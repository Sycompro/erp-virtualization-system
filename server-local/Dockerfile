# Dockerfile para Servidor Local de Streaming
FROM rust:1.75-slim as builder

# Instalar dependencias de compilación
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copiar manifiestos de Cargo
COPY Cargo.toml Cargo.lock ./

# Crear src dummy para cachear dependencias
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Construir dependencias
RUN cargo build --release && rm -rf src

# Copiar código fuente real
COPY src ./src

# Construir aplicación final
RUN cargo build --release

# Etapa de runtime
FROM debian:bookworm-slim

# Instalar dependencias de runtime y Docker
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    docker.io \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean

# Crear usuario para el servicio
RUN useradd -r -s /bin/false -m -d /app -G docker local-user

# Copiar binario
COPY --from=builder /app/target/release/erp-local-server /app/
RUN chown local-user:local-user /app/erp-local-server

# Cambiar a usuario del servicio
USER local-user
WORKDIR /app

# Exponer puertos
EXPOSE 8080
EXPOSE 5900-5910
EXPOSE 3389-3399

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Variables de entorno
ENV RUST_LOG=info
ENV RAILWAY_API_URL=https://your-railway-app.railway.app

# Comando de inicio
CMD ["./erp-local-server"]