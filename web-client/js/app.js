/**
 * ERP Virtualization - Main Application
 * Controla la navegación, renderizado de apps y lógica principal
 */
(async () => {
    'use strict';

    // ─── DOM Elements ───
    const views = {
        login: document.getElementById('login-view'),
        dashboard: document.getElementById('dashboard-view'),
        viewer: document.getElementById('viewer-view')
    };

    const loginForm = document.getElementById('login-form');
    const loginError = document.getElementById('login-error');
    const loginBtn = document.getElementById('login-btn');
    const logoutBtn = document.getElementById('logout-btn');

    const greetingText = document.getElementById('greeting-text');
    const navUserInitial = document.getElementById('nav-user-initial');
    const statApps = document.getElementById('stat-apps');
    const statSessions = document.getElementById('stat-sessions');
    const appsGrid = document.getElementById('apps-grid');

    const viewerAppName = document.getElementById('viewer-app-name');
    const viewerStatus = document.getElementById('viewer-status');
    const viewerConnectingStatus = document.getElementById('viewer-connecting-status');
    const viewerOverlay = document.getElementById('viewer-overlay');
    const viewerBackBtn = document.getElementById('viewer-back-btn');
    const viewerDisconnectBtn = document.getElementById('viewer-disconnect-btn');
    const viewerFullscreenBtn = document.getElementById('viewer-fullscreen-btn');
    const remoteVideo = document.getElementById('remote-video');

    const filterTabs = document.querySelectorAll('.filter-tab');

    // ─── State ───
    let currentApps = [];
    let currentFilter = 'all';

    // ─── App Icons (emoji mapping) ───
    const APP_ICONS = {
        'sap': '🏢',
        'office': '📊',
        'autocad': '📐',
        'libreoffice': '📝',
        'windows-desktop': '🖥️'
    };

    const CATEGORY_MAP = {
        'ERP Systems': 'erp',
        'Office Suite': 'office',
        'Design & CAD': 'design',
        'Full Desktop': 'office'
    };

    // ═══════════════════════════════════════════════
    // NAVIGATION
    // ═══════════════════════════════════════════════
    function showView(viewName) {
        Object.values(views).forEach(v => v.classList.remove('active'));
        if (views[viewName]) {
            views[viewName].classList.add('active');
        }
    }

    // ═══════════════════════════════════════════════
    // LOGIN
    // ═══════════════════════════════════════════════
    loginForm.addEventListener('submit', async (e) => {
        e.preventDefault();
        const username = document.getElementById('username').value.trim();
        const password = document.getElementById('password').value;

        if (!username || !password) return;

        // UI: show loading
        loginBtn.querySelector('.btn-text').classList.add('hidden');
        loginBtn.querySelector('.btn-loader').classList.remove('hidden');
        loginBtn.disabled = true;
        loginError.classList.add('hidden');

        try {
            await AuthService.login(username, password);
            showToast('Sesión iniciada correctamente', 'success');
            await loadDashboard();
            showView('dashboard');
        } catch (error) {
            loginError.textContent = error.message || 'Error al iniciar sesión';
            loginError.classList.remove('hidden');
        } finally {
            loginBtn.querySelector('.btn-text').classList.remove('hidden');
            loginBtn.querySelector('.btn-loader').classList.add('hidden');
            loginBtn.disabled = false;
        }
    });

    logoutBtn.addEventListener('click', async () => {
        await AuthService.logout();
        showView('login');
        showToast('Sesión cerrada', 'info');
    });

    // ═══════════════════════════════════════════════
    // DASHBOARD
    // ═══════════════════════════════════════════════
    async function loadDashboard() {
        // Update greeting
        const user = AuthService.user;
        const hour = new Date().getHours();
        let greeting = 'Buenas noches';
        if (hour >= 6 && hour < 12) greeting = 'Buenos días';
        else if (hour >= 12 && hour < 19) greeting = 'Buenas tardes';

        greetingText.textContent = `${greeting}, ${user?.full_name || user?.username || 'Usuario'}`;
        navUserInitial.textContent = (user?.full_name || user?.username || 'U')[0].toUpperCase();

        // Load apps
        try {
            const apps = await AuthService.getApplications();
            currentApps = Array.isArray(apps) ? apps : (apps?.applications || []);
            renderApps(currentApps);
            statApps.textContent = currentApps.length;
        } catch (error) {
            // Si falla la API, mostrar apps de demostración
            currentApps = getDemoApps();
            renderApps(currentApps);
            statApps.textContent = currentApps.length;
            console.warn('Usando apps de demostración:', error);
        }

        // Load stats
        try {
            const stats = await AuthService.getSystemStats();
            if (stats) {
                statSessions.textContent = stats.active_sessions || 0;
            }
        } catch {
            statSessions.textContent = '0';
        }
    }

    function getDemoApps() {
        return [
            { id: '1', name: 'SAP GUI', app_type: 'sap', category: 'ERP Systems', description: 'Sistema ERP empresarial SAP con interfaz completa', display_protocol: 'VNC' },
            { id: '2', name: 'Microsoft Office', app_type: 'office', category: 'Office Suite', description: 'Word, Excel, PowerPoint, Outlook completos', display_protocol: 'RDP' },
            { id: '3', name: 'AutoCAD', app_type: 'autocad', category: 'Design & CAD', description: 'Diseño CAD profesional 2D y 3D', display_protocol: 'RDP' },
            { id: '4', name: 'LibreOffice', app_type: 'libreoffice', category: 'Office Suite', description: 'Suite ofimática libre: Writer, Calc, Impress', display_protocol: 'VNC' },
            { id: '5', name: 'Windows Desktop', app_type: 'windows-desktop', category: 'Full Desktop', description: 'Escritorio Windows completo con todas las aplicaciones', display_protocol: 'RDP' }
        ];
    }

    function renderApps(apps) {
        const filtered = currentFilter === 'all'
            ? apps
            : apps.filter(app => CATEGORY_MAP[app.category] === currentFilter);

        if (filtered.length === 0) {
            appsGrid.innerHTML = `
                <div class="apps-loading">
                    <p>No hay aplicaciones disponibles en esta categoría</p>
                </div>`;
            return;
        }

        appsGrid.innerHTML = filtered.map(app => `
            <div class="app-card" data-app-id="${app.id}" data-app-type="${app.app_type}" onclick="window._launchApp('${app.app_type}', '${app.name}')">
                <div class="app-card-header">
                    <div class="app-icon ${app.app_type}">
                        ${APP_ICONS[app.app_type] || '📦'}
                    </div>
                    <div>
                        <div class="app-card-title">${app.name}</div>
                        <div class="app-card-category">${app.category}</div>
                    </div>
                </div>
                <div class="app-card-description">${app.description}</div>
                <div class="app-card-footer">
                    <span class="app-protocol">
                        <svg width="10" height="10" viewBox="0 0 24 24" fill="currentColor"><circle cx="12" cy="12" r="10"/></svg>
                        ${app.display_protocol}
                    </span>
                    <span class="app-launch-hint">Iniciar →</span>
                </div>
            </div>
        `).join('');
    }

    // Filter tabs
    filterTabs.forEach(tab => {
        tab.addEventListener('click', () => {
            filterTabs.forEach(t => t.classList.remove('active'));
            tab.classList.add('active');
            currentFilter = tab.dataset.category;
            renderApps(currentApps);
        });
    });

    // ═══════════════════════════════════════════════
    // VIEWER (WebRTC Remote Desktop)
    // ═══════════════════════════════════════════════
    window._launchApp = async function (appType, appName) {
        viewerAppName.textContent = appName;
        viewerOverlay.classList.remove('hidden');
        viewerConnectingStatus.textContent = 'Iniciando contenedor...';
        showView('viewer');

        // Setup WebRTC callbacks
        WebRTCService.onStreamReady((stream) => {
            remoteVideo.srcObject = stream;
            viewerOverlay.classList.add('hidden');
            viewerStatus.textContent = 'Conectado';
            viewerStatus.style.color = 'var(--accent-success)';
        });

        WebRTCService.onStatusChange((state, message) => {
            viewerConnectingStatus.textContent = message;
            viewerStatus.textContent = message;
            if (state === 'error') {
                viewerStatus.style.color = 'var(--accent-danger)';
            }
        });

        WebRTCService.onDisconnected(() => {
            showToast('Conexión perdida con el escritorio remoto', 'error');
            backToDashboard();
        });

        try {
            await WebRTCService.connect(appType);
        } catch (error) {
            showToast(`Error: ${error.message}`, 'error');
            setTimeout(() => backToDashboard(), 2000);
        }
    };

    function backToDashboard() {
        WebRTCService.disconnect();
        remoteVideo.srcObject = null;
        showView('dashboard');
    }

    viewerBackBtn.addEventListener('click', backToDashboard);
    viewerDisconnectBtn.addEventListener('click', backToDashboard);

    viewerFullscreenBtn.addEventListener('click', () => {
        const container = document.querySelector('.viewer-canvas-container');
        if (!document.fullscreenElement) {
            container.requestFullscreen().catch(() => { });
        } else {
            document.exitFullscreen();
        }
    });

    // Mouse/Keyboard input forwarding
    remoteVideo.addEventListener('mousemove', (e) => {
        const rect = remoteVideo.getBoundingClientRect();
        const x = (e.clientX - rect.left) / rect.width;
        const y = (e.clientY - rect.top) / rect.height;
        WebRTCService.sendMouseEvent('move', x, y, 0);
    });

    remoteVideo.addEventListener('mousedown', (e) => {
        const rect = remoteVideo.getBoundingClientRect();
        const x = (e.clientX - rect.left) / rect.width;
        const y = (e.clientY - rect.top) / rect.height;
        WebRTCService.sendMouseEvent('down', x, y, e.button);
    });

    remoteVideo.addEventListener('mouseup', (e) => {
        const rect = remoteVideo.getBoundingClientRect();
        const x = (e.clientX - rect.left) / rect.width;
        const y = (e.clientY - rect.top) / rect.height;
        WebRTCService.sendMouseEvent('up', x, y, e.button);
    });

    document.addEventListener('keydown', (e) => {
        if (views.viewer.classList.contains('active')) {
            WebRTCService.sendKeyEvent('down', e.key, e.code);
            e.preventDefault();
        }
    });

    document.addEventListener('keyup', (e) => {
        if (views.viewer.classList.contains('active')) {
            WebRTCService.sendKeyEvent('up', e.key, e.code);
            e.preventDefault();
        }
    });

    // ═══════════════════════════════════════════════
    // TOAST NOTIFICATIONS
    // ═══════════════════════════════════════════════
    function showToast(message, type = 'info') {
        const container = document.getElementById('toast-container');
        const toast = document.createElement('div');
        toast.className = `toast ${type}`;
        toast.textContent = message;
        container.appendChild(toast);

        setTimeout(() => {
            toast.classList.add('toast-exit');
            setTimeout(() => toast.remove(), 300);
        }, 4000);
    }

    // ═══════════════════════════════════════════════
    // INITIALIZATION
    // ═══════════════════════════════════════════════
    async function init() {
        if (AuthService.isAuthenticated) {
            const isValid = await AuthService.validateToken();
            if (isValid) {
                await loadDashboard();
                showView('dashboard');
                return;
            }
            // Token inválido, limpiar
            await AuthService.logout();
        }
        showView('login');
    }

    init();
})();
