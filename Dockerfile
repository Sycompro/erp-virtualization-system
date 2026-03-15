# Dockerfile para Railway - Versión Simple
FROM node:18-alpine

WORKDIR /app

# Copiar archivos de la API
COPY railway-api/package*.json ./
RUN npm install

COPY railway-api/ ./

# Crear un servidor simple que funcione
COPY railway-api/static/ ./static/

EXPOSE $PORT

# Usar Node.js en lugar de Rust para deployment rápido
CMD ["node", "initdb.js"]