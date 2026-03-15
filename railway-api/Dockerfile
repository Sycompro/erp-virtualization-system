# Dockerfile optimizado para Railway API Service
FROM rust:1.93-slim as builder

# Instalar dependencias de compilación
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copiar todo el proyecto
COPY . .

# Construir aplicación
RUN cargo build --release --bin erp-railway-api

# Etapa de runtime
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libpq5 \
    curl \
    && rm -rf /var/lib/apt/lists/*

RUN useradd -r -s /bin/false -m -d /app railway-user

COPY --from=builder /app/target/release/erp-railway-api /app/
RUN chown railway-user:railway-user /app/erp-railway-api

USER railway-user
WORKDIR /app

EXPOSE $PORT

ENV RUST_LOG=info

CMD ["./erp-railway-api"]