const { Client } = require('pg');
const fs = require('fs');
const path = require('path');

async function run() {
    // Railway URL proxy pública, extraída a partir de los permisos y de la contraseña de pg interna.
    const connectionString = "postgresql://postgres:ITqVqDWQbyXlpdXgkdPwHnTlcIMFRkpf@shortline.proxy.rlwy.net:16432/railway";

    console.log("Conectando a la base de datos de producción (Public Network)...");
    const client = new Client({
        connectionString: connectionString
    });

    try {
        await client.connect();
        console.log("Conexión exitosa. Leyendo init.sql...");

        const sqlPath = path.join(__dirname, 'database', 'init.sql');
        const sql = fs.readFileSync(sqlPath, 'utf8');

        console.log("Ejecutando script SQL...");
        await client.query(sql);
        console.log("¡Script SQL ejecutado exitosamente! La base de datos está lista.");

    } catch (e) {
        console.error("Error ejecutando SQL:", e);
    } finally {
        await client.end();
    }
}

run();
