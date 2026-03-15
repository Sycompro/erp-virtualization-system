-- Insert default applications
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

-- Create admin user (password: admin123)
INSERT INTO users (username, email, password_hash, full_name) VALUES
('admin', 'admin@erpvirtualization.com', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj3bp.Gm.F5e', 'Administrador del Sistema')
ON CONFLICT (username) DO NOTHING;

-- Create tablet users (password: admin123)
INSERT INTO users (username, email, password_hash, full_name) VALUES
('tablet1', 'tablet1@empresa.com', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj3bp.Gm.F5e', 'Usuario Tablet 1'),
('tablet2', 'tablet2@empresa.com', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj3bp.Gm.F5e', 'Usuario Tablet 2'),
('tablet3', 'tablet3@empresa.com', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj3bp.Gm.F5e', 'Usuario Tablet 3'),
('tablet4', 'tablet4@empresa.com', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj3bp.Gm.F5e', 'Usuario Tablet 4'),
('tablet5', 'tablet5@empresa.com', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj3bp.Gm.F5e', 'Usuario Tablet 5')
ON CONFLICT (username) DO NOTHING;