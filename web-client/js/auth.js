/**
 * ERP Virtualization - Auth Service
 * Maneja la autenticación contra Railway API
 */
const AuthService = (() => {
    // URL de la API de Railway — cambiar en producción
    const API_URL = 'https://erp-api-production-6448.up.railway.app';

    let _token = localStorage.getItem('erp_token') || null;
    let _user = JSON.parse(localStorage.getItem('erp_user') || 'null');

    /**
     * Iniciar sesión
     */
    async function login(username, password) {
        const response = await fetch(`${API_URL}/api/auth/login`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ username, password })
        });

        if (!response.ok) {
            const error = await response.json().catch(() => ({}));
            throw new Error(error.message || 'Credenciales incorrectas');
        }

        const data = await response.json();
        _token = data.token;
        _user = data.user || { username, full_name: username };

        localStorage.setItem('erp_token', _token);
        localStorage.setItem('erp_user', JSON.stringify(_user));

        return { token: _token, user: _user };
    }

    /**
     * Cerrar sesión
     */
    async function logout() {
        try {
            if (_token) {
                await fetch(`${API_URL}/api/auth/logout`, {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                        'Authorization': `Bearer ${_token}`
                    },
                    body: JSON.stringify({ token: _token })
                });
            }
        } catch (e) {
            console.warn('Error al cerrar sesión en servidor:', e);
        } finally {
            _token = null;
            _user = null;
            localStorage.removeItem('erp_token');
            localStorage.removeItem('erp_user');
        }
    }

    /**
     * Validar token existente
     */
    async function validateToken() {
        if (!_token) return false;

        try {
            const response = await fetch(`${API_URL}/api/auth/validate`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ token: _token })
            });
            return response.ok;
        } catch {
            return false;
        }
    }

    /**
     * Obtener lista de aplicaciones disponibles
     */
    async function getApplications() {
        const response = await fetch(`${API_URL}/api/applications`, {
            headers: { 'Authorization': `Bearer ${_token}` }
        });

        if (!response.ok) throw new Error('Error al cargar aplicaciones');
        return response.json();
    }

    /**
     * Obtener estadísticas del sistema
     */
    async function getSystemStats() {
        try {
            const response = await fetch(`${API_URL}/api/system/stats`, {
                headers: { 'Authorization': `Bearer ${_token}` }
            });
            if (!response.ok) return null;
            return response.json();
        } catch {
            return null;
        }
    }

    /**
     * Hacer request autenticado genérico
     */
    async function authFetch(path, options = {}) {
        return fetch(`${API_URL}${path}`, {
            ...options,
            headers: {
                'Content-Type': 'application/json',
                'Authorization': `Bearer ${_token}`,
                ...options.headers
            }
        });
    }

    /**
     * Configurar URL de la API
     */
    function setApiUrl(url) {
        localStorage.setItem('erp_api_url', url);
    }

    return {
        login,
        logout,
        validateToken,
        getApplications,
        getSystemStats,
        authFetch,
        setApiUrl,
        get token() { return _token; },
        get user() { return _user; },
        get isAuthenticated() { return !!_token; },
        get apiUrl() { return API_URL; }
    };
})();
