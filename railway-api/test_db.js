const { Client } = require('pg');
const client = new Client({ connectionString: 'postgresql://postgres:ITqVqDWQbyXlpdXgkdPwHnTlcIMFRkpf@shortline.proxy.rlwy.net:16432/railway' });
client.connect().then(() => client.query("SELECT username, password_hash FROM users WHERE username = 'admin'")).then(res => { console.log(res.rows); client.end() }).catch(err => console.log(err));
