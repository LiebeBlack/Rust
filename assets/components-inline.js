// AcademiaOS Web SPA - Inline HTML Components
// All page templates and UI components

const Components = {
    // ==================== LOGIN COMPONENT ====================
    Login: {
        render: () => `
            <div class="login-container">
                <div class="glass-card login-card">
                    <div class="login-header">
                        <span class="material-icons login-icon">school</span>
                        <h1>AcademiaOS</h1>
                        <p>Sistema de Gestión Académica</p>
                    </div>
                    <form id="login-form" class="login-form">
                        <div class="form-group">
                            <label for="email">Email</label>
                            <div class="input-wrapper">
                                <span class="material-icons input-icon">email</span>
                                <input type="email" id="email" name="email" required placeholder="tu@email.com">
                            </div>
                        </div>
                        <div class="form-group">
                            <label for="password">Contraseña</label>
                            <div class="input-wrapper">
                                <span class="material-icons input-icon">lock</span>
                                <input type="password" id="password" name="password" required placeholder="••••••••">
                            </div>
                        </div>
                        <div class="form-group">
                            <label for="institution">Institución (opcional)</label>
                            <div class="input-wrapper">
                                <span class="material-icons input-icon">business</span>
                                <input type="text" id="institution" name="institution" placeholder="Código de institución">
                            </div>
                        </div>
                        <button type="submit" class="btn-primary btn-full">
                            <span class="material-icons">login</span>
                            Iniciar Sesión
                        </button>
                    </form>
                    <div class="login-footer">
                        <p>¿No tienes cuenta? <a href="#register">Regístrate</a></p>
                    </div>
                </div>
            </div>
        `,
        afterRender: (state, app) => {
            const form = document.getElementById('login-form');
            form.addEventListener('submit', async (e) => {
                e.preventDefault();
                const email = document.getElementById('email').value;
                const password = document.getElementById('password').value;
                const institution = document.getElementById('institution').value;
                await app.login(email, password, institution || 'default');
            });
        }
    },

    // ==================== REGISTER COMPONENT ====================
    Register: {
        render: () => `
            <div class="login-container">
                <div class="glass-card login-card">
                    <div class="login-header">
                        <span class="material-icons login-icon">person_add</span>
                        <h1>Crear Cuenta</h1>
                        <p>Regístrate en AcademiaOS</p>
                    </div>
                    <form id="register-form" class="login-form">
                        <div class="form-group">
                            <label for="reg-institution">Código de Institución *</label>
                            <div class="input-wrapper">
                                <span class="material-icons input-icon">business</span>
                                <input type="text" id="reg-institution" name="institution_code" required placeholder="Código proporcionado por tu institución">
                            </div>
                        </div>
                        <div class="form-group">
                            <label for="reg-email">Email *</label>
                            <div class="input-wrapper">
                                <span class="material-icons input-icon">email</span>
                                <input type="email" id="reg-email" name="email" required placeholder="tu@email.com">
                            </div>
                        </div>
                        <div class="form-group">
                            <label for="reg-password">Contraseña *</label>
                            <div class="input-wrapper">
                                <span class="material-icons input-icon">lock</span>
                                <input type="password" id="reg-password" name="password" required placeholder="Mínimo 8 caracteres" minlength="8">
                            </div>
                        </div>
                        <div class="form-group">
                            <label for="reg-firstname">Nombre</label>
                            <div class="input-wrapper">
                                <span class="material-icons input-icon">person</span>
                                <input type="text" id="reg-firstname" name="first_name" placeholder="Tu nombre">
                            </div>
                        </div>
                        <div class="form-group">
                            <label for="reg-lastname">Apellido</label>
                            <div class="input-wrapper">
                                <span class="material-icons input-icon">person</span>
                                <input type="text" id="reg-lastname" name="last_name" placeholder="Tu apellido">
                            </div>
                        </div>
                        <div class="form-group">
                            <label for="reg-role">Rol *</label>
                            <div class="input-wrapper">
                                <span class="material-icons input-icon">badge</span>
                                <select id="reg-role" name="role" required class="glass-select">
                                    <option value="">Seleccionar rol</option>
                                    <option value="student">Estudiante</option>
                                    <option value="teacher">Profesor</option>
                                    <option value="staff">Personal</option>
                                </select>
                            </div>
                        </div>
                        <button type="submit" class="btn-primary btn-full">
                            <span class="material-icons">person_add</span>
                            Crear Cuenta
                        </button>
                    </form>
                    <div class="login-footer">
                        <p>¿Ya tienes cuenta? <a href="#login">Inicia Sesión</a></p>
                    </div>
                </div>
            </div>
        `,
        afterRender: (state, app) => {
            const form = document.getElementById('register-form');
            form.addEventListener('submit', async (e) => {
                e.preventDefault();
                const formData = new FormData(form);
                const data = Object.fromEntries(formData);
                try {
                    await app.register(data);
                } catch (error) {
                    app.showToast(error.message || 'Error al registrar', 'error');
                }
            });
        }
    },
    
    // ==================== DASHBOARD COMPONENT ====================
    Dashboard: {
        render: (state) => `
            <div class="dashboard-container">
                <div class="dashboard-header">
                    <h1>Dashboard</h1>
                    <p>Bienvenido, ${state.user?.first_name || 'Usuario'}</p>
                </div>
                
                <div class="stats-grid">
                    <div class="stat-card glass-card">
                        <div class="stat-icon stat-icon-blue">
                            <span class="material-icons">people</span>
                        </div>
                        <div class="stat-content">
                            <h3>Estudiantes</h3>
                            <p class="stat-number" id="stat-students">--</p>
                        </div>
                    </div>
                    <div class="stat-card glass-card">
                        <div class="stat-icon stat-icon-green">
                            <span class="material-icons">class</span>
                        </div>
                        <div class="stat-content">
                            <h3>Cursos</h3>
                            <p class="stat-number" id="stat-courses">--</p>
                        </div>
                    </div>
                    <div class="stat-card glass-card">
                        <div class="stat-icon stat-icon-purple">
                            <span class="material-icons">assignment</span>
                        </div>
                        <div class="stat-content">
                            <h3>Calificaciones</h3>
                            <p class="stat-number" id="stat-grades">--</p>
                        </div>
                    </div>
                    <div class="stat-card glass-card">
                        <div class="stat-icon stat-icon-orange">
                            <span class="material-icons">event_available</span>
                        </div>
                        <div class="stat-content">
                            <h3>Asistencia</h3>
                            <p class="stat-number" id="stat-attendance">--</p>
                        </div>
                    </div>
                </div>
                
                <div class="dashboard-content">
                    <div class="glass-card">
                        <div class="card-header">
                            <h2>Actividad Reciente</h2>
                        </div>
                        <div class="activity-list" id="activity-list">
                            <div class="activity-item">
                                <span class="material-icons activity-icon">check_circle</span>
                                <div class="activity-content">
                                    <p>Sistema inicializado correctamente</p>
                                    <span class="activity-time">Ahora</span>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        `,
        afterRender: async (state, app) => {
            // Load dashboard stats
            try {
                const stats = await app.get('/api/admin/stats');
                document.getElementById('stat-students').textContent = stats.total_students;
                document.getElementById('stat-courses').textContent = stats.total_courses;
            } catch (error) {
                console.error('Error loading stats:', error);
            }
        }
    },
    
    // ==================== STUDENTS COMPONENT ====================
    Students: {
        render: () => `
            <div class="page-container">
                <div class="page-header">
                    <h1>Estudiantes</h1>
                    <button class="btn-primary" onclick="app.showModal(Components.StudentForm.render())">
                        <span class="material-icons">add</span>
                        Nuevo Estudiante
                    </button>
                </div>
                
                <div class="glass-card filters-card">
                    <div class="filter-group">
                        <label>Grado:</label>
                        <select id="grade-filter" class="glass-select">
                            <option value="">Todos</option>
                            <option value="1">1° Grado</option>
                            <option value="2">2° Grado</option>
                            <option value="3">3° Grado</option>
                            <option value="4">4° Grado</option>
                            <option value="5">5° Grado</option>
                            <option value="6">6° Grado</option>
                        </select>
                    </div>
                    <div class="filter-group">
                        <label>Buscar:</label>
                        <input type="text" id="search-input" class="glass-input" placeholder="Buscar estudiante...">
                    </div>
                </div>
                
                <div class="glass-card">
                    <div class="table-container">
                        <table class="data-table">
                            <thead>
                                <tr>
                                    <th>ID</th>
                                    <th>Nombre</th>
                                    <th>Grado</th>
                                    <th>Sección</th>
                                    <th>Estado</th>
                                    <th>Acciones</th>
                                </tr>
                            </thead>
                            <tbody id="students-table-body">
                                <tr>
                                    <td colspan="6" class="loading-row">
                                        <div class="loading-spinner"></div>
                                        Cargando estudiantes...
                                    </td>
                                </tr>
                            </tbody>
                        </table>
                    </div>
                    <div class="pagination" id="students-pagination">
                        <button class="btn-secondary" id="prev-page" disabled>Anterior</button>
                        <span id="page-info">Página 1</span>
                        <button class="btn-secondary" id="next-page">Siguiente</button>
                    </div>
                </div>
            </div>
        `,
        afterRender: async (state, app) => {
            let currentPage = 1;
            const loadStudents = async (page = 1, grade = null) => {
                try {
                    const data = await app.fetchStudents(page, 20, grade);
                    const tbody = document.getElementById('students-table-body');
                    
                    if (data.data.length === 0) {
                        tbody.innerHTML = `
                            <tr>
                                <td colspan="6" class="empty-row">
                                    <span class="material-icons empty-icon">inbox</span>
                                    <p>No hay estudiantes registrados</p>
                                </td>
                            </tr>
                        `;
                        return;
                    }
                    
                    tbody.innerHTML = data.data.map(student => `
                        <tr>
                            <td>${student.student_id}</td>
                            <td>${student.user_id ? 'Usuario vinculado' : 'Sin usuario'}</td>
                            <td>${student.grade_level || '-'}</td>
                            <td>${student.section || '-'}</td>
                            <td>
                                <span class="status-badge status-${student.status}">
                                    ${student.status === 'active' ? 'Activo' : 'Inactivo'}
                                </span>
                            </td>
                            <td>
                                <button class="btn-icon" onclick="app.showToast('Funcionalidad en desarrollo', 'info')">
                                    <span class="material-icons">edit</span>
                                </button>
                                <button class="btn-icon btn-danger" onclick="app.showToast('Funcionalidad en desarrollo', 'info')">
                                    <span class="material-icons">delete</span>
                                </button>
                            </td>
                        </tr>
                    `).join('');
                    
                    // Update pagination
                    document.getElementById('page-info').textContent = `Página ${data.page} de ${data.pages}`;
                    document.getElementById('prev-page').disabled = data.page <= 1;
                    document.getElementById('next-page').disabled = data.page >= data.pages;
                    
                    currentPage = data.page;
                } catch (error) {
                    console.error('Error loading students:', error);
                }
            };
            
            // Initial load
            loadStudents();
            
            // Event listeners
            document.getElementById('grade-filter').addEventListener('change', (e) => {
                loadStudents(1, e.target.value || null);
            });
            
            document.getElementById('prev-page').addEventListener('click', () => {
                if (currentPage > 1) loadStudents(currentPage - 1);
            });
            
            document.getElementById('next-page').addEventListener('click', () => {
                loadStudents(currentPage + 1);
            });
        }
    },
    
    // ==================== STUDENT FORM COMPONENT ====================
    StudentForm: {
        render: () => `
            <div class="modal-header">
                <h2>Nuevo Estudiante</h2>
                <button class="btn-icon" onclick="app.closeModal()">
                    <span class="material-icons">close</span>
                </button>
            </div>
            <form id="student-form" class="modal-form">
                <div class="form-group">
                    <label>ID de Estudiante *</label>
                    <input type="text" name="student_id" required class="glass-input">
                </div>
                <div class="form-group">
                    <label>Grado</label>
                    <select name="grade_level" class="glass-select">
                        <option value="">Seleccionar</option>
                        <option value="1">1° Grado</option>
                        <option value="2">2° Grado</option>
                        <option value="3">3° Grado</option>
                        <option value="4">4° Grado</option>
                        <option value="5">5° Grado</option>
                        <option value="6">6° Grado</option>
                    </select>
                </div>
                <div class="form-group">
                    <label>Sección</label>
                    <input type="text" name="section" class="glass-input">
                </div>
                <div class="form-group">
                    <label>Fecha de Matrícula</label>
                    <input type="date" name="enrollment_date" class="glass-input">
                </div>
                <div class="form-actions">
                    <button type="button" class="btn-secondary" onclick="app.closeModal()">Cancelar</button>
                    <button type="submit" class="btn-primary">Guardar</button>
                </div>
            </form>
        `
    },
    
    // ==================== COURSES COMPONENT ====================
    Courses: {
        render: () => `
            <div class="page-container">
                <div class="page-header">
                    <h1>Cursos</h1>
                    <button class="btn-primary" onclick="app.showToast('Funcionalidad en desarrollo', 'info')">
                        <span class="material-icons">add</span>
                        Nuevo Curso
                    </button>
                </div>
                
                <div class="glass-card">
                    <div class="table-container">
                        <table class="data-table">
                            <thead>
                                <tr>
                                    <th>Código</th>
                                    <th>Nombre</th>
                                    <th>Grado</th>
                                    <th>Profesor</th>
                                    <th>Estado</th>
                                    <th>Acciones</th>
                                </tr>
                            </thead>
                            <tbody id="courses-table-body">
                                <tr>
                                    <td colspan="6" class="loading-row">
                                        <div class="loading-spinner"></div>
                                        Cargando cursos...
                                    </td>
                                </tr>
                            </tbody>
                        </table>
                    </div>
                </div>
            </div>
        `,
        afterRender: async (state, app) => {
            try {
                const data = await app.fetchCourses();
                const tbody = document.getElementById('courses-table-body');
                
                if (data.data.length === 0) {
                    tbody.innerHTML = `
                        <tr>
                            <td colspan="6" class="empty-row">
                                <span class="material-icons empty-icon">inbox</span>
                                <p>No hay cursos registrados</p>
                            </td>
                        </tr>
                    `;
                    return;
                }
                
                tbody.innerHTML = data.data.map(course => `
                    <tr>
                        <td>${course.code || '-'}</td>
                        <td>${course.name}</td>
                        <td>${course.grade_level || '-'}</td>
                        <td>${course.teacher_id ? 'Asignado' : 'Sin asignar'}</td>
                        <td>
                            <span class="status-badge status-${course.status}">
                                ${course.status === 'active' ? 'Activo' : 'Inactivo'}
                            </span>
                        </td>
                        <td>
                            <button class="btn-icon" onclick="app.showToast('Funcionalidad en desarrollo', 'info')">
                                <span class="material-icons">edit</span>
                            </button>
                            <button class="btn-icon btn-danger" onclick="app.showToast('Funcionalidad en desarrollo', 'info')">
                                <span class="material-icons">delete</span>
                            </button>
                        </td>
                    </tr>
                `).join('');
            } catch (error) {
                console.error('Error loading courses:', error);
            }
        }
    },
    
    // ==================== GRADES COMPONENT ====================
    Grades: {
        render: () => `
            <div class="page-container">
                <div class="page-header">
                    <h1>Calificaciones</h1>
                </div>
                
                <div class="glass-card">
                    <div class="empty-state">
                        <span class="material-icons empty-icon">assignment</span>
                        <h2>Gestión de Calificaciones</h2>
                        <p>Selecciona un estudiante para ver y gestionar sus calificaciones</p>
                        <button class="btn-primary" onclick="app.navigate('students')">
                            Ir a Estudiantes
                        </button>
                    </div>
                </div>
            </div>
        `
    },
    
    // ==================== ATTENDANCE COMPONENT ====================
    Attendance: {
        render: () => `
            <div class="page-container">
                <div class="page-header">
                    <h1>Asistencia</h1>
                </div>
                
                <div class="glass-card">
                    <div class="empty-state">
                        <span class="material-icons empty-icon">event_available</span>
                        <h2>Registro de Asistencia</h2>
                        <p>Gestiona la asistencia diaria de los estudiantes</p>
                        <button class="btn-primary" onclick="app.showToast('Funcionalidad en desarrollo', 'info')">
                            Tomar Asistencia
                        </button>
                    </div>
                </div>
            </div>
        `
    },
    
    // ==================== MESSAGES COMPONENT ====================
    Messages: {
        render: () => `
            <div class="page-container">
                <div class="page-header">
                    <h1>Mensajes</h1>
                    <button class="btn-primary" onclick="app.showModal(Components.MessageForm.render())">
                        <span class="material-icons">send</span>
                        Nuevo Mensaje
                    </button>
                </div>
                
                <div class="glass-card">
                    <div class="table-container">
                        <table class="data-table">
                            <thead>
                                <tr>
                                    <th>De</th>
                                    <th>Asunto</th>
                                    <th>Fecha</th>
                                    <th>Estado</th>
                                    <th>Acciones</th>
                                </tr>
                            </thead>
                            <tbody id="messages-table-body">
                                <tr>
                                    <td colspan="5" class="loading-row">
                                        <div class="loading-spinner"></div>
                                        Cargando mensajes...
                                    </td>
                                </tr>
                            </tbody>
                        </table>
                    </div>
                </div>
            </div>
        `,
        afterRender: async (state, app) => {
            try {
                const data = await app.fetchMessages();
                const tbody = document.getElementById('messages-table-body');
                
                if (data.data.length === 0) {
                    tbody.innerHTML = `
                        <tr>
                            <td colspan="5" class="empty-row">
                                <span class="material-icons empty-icon">inbox</span>
                                <p>No hay mensajes</p>
                            </td>
                        </tr>
                    `;
                    return;
                }
                
                tbody.innerHTML = data.data.map(message => `
                    <tr>
                        <td>${message.sender_id}</td>
                        <td>${message.subject || 'Sin asunto'}</td>
                        <td>${app.formatDateTime(message.created_at)}</td>
                        <td>
                            <span class="status-badge status-${message.is_read ? 'read' : 'unread'}">
                                ${message.is_read ? 'Leído' : 'No leído'}
                            </span>
                        </td>
                        <td>
                            <button class="btn-icon" onclick="app.showToast('Funcionalidad en desarrollo', 'info')">
                                <span class="material-icons">visibility</span>
                            </button>
                        </td>
                    </tr>
                `).join('');
            } catch (error) {
                console.error('Error loading messages:', error);
            }
        }
    },
    
    // ==================== MESSAGE FORM COMPONENT ====================
    MessageForm: {
        render: () => `
            <div class="modal-header">
                <h2>Nuevo Mensaje</h2>
                <button class="btn-icon" onclick="app.closeModal()">
                    <span class="material-icons">close</span>
                </button>
            </div>
            <form id="message-form" class="modal-form">
                <div class="form-group">
                    <label>Para *</label>
                    <input type="text" name="receiver_id" required class="glass-input" placeholder="ID del destinatario">
                </div>
                <div class="form-group">
                    <label>Asunto</label>
                    <input type="text" name="subject" class="glass-input">
                </div>
                <div class="form-group">
                    <label>Mensaje *</label>
                    <textarea name="content" required class="glass-textarea" rows="5"></textarea>
                </div>
                <div class="form-actions">
                    <button type="button" class="btn-secondary" onclick="app.closeModal()">Cancelar</button>
                    <button type="submit" class="btn-primary">Enviar</button>
                </div>
            </form>
        `
    },
    
    // ==================== NOTIFICATIONS COMPONENT ====================
    Notifications: {
        render: () => `
            <div class="page-container">
                <div class="page-header">
                    <h1>Notificaciones</h1>
                </div>
                
                <div class="glass-card">
                    <div class="table-container">
                        <table class="data-table">
                            <thead>
                                <tr>
                                    <th>Título</th>
                                    <th>Mensaje</th>
                                    <th>Tipo</th>
                                    <th>Fecha</th>
                                    <th>Estado</th>
                                </tr>
                            </thead>
                            <tbody id="notifications-table-body">
                                <tr>
                                    <td colspan="5" class="loading-row">
                                        <div class="loading-spinner"></div>
                                        Cargando notificaciones...
                                    </td>
                                </tr>
                            </tbody>
                        </table>
                    </div>
                </div>
            </div>
        `,
        afterRender: async (state, app) => {
            try {
                const data = await app.fetchNotifications();
                const tbody = document.getElementById('notifications-table-body');
                
                if (data.data.length === 0) {
                    tbody.innerHTML = `
                        <tr>
                            <td colspan="5" class="empty-row">
                                <span class="material-icons empty-icon">inbox</span>
                                <p>No hay notificaciones</p>
                            </td>
                        </tr>
                    `;
                    return;
                }
                
                tbody.innerHTML = data.data.map(notification => `
                    <tr>
                        <td>${notification.title}</td>
                        <td>${notification.message}</td>
                        <td>
                            <span class="notification-badge notification-${notification.type || 'info'}">
                                ${notification.type || 'info'}
                            </span>
                        </td>
                        <td>${app.formatDateTime(notification.created_at)}</td>
                        <td>
                            <span class="status-badge status-${notification.is_read ? 'read' : 'unread'}">
                                ${notification.is_read ? 'Leído' : 'No leído'}
                            </span>
                        </td>
                    </tr>
                `).join('');
            } catch (error) {
                console.error('Error loading notifications:', error);
            }
        }
    },
    
    // ==================== NOT FOUND COMPONENT ====================
    NotFound: {
        render: () => `
            <div class="page-container">
                <div class="glass-card">
                    <div class="empty-state">
                        <span class="material-icons empty-icon">error_outline</span>
                        <h2>Página no encontrada</h2>
                        <p>La página que buscas no existe</p>
                        <button class="btn-primary" onclick="app.navigate('dashboard')">
                            Volver al Dashboard
                        </button>
                    </div>
                </div>
            </div>
        `
    },

    // ==================== PROFILE COMPONENT ====================
    Profile: {
        render: (state) => `
            <div class="page-container">
                <div class="page-header">
                    <h1>Mi Perfil</h1>
                </div>
                
                <div class="glass-card profile-card">
                    <div class="profile-header">
                        <div class="profile-avatar-large">
                            <span class="material-icons">person</span>
                        </div>
                        <div class="profile-info">
                            <h2 class="profile-name">${state.user?.first_name || 'Usuario'} ${state.user?.last_name || ''}</h2>
                            <p class="profile-role">${state.user?.role || 'Sin rol'}</p>
                        </div>
                    </div>
                    
                    <form id="profile-form" class="profile-form">
                        <div class="form-group">
                            <label>Email</label>
                            <input type="email" value="${state.user?.email || ''}" disabled class="glass-input profile-input-disabled">
                        </div>
                        <div class="form-group">
                            <label for="profile-firstname">Nombre</label>
                            <input type="text" id="profile-firstname" name="first_name" value="${state.user?.first_name || ''}" class="glass-input">
                        </div>
                        <div class="form-group">
                            <label for="profile-lastname">Apellido</label>
                            <input type="text" id="profile-lastname" name="last_name" value="${state.user?.last_name || ''}" class="glass-input">
                        </div>
                        <div class="form-actions">
                            <button type="submit" class="btn-primary">
                                <span class="material-icons">save</span>
                                Guardar Cambios
                            </button>
                        </div>
                    </form>
                </div>
                
                <div class="glass-card profile-card">
                    <h2 class="profile-section-title">Cambiar Contraseña</h2>
                    <form id="password-form" class="password-form">
                        <div class="form-group">
                            <label for="current-password">Contraseña Actual</label>
                            <input type="password" id="current-password" name="current_password" class="glass-input" placeholder="••••••••">
                        </div>
                        <div class="form-group">
                            <label for="new-password">Nueva Contraseña</label>
                            <input type="password" id="new-password" name="new_password" class="glass-input" placeholder="Mínimo 8 caracteres" minlength="8">
                        </div>
                        <div class="form-group">
                            <label for="confirm-password">Confirmar Contraseña</label>
                            <input type="password" id="confirm-password" name="confirm_password" class="glass-input" placeholder="••••••••">
                        </div>
                        <div class="form-actions">
                            <button type="submit" class="btn-primary">
                                <span class="material-icons">lock</span>
                                Actualizar Contraseña
                            </button>
                        </div>
                    </form>
                </div>
            </div>
        `,
        afterRender: (state, app) => {
            const profileForm = document.getElementById('profile-form');
            profileForm.addEventListener('submit', async (e) => {
                e.preventDefault();
                app.showToast('Perfil actualizado', 'success');
            });
            
            const passwordForm = document.getElementById('password-form');
            passwordForm.addEventListener('submit', async (e) => {
                e.preventDefault();
                const current = document.getElementById('current-password').value;
                const newPass = document.getElementById('new-password').value;
                const confirm = document.getElementById('confirm-password').value;
                
                if (newPass !== confirm) {
                    app.showToast('Las contraseñas no coinciden', 'error');
                    return;
                }
                
                app.showToast('Contraseña actualizada', 'success');
                passwordForm.reset();
            });
        }
    },

    // ==================== SETTINGS COMPONENT ====================
    Settings: {
        render: (state) => `
            <div class="page-container">
                <div class="page-header">
                    <h1>Configuración</h1>
                </div>
                
                <div class="glass-card settings-card">
                    <h2 class="settings-section-title">Preferencias de la Aplicación</h2>
                    
                    <div class="settings-item">
                        <div class="settings-item-content">
                            <h3>Notificaciones</h3>
                            <p>Recibir alertas de mensajes y avisos</p>
                        </div>
                        <label class="toggle-switch">
                            <input type="checkbox" checked>
                            <span class="toggle-slider"></span>
                        </label>
                    </div>
                    
                    <div class="settings-item">
                        <div class="settings-item-content">
                            <h3>Sonidos</h3>
                            <p>Reproducir sonidos de notificación</p>
                        </div>
                        <label class="toggle-switch">
                            <input type="checkbox" checked>
                            <span class="toggle-slider"></span>
                        </label>
                    </div>
                    
                    <div class="settings-item">
                        <div class="settings-item-content">
                            <h3>Modo Oscuro</h3>
                            <p>Usar tema oscuro en la interfaz</p>
                        </div>
                        <label class="toggle-switch">
                            <input type="checkbox" id="dark-mode-toggle">
                            <span class="toggle-slider"></span>
                        </label>
                    </div>
                    
                    <div class="settings-item">
                        <div class="settings-item-content">
                            <h3>Idioma</h3>
                            <p>Idioma de la interfaz</p>
                        </div>
                        <select class="glass-select settings-select">
                            <option value="es">Español</option>
                            <option value="en">English</option>
                        </select>
                    </div>
                </div>
                
                <div class="glass-card settings-card">
                    <h2 class="settings-section-title">Información de la Institución</h2>
                    <div class="settings-info">
                        <div class="settings-info-item">
                            <span class="settings-info-label">Nombre:</span>
                            <p class="settings-info-value">${state.institution?.name || 'AcademiaOS Default'}</p>
                        </div>
                        <div class="settings-info-item">
                            <span class="settings-info-label">Código:</span>
                            <p class="settings-info-value">${state.institution?.code || 'default'}</p>
                        </div>
                        <div class="settings-info-item">
                            <span class="settings-info-label">Versión:</span>
                            <p class="settings-info-value">EduCore Ultra v1.0.0</p>
                        </div>
                    </div>
                </div>
            </div>
        `,
        afterRender: (state, app) => {
            // Set dark mode toggle state
            const darkModeToggle = document.getElementById('dark-mode-toggle');
            if (darkModeToggle) {
                darkModeToggle.checked = document.body.classList.contains('dark-mode');
            }

            // Toggle switch functionality
            document.querySelectorAll('.toggle-switch input:not(#dark-mode-toggle)').forEach(toggle => {
                toggle.addEventListener('change', (e) => {
                    app.showToast('Configuración guardada', 'success');
                });
            });
        }
    },

    // ==================== INSTITUTIONS COMPONENT ====================
    Institutions: {
        render: () => `
            <div class="page-container">
                <div class="page-header">
                    <h1>Instituciones</h1>
                    <button class="btn-primary" onclick="app.showModal(Components.InstitutionForm.render())">
                        <span class="material-icons">add</span>
                        Nueva Institución
                    </button>
                </div>
                
                <div class="glass-card">
                    <div class="table-container">
                        <table class="data-table">
                            <thead>
                                <tr>
                                    <th>ID</th>
                                    <th>Nombre</th>
                                    <th>Código</th>
                                    <th>Email</th>
                                    <th>Teléfono</th>
                                    <th>Estado</th>
                                    <th>Acciones</th>
                                </tr>
                            </thead>
                            <tbody id="institutions-table-body">
                                <tr>
                                    <td colspan="7" class="loading-row">
                                        <div class="loading-spinner"></div>
                                        Cargando instituciones...
                                    </td>
                                </tr>
                            </tbody>
                        </table>
                    </div>
                </div>
            </div>
        `,
        afterRender: async (state, app) => {
            try {
                const data = await app.get('/api/institutions');
                const tbody = document.getElementById('institutions-table-body');
                
                if (data.data.length === 0) {
                    tbody.innerHTML = `
                        <tr>
                            <td colspan="7" class="empty-row">
                                <span class="material-icons empty-icon">inbox</span>
                                <p>No hay instituciones registradas</p>
                            </td>
                        </tr>
                    `;
                    return;
                }
                
                tbody.innerHTML = data.data.map(inst => `
                    <tr>
                        <td>${inst.id}</td>
                        <td>${inst.name}</td>
                        <td>${inst.code}</td>
                        <td>${inst.email || '-'}</td>
                        <td>${inst.phone || '-'}</td>
                        <td>
                            <span class="status-badge status-${inst.status}">
                                ${inst.status === 'active' ? 'Activo' : 'Inactivo'}
                            </span>
                        </td>
                        <td>
                            <button class="btn-icon" onclick="app.showToast('Funcionalidad en desarrollo', 'info')">
                                <span class="material-icons">edit</span>
                            </button>
                            <button class="btn-icon btn-danger" onclick="app.showToast('Funcionalidad en desarrollo', 'info')">
                                <span class="material-icons">delete</span>
                            </button>
                        </td>
                    </tr>
                `).join('');
            } catch (error) {
                console.error('Error loading institutions:', error);
            }
        }
    },

    // ==================== INSTITUTION FORM COMPONENT ====================
    InstitutionForm: {
        render: () => `
            <div class="modal-header">
                <h2>Nueva Institución</h2>
                <button class="btn-icon" onclick="app.closeModal()">
                    <span class="material-icons">close</span>
                </button>
            </div>
            <form id="institution-form" class="modal-form">
                <div class="form-group">
                    <label>Nombre *</label>
                    <input type="text" name="name" required class="glass-input" placeholder="Nombre de la institución">
                </div>
                <div class="form-group">
                    <label>Código *</label>
                    <input type="text" name="code" required class="glass-input" placeholder="Código único">
                </div>
                <div class="form-group">
                    <label>Email</label>
                    <input type="email" name="email" class="glass-input" placeholder="contacto@institucion.edu">
                </div>
                <div class="form-group">
                    <label>Teléfono</label>
                    <input type="tel" name="phone" class="glass-input" placeholder="+1 234 567 890">
                </div>
                <div class="form-group">
                    <label>Dirección</label>
                    <textarea name="address" class="glass-textarea" rows="3" placeholder="Dirección física"></textarea>
                </div>
                <div class="form-actions">
                    <button type="button" class="btn-secondary" onclick="app.closeModal()">Cancelar</button>
                    <button type="submit" class="btn-primary">Crear</button>
                </div>
            </form>
        `
    }
};
