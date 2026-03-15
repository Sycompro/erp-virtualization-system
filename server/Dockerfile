# Dockerfile para ERP Virtualization Server

# Etapa de construcción
FROM rust:1.75-slim as builder

# Instalar dependencias del sistema
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Crear directorio de trabajo
WORKDIR /app

# Copiar archivos de configuración de Cargo
COPY Cargo.toml Cargo.lock ./

# Crear directorio src con main.rs dummy para cachear dependencias
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Construir dependencias (se cachea si Cargo.toml no cambia)
RUN cargo build --release && rm -rf src

# Copiar código fuente real
COPY src ./src

# Construir aplicación
RUN cargo build --release

# Etapa de runtime
FROM debian:bookworm-slim

# Instalar dependencias de runtime
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libpq5 \
    curl \
    docker.io \
    && rm -rf /var/lib/apt/lists/*

# Crear usuario no-root
RUN useradd -r -s /bin/false -m -d /app erp-user

# Crear directorios necesarios
RUN mkdir -p /app/data /app/logs /app/config \
    && chown -R erp-user:erp-user /app

# Copiar binario desde etapa de construcción
COPY --from=builder /app/target/release/erp-virtualization-server /app/

# Copiar archivos de configuración
COPY config/ /app/config/

# Cambiar a usuario no-root
USER erp-user
WORKDIR /app

# Exponer puertos
EXPOSE 8080 8443

# Variables de entorno por defecto
ENV RUST_LOG=info
ENV DATABASE_URL=postgresql://erp_user:password@localhost:5432/erp_db
ENV REDIS_URL=redis://localhost:6379
ENV JWT_SECRET=change-this-in-production

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Comando por defecto
CMD ["./erp-virtualization-server"]