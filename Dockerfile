# Dockerfile mínimo - Solo panel estático
FROM nginx:alpine

# Copiar panel HTML
COPY server-railway/static/ /usr/share/nginx/html/

# Configurar nginx para SPA
RUN echo 'server { \
    listen 8080; \
    server_name _; \
    root /usr/share/nginx/html; \
    index index.html; \
    location / { \
        try_files $uri $uri/ /index.html; \
    } \
    location /health { \
        return 200 "OK"; \
        add_header Content-Type text/plain; \
    } \
}' > /etc/nginx/conf.d/default.conf

EXPOSE 8080

CMD ["nginx", "-g", "daemon off;"]