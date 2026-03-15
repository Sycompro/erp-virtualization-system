const { Client } = require('pg');
const bcrypt = require('bcryptjs');

const connectionString = "postgresql://postgres:ITqVqDWQbyXlpdXgkdPwHnTlcIMFRkpf@shortline.proxy.rlwy.net:16432/railway";
const client = new Client({ connectionString });

async function run() {
    await client.connect();
    console.log("Conectado a la base de datos de Railway.");

    // Hash real de admin123
    const hash = bcrypt.hashSync('admin123', 10);
    console.log("Nuevo Hash:", hash);

    // Fix trigger function
    await client.query(`
        CREATE OR REPLACE FUNCTION update_updated_at()
        RETURNS TRIGGER AS $$
        BEGIN
            NEW.updated_at = CURRENT_TIMESTAMP;
            RETURN NEW;
        END;
        $$ LANGUAGE plpgsql;

        DROP TRIGGER IF EXISTS update_users_updated_at ON users;
        CREATE TRIGGER update_users_updated_at
            BEFORE UPDATE ON users
            FOR EACH ROW
            EXECUTE FUNCTION update_updated_at();
    `);

    // Update admin user
    const res = await client.query("UPDATE users SET password_hash = $1 WHERE username = 'admin' OR username LIKE 'tablet%'", [hash]);
    console.log(`¡Usuarios actualizados! Filas afectadas: ${res.rowCount}`);

    await client.end();
}

run().catch(console.error);
