# EduCore Ultra

Sistema de gestión académica completo con backend en Rust y frontend web SPA, implementado con Clean Architecture.

## 🚀 Características

### Backend (Rust)
- **Clean Architecture**: Separación clara de capas (Domain, Data, Service, Presentation)
- **Multi-Tenant**: Soporte para múltiples instituciones con aislamiento de datos
- **JWT Authentication**: Autenticación segura con tokens JWT multi-tenant
- **SQLite WAL Mode**: Base de datos ligera con modo WAL para alto rendimiento
- **Cluster Mode**: Soporte para despliegue multi-servidor con coordinación automática
- **File Management**: Upload/download con deduplicación SHA256 y compresión de imágenes
- **REST API**: API RESTful completa con documentación
- **Production Ready**: Manejo de errores global, validación, logging, y más

### Frontend Web (SPA)
- **Vanilla JavaScript**: Sin frameworks, ES6+ moderno
- **Glassmorphism UI**: Diseño moderno con efectos de cristal
- **Single Page Application**: Navegación fluida sin recargas
- **PWA Support**: Service Worker para modo offline
- **Responsive Design**: Optimizado para móvil y desktop
- **Component-Based**: Arquitectura de componentes reutilizables
- **Error Handling**: Fallbacks y manejo de errores robusto

## 📋 Requisitos

- Rust 1.75 o superior
- SQLite 3
- (Opcional) Docker para despliegue en contenedores

## 🌐 Frontend Web

El frontend web es una SPA (Single Page Application) servida directamente por el backend Rust.

### Características del Frontend

- **Glassmorphism UI**: Diseño moderno con efectos de cristal y gradientes
- **SPA Router**: Navegación sin recargas usando hash-based routing
- **Componentes Modulares**: Arquitectura de componentes reutilizables
- **PWA Support**: Service Worker para caching y modo offline
- **Responsive**: Optimizado para móvil, tablet y desktop
- **Error Handling**: Fallbacks robustos y manejo de errores global

### Estructura del Frontend

```
assets/
├── index.html              # Página principal SPA
├── app.js                  # Router y lógica principal
├── components-inline.js    # Templates de componentes
├── styles.css              # Estilos Glassmorphism
├── sw.js                   # Service Worker PWA
├── manifest.json           # PWA manifest
└── generate-icons.html     # Generador de iconos PWA
```

### Generar Iconos PWA

1. Abre `assets/generate-icons.html` en tu navegador
2. Los iconos se generarán automáticamente
3. Descarga cada icono y guárdalo en la carpeta `assets/`
4. Nombres requeridos: icon-72.png, icon-96.png, icon-128.png, icon-144.png, icon-152.png, icon-192.png, icon-384.png, icon-512.png

## 🔧 Instalación

### Desde fuente

```bash
# Clonar el repositorio
git clone https://github.com/academiaos/educore-ultra.git
cd educore-ultra

# Copiar archivo de configuración
cp .env.example .env

# Editar configuración según necesidad
nano .env

# Compilar y ejecutar
cargo run --release
```

### Con Docker

```bash
# Construir imagen
docker build -t educore-ultra:latest .

# Ejecutar contenedor
docker run -d \
  -p 3000:3000 \
  -v $(pwd)/db:/app/db \
  -v $(pwd)/uploads:/app/uploads \
  --name educore-ultra \
  educore-ultra:latest
```

## ⚙️ Configuración

Variables de entorno disponibles:

```bash
# Servidor
HOST=0.0.0.0
PORT=3000

# Base de datos
DATABASE_PATH=./db/academia.db
DATABASE_MAX_CONNECTIONS=20

# JWT
JWT_SECRET=your-super-secret-jwt-key-change-in-production-min-32-chars
JWT_EXPIRATION_HOURS=24

# Cluster (opcional)
ENABLE_CLUSTER=false
NODE_ID=node-1
NODE_NAME=Alpha-Leader
SEED_NODES=192.168.1.101:3000,192.168.1.102:3000

# Logging
RUST_LOG=educore_ultra=debug,tower_http=debug,axum=debug

# Archivos
UPLOAD_DIR=./uploads
MAX_FILE_SIZE=52428800
```

## 📚 API REST

### Authentication

#### Login
```http
POST /api/auth/login
Content-Type: application/json

{
  "email": "admin@academiaos.com",
  "password": "admin123",
  "institution_code": "default"
}
```

#### Register
```http
POST /api/auth/register
Content-Type: application/json

{
  "institution_code": "default",
  "email": "user@example.com",
  "password": "password123",
  "first_name": "John",
  "last_name": "Doe",
  "role": "student"
}
```

#### Get Current User
```http
GET /api/auth/me
Authorization: Bearer <token>
```

#### Logout
```http
POST /api/auth/logout
Authorization: Bearer <token>
```

### Institutions

#### List Institutions
```http
GET /api/institutions?page=1&limit=20
```

#### Create Institution
```http
POST /api/institutions
Authorization: Bearer <token>
Content-Type: application/json

{
  "name": "My School",
  "code": "myschool",
  "domain": "myschool.edu"
}
```

#### Get Institution
```http
GET /api/institutions/:id
```

#### Update Institution
```http
PUT /api/institutions/:id
Authorization: Bearer <token>
Content-Type: application/json

{
  "name": "Updated School Name",
  "status": "active"
}
```

#### Delete Institution
```http
DELETE /api/institutions/:id
Authorization: Bearer <token>
```

### Students

#### List Students
```http
GET /api/students?page=1&limit=20&grade=1
Authorization: Bearer <token>
```

#### Create Student
```http
POST /api/students
Authorization: Bearer <token>
Content-Type: application/json

{
  "institution_id": 1,
  "user_id": 1,
  "student_id": "STU001",
  "grade_level": 1,
  "section": "A",
  "enrollment_date": "2024-01-01"
}
```

#### Get Student
```http
GET /api/students/:id
Authorization: Bearer <token>
```

#### Update Student
```http
PUT /api/students/:id
Authorization: Bearer <token>
Content-Type: application/json

{
  "grade_level": 2,
  "section": "B",
  "status": "active"
}
```

#### Delete Student
```http
DELETE /api/students/:id
Authorization: Bearer <token>
```

### Courses

#### List Courses
```http
GET /api/courses?page=1&limit=20
Authorization: Bearer <token>
```

#### Create Course
```http
POST /api/courses
Authorization: Bearer <token>
Content-Type: application/json

{
  "institution_id": 1,
  "name": "Mathematics 101",
  "code": "MATH101",
  "teacher_id": 1,
  "grade_level": 1,
  "schedule": "{\"monday\": \"09:00-10:00\"}"
}
```

#### Get Course
```http
GET /api/courses/:id
Authorization: Bearer <token>
```

#### Update Course
```http
PUT /api/courses/:id
Authorization: Bearer <token>
Content-Type: application/json

{
  "name": "Updated Course Name",
  "teacher_id": 2
}
```

#### Delete Course
```http
DELETE /api/courses/:id
Authorization: Bearer <token>
```

### Messages

#### List Messages
```http
GET /api/messages?page=1&limit=20
Authorization: Bearer <token>
```

#### Send Message
```http
POST /api/messages
Authorization: Bearer <token>
Content-Type: application/json

{
  "receiver_id": 2,
  "subject": "Meeting Request",
  "content": "Can we meet tomorrow?"
}
```

#### Get Message
```http
GET /api/messages/:id
Authorization: Bearer <token>
```

#### Mark as Read
```http
PUT /api/messages/:id
Authorization: Bearer <token>
```

#### Delete Message
```http
DELETE /api/messages/:id
Authorization: Bearer <token>
```

### Notifications

#### List Notifications
```http
GET /api/notifications?page=1&limit=20
Authorization: Bearer <token>
```

#### Create Notification
```http
POST /api/notifications
Authorization: Bearer <token>
Content-Type: application/json

{
  "user_id": 1,
  "title": "New Assignment",
  "message": "You have a new assignment due tomorrow",
  "type": "info"
}
```

#### Mark as Read
```http
PUT /api/notifications/:id/read
Authorization: Bearer <token>
```

#### Delete Notification
```http
DELETE /api/notifications/:id
Authorization: Bearer <token>
```

### Health & System

#### Health Check
```http
GET /health
```

#### Detailed Health Check
```http
GET /health/detailed
```

#### System Stats (Admin only)
```http
GET /api/admin/stats
Authorization: Bearer <token>
```

## 🏗️ Arquitectura

```
educore-ultra/
├── src/
│   ├── main.rs              # Entry point
│   ├── domain.rs            # Domain models
│   ├── error.rs             # Error handling
│   ├── db.rs                # Database initialization
│   ├── repository.rs        # Repository pattern
│   ├── service.rs           # Business logic
│   ├── auth.rs              # JWT authentication
│   ├── state.rs             # Application state
│   ├── files.rs             # File management
│   ├── cluster.rs           # Cluster coordination
│   └── web.rs               # API routes
├── assets/                  # Static assets
├── db/                      # Database files
├── uploads/                 # File uploads
├── Cargo.toml               # Dependencies
├── Dockerfile               # Docker configuration
└── README.md                # This file
```

## 🔐 Seguridad

- **JWT Tokens**: Tokens JWT con expiración configurable
- **Password Hashing**: Hashing con bcrypt
- **Role-Based Access Control**: Control de acceso basado en roles (admin, teacher, student, staff)
- **Multi-Tenant Isolation**: Aislamiento de datos por institución
- **CORS**: Configuración de CORS para seguridad de API

## 🚀 Despliegue

### Docker Compose

```yaml
version: '3.8'
services:
  educore-ultra:
    build: .
    ports:
      - "3000:3000"
    volumes:
      - ./db:/app/db
      - ./uploads:/app/uploads
    environment:
      - JWT_SECRET=your-secret-key
      - ENABLE_CLUSTER=false
    restart: unless-stopped
```

### Kubernetes

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: educore-ultra
spec:
  replicas: 3
  selector:
    matchLabels:
      app: educore-ultra
  template:
    metadata:
      labels:
        app: educore-ultra
    spec:
      containers:
      - name: educore-ultra
        image: educore-ultra:latest
        ports:
        - containerPort: 3000
        env:
        - name: JWT_SECRET
          valueFrom:
            secretKeyRef:
              name: educore-secrets
              key: jwt-secret
        - name: ENABLE_CLUSTER
          value: "true"
```

## 📊 Monitoreo

El sistema incluye endpoints de health check:

- `/health` - Health check básico
- `/health/detailed` - Health check detallado con estado de base de datos

## 🧪 Testing

```bash
# Ejecutar tests
cargo test

# Ejecutar tests con coverage
cargo tarpaulin --out Html
```

## 🤝 Contribución

1. Fork el proyecto
2. Crea una rama para tu feature (`git checkout -b feature/AmazingFeature`)
3. Commit tus cambios (`git commit -m 'Add some AmazingFeature'`)
4. Push a la rama (`git push origin feature/AmazingFeature`)
5. Abre un Pull Request

## 📝 Licencia

Este proyecto está licenciado bajo la Licencia MIT - ver el archivo LICENSE para detalles.

## 👥 Autores

- AcademiaOS Team

## 🙏 Agradecimientos

- Axum Framework
- SQLx
- Rust Community
