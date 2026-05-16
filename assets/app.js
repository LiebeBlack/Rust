// AcademiaOS Web SPA - Main Application Logic
// Router, State Management, API Client, and Fallbacks

class AcademiaApp {
    constructor() {
        this.state = {
            user: null,
            token: null,
            institution: null,
            currentPage: 'dashboard',
            isLoading: false,
            error: null
        };
        
        this.apiBase = window.location.origin;
        this.routes = {};
        this.components = {};
        
        this.init();
    }
    
    async init() {
        try {
            // Load saved session
            this.loadSession();
            
            // Load dark mode preference
            this.loadDarkModePreference();
            
            // Initialize router
            this.initRouter();
            
            // Register components
            this.registerComponents();
            
            // Check authentication
            if (this.state.token) {
                await this.fetchCurrentUser();
            } else {
                // Check if trying to access register page
                const hash = window.location.hash.slice(1);
                if (hash === 'register') {
                    this.showRegister();
                } else {
                    this.showLogin();
                }
            }
            
            // Hide loading screen
            this.hideLoading();
            
        } catch (error) {
            console.error('Initialization error:', error);
            this.showError('No se pudo inicializar la aplicación');
        }
    }
    
    // ==================== ROUTER ====================
    
    initRouter() {
        window.addEventListener('hashchange', () => this.handleRouteChange());
        this.handleRouteChange();
    }
    
    handleRouteChange() {
        const hash = window.location.hash.slice(1) || 'dashboard';
        
        // Handle auth pages
        if (hash === 'login' || hash === 'register') {
            if (this.state.token) {
                // Already logged in, redirect to dashboard
                this.navigate('dashboard');
                return;
            }
            if (hash === 'login') {
                this.showLogin();
            } else {
                this.showRegister();
            }
            return;
        }
        
        // Handle authenticated pages
        if (!this.state.token) {
            this.showLogin();
            return;
        }
        
        this.state.currentPage = hash;
        this.renderPage(hash);
        this.updateActiveNav(hash);
    }
    
    navigate(page) {
        window.location.hash = page;
    }
    
    updateActiveNav(page) {
        document.querySelectorAll('.nav-item').forEach(link => {
            link.classList.remove('active');
            if (link.getAttribute('data-page') === page) {
                link.classList.add('active');
            }
        });
    }
    
    // ==================== COMPONENTS ====================
    
    registerComponents() {
        this.components = {
            login: Components.Login,
            register: Components.Register,
            dashboard: Components.Dashboard,
            students: Components.Students,
            courses: Components.Courses,
            grades: Components.Grades,
            attendance: Components.Attendance,
            messages: Components.Messages,
            notifications: Components.Notifications,
            profile: Components.Profile,
            settings: Components.Settings,
            institutions: Components.Institutions
        };
    }
    
    renderPage(page) {
        const content = document.getElementById('page-content');
        const component = this.components[page];
        
        if (!content) {
            console.error('Page content element not found');
            return;
        }
        
        if (component) {
            try {
                content.innerHTML = component.render(this.state);
                if (component.afterRender) {
                    component.afterRender(this.state, this);
                }
                this.updateUserInfo();
            } catch (error) {
                console.error('Error rendering page:', error);
                content.innerHTML = Components.NotFound.render();
            }
        } else {
            content.innerHTML = Components.NotFound.render();
        }
    }
    
    // ==================== API CLIENT ====================
    
    async apiCall(endpoint, options = {}) {
        const url = `${this.apiBase}${endpoint}`;
        const headers = {
            'Content-Type': 'application/json',
            ...options.headers
        };
        
        if (this.state.token) {
            headers['Authorization'] = `Bearer ${this.state.token}`;
        }
        
        try {
            const response = await fetch(url, {
                ...options,
                headers,
                credentials: 'include'
            });
            
            if (!response.ok) {
                if (response.status === 401) {
                    // Unauthorized - clear session and redirect to login
                    this.clearSession();
                    this.state.user = null;
                    this.state.token = null;
                    this.state.institution = null;
                    this.showLogin();
                    throw new Error('Sesión expirada. Por favor inicia sesión nuevamente.');
                }
                
                let errorMessage = 'Error en la petición';
                try {
                    const error = await response.json();
                    errorMessage = error.message || error.detail || errorMessage;
                } catch (e) {
                    errorMessage = response.statusText || errorMessage;
                }
                throw new Error(errorMessage);
            }
            
            const contentType = response.headers.get('content-type');
            if (contentType && contentType.includes('application/json')) {
                return await response.json();
            }
            return await response.text();
        } catch (error) {
            console.error('API Error:', error);
            throw error;
        }
    }
    
    async get(endpoint) {
        return this.apiCall(endpoint, { method: 'GET' });
    }
    
    async post(endpoint, data) {
        return this.apiCall(endpoint, {
            method: 'POST',
            body: JSON.stringify(data)
        });
    }
    
    async put(endpoint, data) {
        return this.apiCall(endpoint, {
            method: 'PUT',
            body: JSON.stringify(data)
        });
    }
    
    async delete(endpoint) {
        return this.apiCall(endpoint, { method: 'DELETE' });
    }
    
    // ==================== AUTHENTICATION ====================
    
    async login(email, password, institutionCode = 'default') {
        try {
            this.setLoading(true);
            
            const response = await this.post('/api/auth/login', {
                email,
                password,
                institution_code: institutionCode
            });
            
            this.state.token = response.token;
            this.state.user = response.user;
            this.state.institution = response.institution;
            
            this.saveSession();
            this.updateUserInfo();
            this.showMainApp();
            this.navigate('dashboard');
            this.showToast('Inicio de sesión exitoso', 'success');
            
        } catch (error) {
            this.showToast(error.message || 'Error al iniciar sesión', 'error');
        } finally {
            this.setLoading(false);
        }
    }

    async register(data) {
        try {
            this.setLoading(true);
            
            const response = await this.post('/api/auth/register', data);
            
            this.showToast('Cuenta creada exitosamente', 'success');
            this.navigate('login');
            
        } catch (error) {
            this.showToast(error.message || 'Error al registrar', 'error');
            throw error;
        } finally {
            this.setLoading(false);
        }
    }
    
    async logout() {
        try {
            await this.post('/api/auth/logout');
        } catch (error) {
            console.error('Logout error:', error);
        }
        
        this.clearSession();
        this.state.user = null;
        this.state.token = null;
        this.state.institution = null;
        this.showLogin();
        this.showToast('Sesión cerrada', 'info');
    }
    
    async fetchCurrentUser() {
        try {
            const user = await this.get('/api/auth/me');
            if (user) {
                this.state.user = user;
                this.updateUserInfo();
            } else {
                throw new Error('No user data received');
            }
        } catch (error) {
            console.error('Error fetching user:', error);
            this.clearSession();
            this.showLogin();
        }
    }
    
    // ==================== SESSION MANAGEMENT ====================
    
    saveSession() {
        try {
            localStorage.setItem('academia_token', this.state.token);
            localStorage.setItem('academia_user', JSON.stringify(this.state.user));
            localStorage.setItem('academia_institution', JSON.stringify(this.state.institution));
            localStorage.setItem('academia_timestamp', Date.now().toString());
        } catch (error) {
            console.error('Error saving session:', error);
        }
    }
    
    loadSession() {
        try {
            const token = localStorage.getItem('academia_token');
            const user = localStorage.getItem('academia_user');
            const institution = localStorage.getItem('academia_institution');
            const timestamp = localStorage.getItem('academia_timestamp');
            
            // Check if session is older than 24 hours
            if (timestamp) {
                const sessionAge = Date.now() - parseInt(timestamp);
                const maxAge = 24 * 60 * 60 * 1000; // 24 hours
                if (sessionAge > maxAge) {
                    this.clearSession();
                    return;
                }
            }
            
            if (token && user) {
                this.state.token = token;
                this.state.user = JSON.parse(user);
                this.state.institution = institution ? JSON.parse(institution) : null;
            }
        } catch (error) {
            console.error('Error loading session:', error);
            this.clearSession();
        }
    }
    
    clearSession() {
        try {
            localStorage.removeItem('academia_token');
            localStorage.removeItem('academia_user');
            localStorage.removeItem('academia_institution');
            localStorage.removeItem('academia_timestamp');
        } catch (error) {
            console.error('Error clearing session:', error);
        }
    }
    
    // ==================== UI HELPERS ====================
    
    showLogin() {
        const authContainer = document.getElementById('auth-container');
        const mainApp = document.getElementById('main-app');
        const content = document.getElementById('auth-content');
        
        if (!authContainer || !mainApp || !content) {
            console.error('Required DOM elements not found');
            return;
        }
        
        authContainer.classList.remove('hidden');
        mainApp.classList.add('hidden');
        content.innerHTML = Components.Login.render();
        Components.Login.afterRender(this.state, this);
    }

    showRegister() {
        const authContainer = document.getElementById('auth-container');
        const mainApp = document.getElementById('main-app');
        const content = document.getElementById('auth-content');
        
        if (!authContainer || !mainApp || !content) {
            console.error('Required DOM elements not found');
            return;
        }
        
        authContainer.classList.remove('hidden');
        mainApp.classList.add('hidden');
        content.innerHTML = Components.Register.render();
        Components.Register.afterRender(this.state, this);
    }

    showMainApp() {
        const authContainer = document.getElementById('auth-container');
        const mainApp = document.getElementById('main-app');
        
        if (!authContainer || !mainApp) {
            console.error('Required DOM elements not found');
            return;
        }
        
        authContainer.classList.add('hidden');
        mainApp.classList.remove('hidden');
        this.initSidebar();
        this.initTopbar();
    }

    initSidebar() {
        const hamburgerBtn = document.getElementById('hamburger-btn');
        const sidebar = document.getElementById('sidebar');
        const sidebarOverlay = document.getElementById('sidebar-overlay');
        const sidebarCloseBtn = document.getElementById('sidebar-close-btn');

        if (hamburgerBtn) {
            hamburgerBtn.addEventListener('click', () => {
                sidebar.classList.add('open');
                sidebarOverlay.classList.add('active');
            });
        }

        if (sidebarCloseBtn) {
            sidebarCloseBtn.addEventListener('click', () => {
                sidebar.classList.remove('open');
                sidebarOverlay.classList.remove('active');
            });
        }

        if (sidebarOverlay) {
            sidebarOverlay.addEventListener('click', () => {
                sidebar.classList.remove('open');
                sidebarOverlay.classList.remove('active');
            });
        }

        // Admin section visibility
        const adminSection = document.getElementById('admin-nav-section');
        if (adminSection && this.state.user) {
            adminSection.style.display = this.state.user.role === 'admin' ? 'block' : 'none';
        }
    }

    initTopbar() {
        const userMenuBtn = document.getElementById('topbar-user-btn');
        const userDropdown = document.getElementById('user-dropdown');

        if (userMenuBtn && userDropdown) {
            userMenuBtn.addEventListener('click', (e) => {
                e.stopPropagation();
                userDropdown.classList.toggle('show');
            });

            document.addEventListener('click', () => {
                userDropdown.classList.remove('show');
            });
        }

        // Dark mode toggle
        const darkModeToggle = document.getElementById('dark-mode-toggle');
        if (darkModeToggle) {
            darkModeToggle.addEventListener('change', (e) => {
                this.toggleDarkMode(e.target.checked);
            });
        }

        // Update user info in topbar and sidebar
        this.updateUserInfo();
    }

    toggleDarkMode(enabled) {
        if (enabled) {
            document.body.classList.add('dark-mode');
            localStorage.setItem('darkMode', 'true');
        } else {
            document.body.classList.remove('dark-mode');
            localStorage.setItem('darkMode', 'false');
        }
        this.showToast(enabled ? 'Modo oscuro activado' : 'Modo claro activado', 'info');
    }

    loadDarkModePreference() {
        const savedPreference = localStorage.getItem('darkMode');
        if (savedPreference === 'true') {
            document.body.classList.add('dark-mode');
            const toggle = document.getElementById('dark-mode-toggle');
            if (toggle) toggle.checked = true;
        }
    }
    
    updateUserInfo() {
        if (!this.state.user) return;

        const fullName = `${this.state.user.first_name || ''} ${this.state.user.last_name || ''}`.trim();
        
        // Sidebar user info
        const sidebarUserName = document.getElementById('sidebar-user-name');
        const sidebarUserRole = document.getElementById('sidebar-user-role');
        if (sidebarUserName) sidebarUserName.textContent = fullName || 'Usuario';
        if (sidebarUserRole) sidebarUserRole.textContent = this.state.user.role || '—';

        // Topbar user info
        const topbarUserName = document.getElementById('topbar-user-name');
        if (topbarUserName) topbarUserName.textContent = fullName || 'Usuario';

        // Update page title
        const topbarTitle = document.getElementById('topbar-title');
        if (topbarTitle) {
            const pageNames = {
                dashboard: 'Dashboard',
                students: 'Estudiantes',
                courses: 'Cursos',
                grades: 'Calificaciones',
                attendance: 'Asistencia',
                messages: 'Mensajes',
                notifications: 'Avisos',
                profile: 'Mi Perfil',
                settings: 'Configuración',
                institutions: 'Instituciones'
            };
            topbarTitle.textContent = pageNames[this.state.currentPage] || 'Dashboard';
        }
    }
    
    setLoading(loading) {
        this.state.isLoading = loading;
        const loadingScreen = document.getElementById('loading-screen');
        if (loadingScreen) {
            if (loading) {
                loadingScreen.classList.remove('hidden');
            } else {
                loadingScreen.classList.add('hidden');
            }
        }
    }
    
    hideLoading() {
        const loadingScreen = document.getElementById('loading-screen');
        const mainApp = document.getElementById('main-app');
        
        if (loadingScreen) loadingScreen.classList.add('hidden');
        if (mainApp) {
            if (this.state.token) {
                mainApp.classList.remove('hidden');
            }
        }
    }
    
    showError(message) {
        document.getElementById('error-message').textContent = message;
        document.getElementById('error-screen').classList.remove('hidden');
        document.getElementById('main-app').classList.add('hidden');
    }
    
    hideError() {
        document.getElementById('error-screen').classList.add('hidden');
        document.getElementById('main-app').classList.remove('hidden');
    }
    
    retry() {
        this.hideError();
        this.init();
    }
    
    showToast(message, type = 'info') {
        const container = document.getElementById('toast-container');
        const toast = document.createElement('div');
        toast.className = `toast toast-${type}`;
        toast.innerHTML = `
            <span class="material-icons">${this.getToastIcon(type)}</span>
            <span>${message}</span>
        `;
        container.appendChild(toast);
        
        setTimeout(() => {
            toast.classList.add('fade-out');
            setTimeout(() => toast.remove(), 300);
        }, 3000);
    }
    
    getToastIcon(type) {
        const icons = {
            success: 'check_circle',
            error: 'error',
            warning: 'warning',
            info: 'info'
        };
        return icons[type] || 'info';
    }
    
    showModal(content) {
        const container = document.getElementById('modal-container');
        container.innerHTML = `
            <div class="modal-overlay" onclick="app.closeModal()">
                <div class="modal-content" onclick="event.stopPropagation()">
                    ${content}
                </div>
            </div>
        `;
        container.classList.remove('hidden');
    }
    
    closeModal() {
        const container = document.getElementById('modal-container');
        container.classList.add('hidden');
        container.innerHTML = '';
    }
    
    // ==================== DATA FETCHING ====================
    
    async fetchStudents(page = 1, limit = 20, grade = null) {
        try {
            const params = new URLSearchParams({ page: page.toString(), limit: limit.toString() });
            if (grade) params.append('grade', grade.toString());
            const response = await this.get(`/api/students?${params}`);
            return response || { data: [], page: 1, pages: 1, total: 0 };
        } catch (error) {
            this.showToast('Error al cargar estudiantes', 'error');
            return { data: [], page: 1, pages: 1, total: 0 };
        }
    }
    
    async fetchCourses(page = 1, limit = 20) {
        try {
            const params = new URLSearchParams({ page: page.toString(), limit: limit.toString() });
            const response = await this.get(`/api/courses?${params}`);
            return response || { data: [], page: 1, pages: 1, total: 0 };
        } catch (error) {
            this.showToast('Error al cargar cursos', 'error');
            return { data: [], page: 1, pages: 1, total: 0 };
        }
    }
    
    async fetchMessages(page = 1, limit = 20) {
        try {
            const params = new URLSearchParams({ page: page.toString(), limit: limit.toString() });
            const response = await this.get(`/api/messages?${params}`);
            return response || { data: [], page: 1, pages: 1, total: 0 };
        } catch (error) {
            this.showToast('Error al cargar mensajes', 'error');
            return { data: [], page: 1, pages: 1, total: 0 };
        }
    }
    
    async fetchNotifications(page = 1, limit = 20) {
        try {
            const params = new URLSearchParams({ page: page.toString(), limit: limit.toString() });
            const response = await this.get(`/api/notifications?${params}`);
            return response || { data: [], page: 1, pages: 1, total: 0 };
        } catch (error) {
            this.showToast('Error al cargar notificaciones', 'error');
            return { data: [], page: 1, pages: 1, total: 0 };
        }
    }
    
    // ==================== OFFLINE FALLBACK ====================
    
    isOnline() {
        return navigator.onLine;
    }
    
    setupOfflineListeners() {
        window.addEventListener('online', () => {
            this.showToast('Conexión restaurada', 'success');
        });
        
        window.addEventListener('offline', () => {
            this.showToast('Sin conexión - Modo offline', 'warning');
        });
    }
    
    // ==================== FORMATTING ====================
    
    formatDate(date) {
        return new Date(date).toLocaleDateString('es-ES', {
            year: 'numeric',
            month: 'long',
            day: 'numeric'
        });
    }
    
    formatDateTime(date) {
        return new Date(date).toLocaleString('es-ES', {
            year: 'numeric',
            month: 'long',
            day: 'numeric',
            hour: '2-digit',
            minute: '2-digit'
        });
    }
}

// Initialize app
const app = new AcademiaApp();
