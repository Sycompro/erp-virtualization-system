-- Inicialización de Base de Datos ERP Virtualization
-- Ejecutado automáticamente al crear el container PostgreSQL

-- Crear extensiones necesarias
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- Tabla de usuarios
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    full_name VARCHAR(100),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_login TIMESTAMP,
    is_active BOOLEAN DEFAULT true,
    failed_login_attempts INTEGER DEFAULT 0,
    locked_until TIMESTAMP NULL
);

-- Tabla de credenciales biométricas
CREATE TABLE IF NOT EXISTS biometric_credentials (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    credential_id VARCHAR(255) UNIQUE NOT NULL,
    public_key TEXT NOT NULL,
    counter INTEGER DEFAULT 0,
    device_name VARCHAR(100),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_used TIMESTAMP,
    is_active BOOLEAN DEFAULT true
);

-- Tabla de sesiones activas
CREATE TABLE IF NOT EXISTS user_sessions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    session_token VARCHAR(255) UNIQUE NOT NULL,
    device_id VARCHAR(100),
    device_info JSONB,
    ip_address INET,
    user_agent TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_activity TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMP NOT NULL,
    is_active BOOLEAN DEFAULT true
);

-- Tabla de aplicaciones disponibles
CREATE TABLE IF NOT EXISTS applications (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(100) NOT NULL,
    app_type VARCHAR(50) NOT NULL,
    category VARCHAR(50) NOT NULL,
    description TEXT,
    image_name VARCHAR(200) NOT NULL,
    display_protocol VARCHAR(20) NOT NULL DEFAULT 'VNC',
    default_port INTEGER,
    icon_url VARCHAR(255),
    system_requirements JSONB,
    supported_features JSONB,
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Tabla de containers activos
CREATE TABLE IF NOT EXISTS active_containers (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    container_id VARCHAR(100) UNIQUE NOT NULL,
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    session_id UUID REFERENCES user_sessions(id) ON DELETE CASCADE,
    application_id UUID REFERENCES applications(id),
    app_type VARCHAR(50) NOT NULL,
    status VARCHAR(20) DEFAULT 'starting',
    vnc_port INTEGER,
    rdp_port INTEGER,
    container_ip INET,
    resources_allocated JSONB,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    started_at TIMESTAMP,
    stopped_at TIMESTAMP,
    last_activity TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Tabla de logs de actividad
CREATE TABLE IF NOT EXISTS activity_logs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    session_id UUID REFERENCES user_sessions(id) ON DELETE SET NULL,
    container_id UUID REFERENCES active_containers(id) ON DELETE SET NULL,
    action VARCHAR(50) NOT NULL,
    details JSONB,
    ip_address INET,
    user_agent TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Tabla de configuración del sistema
CREATE TABLE IF NOT EXISTS system_config (
    key VARCHAR(100) PRIMARY KEY,
    value JSONB NOT NULL,
    description TEXT,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_by UUID REFERENCES users(id)
);

-- Índices para optimización
CREATE INDEX IF NOT EXISTS idx_users_username ON users(username);
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
CREATE INDEX IF NOT EXISTS idx_users_active ON users(is_active);

CREATE INDEX IF NOT EXISTS idx_biometric_user_id ON biometric_credentials(user_id);
CREATE INDEX IF NOT EXISTS idx_biometric_credential_id ON biometric_credentials(credential_id);

CREATE INDEX IF NOT EXISTS idx_sessions_user_id ON user_sessions(user_id);
CREATE INDEX IF NOT EXISTS idx_sessions_token ON user_sessions(session_token);
CREATE INDEX IF NOT EXISTS idx_sessions_active ON user_sessions(is_active);
CREATE INDEX IF NOT EXISTS idx_sessions_expires ON user_sessions(expires_at);

CREATE INDEX IF NOT EXISTS idx_applications_type ON applications(app_type);
CREATE INDEX IF NOT EXISTS idx_applications_category ON applications(category);
CREATE INDEX IF NOT EXISTS idx_applications_active ON applications(is_active);

CREATE INDEX IF NOT EXISTS idx_containers_user_id ON active_containers(user_id);
CREATE INDEX IF NOT EXISTS idx_containers_session_id ON active_containers(session_id);
CREATE INDEX IF NOT EXISTS idx_containers_status ON active_containers(status);
CREATE INDEX IF NOT EXISTS idx_containers_container_id ON active_containers(container_id);

CREATE INDEX IF NOT EXISTS idx_activity_user_id ON activity_logs(user_id);
CREATE INDEX IF NOT EXISTS idx_activity_created_at ON activity_logs(created_at);
CREATE INDEX IF NOT EXISTS idx_activity_action ON activity_logs(action);

-- Insertar aplicaciones por defecto
INSERT INTO applications (name, app_type, category, description, image_name, display_protocol, default_port, system_requirements, supported_features) VALUES
('SAP GUI', 'sap', 'ERP Systems', 'Sistema ERP empresarial SAP con interfaz completa', 'erp-virtualization/sap-gui:latest', 'VNC', 5900, 
 '{"min_ram_gb": 4, "recommended_ram_gb": 8, "gpu_required": false, "min_bandwidth_mbps": 5, "recommended_bandwidth_mbps": 15}',
 '["Streaming HD", "Touch optimizado", "Clipboard sync", "Impresión remota"]'),

('Microsoft Office', 'office', 'Office Suite', 'Word, Excel, PowerPoint, Outlook completos', 'erp-virtualization/office:latest', 'RDP', 3389,
 '{"min_ram_gb": 2, "recommended_ram_gb": 4, "gpu_required": false, "min_bandwidth_mbps": 3, "recommended_bandwidth_mbps": 10}',
 '["Streaming HD", "Touch optimizado", "Clipboard sync", "Impresión remota", "Compartir archivos"]'),

('AutoCAD', 'autocad', 'Design & CAD', 'Diseño CAD profesional 2D y 3D', 'erp-virtualization/autocad:latest', 'RDP', 3390,
 '{"min_ram_gb": 8, "recommended_ram_gb": 16, "gpu_required": true, "min_bandwidth_mbps": 10, "recommended_bandwidth_mbps": 25}',
 '["Streaming 4K", "Aceleración GPU", "Precisión de color", "Touch optimizado"]'),

('LibreOffice', 'libreoffice', 'Office Suite', 'Suite ofimática libre: Writer, Calc, Impress', 'erp-virtualization/libreoffice:latest', 'VNC', 5901,
 '{"min_ram_gb": 2, "recommended_ram_gb": 4, "gpu_required": false, "min_bandwidth_mbps": 3, "recommended_bandwidth_mbps": 10}',
 '["Streaming HD", "Touch optimizado", "Clipboard sync", "Código abierto"]'),

('Windows Desktop', 'windows-desktop', 'Full Desktop', 'Escritorio Windows completo con todas las aplicaciones', 'erp-virtualization/windows-desktop:latest', 'RDP', 3391,
 '{"min_ram_gb": 4, "recommended_ram_gb": 8, "gpu_required": false, "min_bandwidth_mbps": 5, "recommended_bandwidth_mbps": 15}',
 '["Escritorio completo", "Múltiples aplicaciones", "Gestión de archivos", "Streaming HD"]')

ON CONFLICT (name) DO NOTHING;

-- Crear usuario administrador por defecto
-- Password: admin123 (hasheado con bcrypt)
INSERT INTO users (username, email, password_hash, full_name) VALUES
('admin', 'admin@erpvirtualization.com', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj3bp.Gm.F5e', 'Administrador del Sistema')
ON CONFLICT (username) DO NOTHING;

-- Crear usuarios de prueba para las 5 tablets
INSERT INTO users (username, email, password_hash, full_name) VALUES
('tablet1', 'tablet1@empresa.com', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj3bp.Gm.F5e', 'Usuario Tablet 1'),
('tablet2', 'tablet2@empresa.com', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj3bp.Gm.F5e', 'Usuario Tablet 2'),
('tablet3', 'tablet3@empresa.com', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj3bp.Gm.F5e', 'Usuario Tablet 3'),
('tablet4', 'tablet4@empresa.com', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj3bp.Gm.F5e', 'Usuario Tablet 4'),
('tablet5', 'tablet5@empresa.com', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj3bp.Gm.F5e', 'Usuario Tablet 5')
ON CONFLICT (username) DO NOTHING;

-- Insertar configuración inicial del sistema
INSERT INTO system_config (key, value, description) VALUES
('max_concurrent_users', '5', 'Máximo número de usuarios concurrentes'),
('session_timeout_minutes', '60', 'Timeout de sesión en minutos'),
('max_containers_per_user', '3', 'Máximo containers por usuario'),
('enable_biometric_auth', 'true', 'Habilitar autenticación biométrica'),
('enable_gpu_acceleration', 'true', 'Habilitar aceleración GPU para aplicaciones CAD'),
('default_vnc_resolution', '"1920x1080"', 'Resolución por defecto para VNC'),
('max_session_duration_hours', '8', 'Duración máxima de sesión en horas'),
('enable_activity_logging', 'true', 'Habilitar logging de actividad'),
('backup_retention_days', '30', 'Días de retención de backups'),
('enable_ssl_only', 'true', 'Forzar conexiones SSL únicamente')
ON CONFLICT (key) DO NOTHING;

-- Función para limpiar sesiones expiradas
CREATE OR REPLACE FUNCTION cleanup_expired_sessions()
RETURNS INTEGER AS $$
DECLARE
    deleted_count INTEGER;
BEGIN
    -- Marcar sesiones expiradas como inactivas
    UPDATE user_sessions 
    SET is_active = false 
    WHERE expires_at < CURRENT_TIMESTAMP AND is_active = true;
    
    GET DIAGNOSTICS deleted_count = ROW_COUNT;
    
    -- Log de limpieza
    INSERT INTO activity_logs (action, details) 
    VALUES ('cleanup_expired_sessions', json_build_object('deleted_count', deleted_count));
    
    RETURN deleted_count;
END;
$$ LANGUAGE plpgsql;

-- Función para actualizar última actividad
CREATE OR REPLACE FUNCTION update_last_activity()
RETURNS TRIGGER AS $$
BEGIN
    NEW.last_activity = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Triggers para actualizar timestamps automáticamente
CREATE TRIGGER update_users_updated_at
    BEFORE UPDATE ON users
    FOR EACH ROW
    EXECUTE FUNCTION update_last_activity();

CREATE TRIGGER update_applications_updated_at
    BEFORE UPDATE ON applications
    FOR EACH ROW
    EXECUTE FUNCTION update_last_activity();

CREATE TRIGGER update_containers_last_activity
    BEFORE UPDATE ON active_containers
    FOR EACH ROW
    EXECUTE FUNCTION update_last_activity();

-- Crear vista para estadísticas del sistema
CREATE OR REPLACE VIEW system_stats AS
SELECT 
    (SELECT COUNT(*) FROM users WHERE is_active = true) as active_users,
    (SELECT COUNT(*) FROM user_sessions WHERE is_active = true) as active_sessions,
    (SELECT COUNT(*) FROM active_containers WHERE status = 'running') as running_containers,
    (SELECT COUNT(*) FROM applications WHERE is_active = true) as available_applications,
    (SELECT COUNT(*) FROM activity_logs WHERE created_at > CURRENT_TIMESTAMP - INTERVAL '24 hours') as activities_last_24h;

-- Comentarios para documentación
COMMENT ON TABLE users IS 'Usuarios del sistema ERP Virtualization';
COMMENT ON TABLE biometric_credentials IS 'Credenciales biométricas FIDO2/WebAuthn';
COMMENT ON TABLE user_sessions IS 'Sesiones activas de usuarios';
COMMENT ON TABLE applications IS 'Aplicaciones disponibles para virtualización';
COMMENT ON TABLE active_containers IS 'Containers Docker activos';
COMMENT ON TABLE activity_logs IS 'Logs de actividad del sistema';
COMMENT ON TABLE system_config IS 'Configuración del sistema';

-- Mensaje de finalización
DO $$
BEGIN
    RAISE NOTICE '✅ Base de datos ERP Virtualization inicializada correctamente';
    RAISE NOTICE '👤 Usuario admin creado (password: admin123)';
    RAISE NOTICE '📱 5 usuarios de tablet creados (tablet1-tablet5, password: admin123)';
    RAISE NOTICE '📊 % aplicaciones configuradas', (SELECT COUNT(*) FROM applications);
END $$;