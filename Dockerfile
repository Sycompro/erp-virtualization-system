# Dockerfile para Railway - ERP API Service
FROM rust:1.75-slim as builder

# Instalar dependencias de compilación
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copiar manifiestos de Cargo desde server-railway
COPY server-railway/Cargo.toml server-railway/Cargo.lock ./

# Crear src dummy para cachear dependencias
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Construir dependencias
RUN cargo build --release && rm -rf src

# Copiar código fuente real desde server-railway
COPY server-railway/src ./src

# Construir aplicación final
RUN cargo build --release

# Etapa de runtime
FROM debian:bookworm-slim

# Instalar dependencias de runtime
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libpq5 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Crear usuario no-root
RUN useradd -r -s /bin/false -m -d /app railway-user

# Copiar binario
COPY --from=builder /app/target/release/erp-railway-api /app/
RUN chown railway-user:railway-user /app/erp-railway-api

# Cambiar a usuario no-root
USER railway-user
WORKDIR /app

# Railway automáticamente asigna PORT
EXPOSE $PORT

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:$PORT/health || exit 1

# Variables de entorno
ENV RUST_LOG=info
ENV RAILWAY_ENVIRONMENT=production

# Comando de inicio
CMD ["./erp-railway-api"]