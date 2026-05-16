// Database Layer - Clean Architecture Data Layer
// SQLite WAL mode with migrations and connection pooling

use sqlx::{Sqlite, Pool};
use crate::error::{AppError, Result};
use std::path::Path;
use std::str::FromStr;

// ============================================================================
// DATABASE POOL TYPE
// ============================================================================

pub type DbPool = Pool<Sqlite>;

// ============================================================================
// DATABASE CONFIGURATION
// ============================================================================

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub database_path: String,
    pub max_connections: u32,
    pub enable_wal: bool,
    pub cache_size: i64,
    pub mmap_size: i64,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            database_path: "./db/academia.db".to_string(),
            max_connections: 20,
            enable_wal: true,
            cache_size: -256 * 1024, // 256MB negative = KB
            mmap_size: 2 * 1024 * 1024 * 1024, // 2GB
        }
    }
}

// ============================================================================
// DATABASE INITIALIZATION
// ============================================================================

pub async fn initialize_database(config: &DatabaseConfig) -> Result<DbPool> {
    // Ensure database directory exists
    if let Some(parent) = Path::new(&config.database_path).parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Create connection string
    let connection_string = format!("sqlite:{}", config.database_path);

    // Create connection pool
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(config.max_connections)
        .connect_with(
            sqlx::sqlite::SqliteConnectOptions::from_str(&connection_string)?
                .create_if_missing(true)
                .pragma("cache_size", config.cache_size.to_string())
                .pragma("mmap_size", config.mmap_size.to_string())
                .pragma("journal_mode", if config.enable_wal { "WAL" } else { "DELETE" })
                .pragma("synchronous", "NORMAL")
                .pragma("temp_store", "MEMORY")
        )
        .await?;

    // Run migrations
    run_migrations(&pool).await?;

    tracing::info!("Database initialized successfully at: {}", config.database_path);

    Ok(pool)
}

// ============================================================================
// MIGRATIONS
// ============================================================================

async fn run_migrations(pool: &DbPool) -> Result<()> {
    // Institutions (Tenants)
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS institutions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            code TEXT UNIQUE NOT NULL,
            domain TEXT,
            status TEXT DEFAULT 'active',
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Users (Multi-tenant)
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            institution_id INTEGER NOT NULL,
            email TEXT NOT NULL,
            password_hash TEXT NOT NULL,
            role TEXT NOT NULL,
            first_name TEXT,
            last_name TEXT,
            avatar_url TEXT,
            status TEXT DEFAULT 'active',
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (institution_id) REFERENCES institutions(id)
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Students
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS students (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            institution_id INTEGER NOT NULL,
            user_id INTEGER,
            student_id TEXT UNIQUE NOT NULL,
            grade_level INTEGER,
            section TEXT,
            enrollment_date DATE,
            status TEXT DEFAULT 'active',
            FOREIGN KEY (institution_id) REFERENCES institutions(id),
            FOREIGN KEY (user_id) REFERENCES users(id)
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Courses
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS courses (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            institution_id INTEGER NOT NULL,
            name TEXT NOT NULL,
            code TEXT,
            teacher_id INTEGER,
            grade_level INTEGER,
            schedule TEXT,
            status TEXT DEFAULT 'active',
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (institution_id) REFERENCES institutions(id),
            FOREIGN KEY (teacher_id) REFERENCES users(id)
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Grades
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS grades (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            institution_id INTEGER NOT NULL,
            student_id INTEGER NOT NULL,
            course_id INTEGER NOT NULL,
            period TEXT,
            score REAL,
            letter_grade TEXT,
            comments TEXT,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (institution_id) REFERENCES institutions(id),
            FOREIGN KEY (student_id) REFERENCES students(id),
            FOREIGN KEY (course_id) REFERENCES courses(id)
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Attendance
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS attendance (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            institution_id INTEGER NOT NULL,
            student_id INTEGER NOT NULL,
            course_id INTEGER,
            date DATE NOT NULL,
            status TEXT NOT NULL,
            notes TEXT,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (institution_id) REFERENCES institutions(id),
            FOREIGN KEY (student_id) REFERENCES students(id),
            FOREIGN KEY (course_id) REFERENCES courses(id)
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Messages
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS messages (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            institution_id INTEGER NOT NULL,
            sender_id INTEGER NOT NULL,
            receiver_id INTEGER NOT NULL,
            subject TEXT,
            content TEXT NOT NULL,
            is_read BOOLEAN DEFAULT 0,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (institution_id) REFERENCES institutions(id),
            FOREIGN KEY (sender_id) REFERENCES users(id),
            FOREIGN KEY (receiver_id) REFERENCES users(id)
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Notifications
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS notifications (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            institution_id INTEGER NOT NULL,
            user_id INTEGER NOT NULL,
            title TEXT NOT NULL,
            message TEXT NOT NULL,
            type TEXT,
            is_read BOOLEAN DEFAULT 0,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (institution_id) REFERENCES institutions(id),
            FOREIGN KEY (user_id) REFERENCES users(id)
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Employees (HR)
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS employees (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            institution_id INTEGER NOT NULL,
            user_id INTEGER,
            employee_id TEXT UNIQUE NOT NULL,
            department TEXT,
            position TEXT,
            salary REAL,
            hire_date DATE,
            status TEXT DEFAULT 'active',
            FOREIGN KEY (institution_id) REFERENCES institutions(id),
            FOREIGN KEY (user_id) REFERENCES users(id)
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Files (Deduplicated)
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS files (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            institution_id INTEGER NOT NULL,
            original_name TEXT NOT NULL,
            stored_name TEXT NOT NULL,
            mime_type TEXT,
            size_bytes INTEGER,
            uploaded_by INTEGER,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (institution_id) REFERENCES institutions(id),
            FOREIGN KEY (uploaded_by) REFERENCES users(id)
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Certificates (QR)
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS certificates (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            institution_id INTEGER NOT NULL,
            student_id INTEGER NOT NULL,
            type TEXT NOT NULL,
            qr_code TEXT UNIQUE NOT NULL,
            issue_date DATE,
            valid_until DATE,
            status TEXT DEFAULT 'active',
            FOREIGN KEY (institution_id) REFERENCES institutions(id),
            FOREIGN KEY (student_id) REFERENCES students(id)
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create indexes for performance
    create_indexes(pool).await?;

    tracing::info!("Database migrations completed successfully");

    Ok(())
}

// ============================================================================
// INDEXES
// ============================================================================

async fn create_indexes(pool: &DbPool) -> Result<()> {
    // Users indexes
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_users_institution ON users(institution_id)")
        .execute(pool)
        .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_users_email ON users(email)")
        .execute(pool)
        .await?;

    // Students indexes
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_students_institution ON students(institution_id)")
        .execute(pool)
        .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_students_grade ON students(grade_level)")
        .execute(pool)
        .await?;

    // Courses indexes
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_courses_institution ON courses(institution_id)")
        .execute(pool)
        .await?;

    // Grades indexes
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_grades_student ON grades(student_id)")
        .execute(pool)
        .await?;

    // Attendance indexes
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_attendance_date ON attendance(date)")
        .execute(pool)
        .await?;

    // Messages indexes
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_messages_receiver ON messages(receiver_id)")
        .execute(pool)
        .await?;

    // Notifications indexes
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_notifications_user ON notifications(user_id)")
        .execute(pool)
        .await?;

    // Files indexes
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_files_hash ON files(stored_name)")
        .execute(pool)
        .await?;

    Ok(())
}

// ============================================================================
// BACKUP FUNCTIONALITY
// ============================================================================

pub async fn create_backup(pool: &DbPool, backup_path: &str) -> Result<()> {
    tracing::info!("Creating database backup at: {}", backup_path);

    // Ensure backup directory exists
    if let Some(parent) = Path::new(backup_path).parent() {
        std::fs::create_dir_all(parent)?;
    }

    // SQLite backup using VACUUM INTO
    sqlx::query(&format!("VACUUM INTO '{}'", backup_path))
        .execute(pool)
        .await?;

    tracing::info!("Database backup completed successfully");

    Ok(())
}

// ============================================================================
// DATABASE HEALTH CHECK
// ============================================================================

pub async fn health_check(pool: &DbPool) -> Result<bool> {
    sqlx::query("SELECT 1")
        .fetch_one(pool)
        .await
        .map(|_| true)
        .map_err(|e| AppError::Database(e))
}

// ============================================================================
// SEED DATA (OPTIONAL)
// ============================================================================

pub async fn seed_default_institution(pool: &DbPool) -> Result<()> {
    // Check if default institution exists
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM institutions WHERE code = ?")
        .bind("default")
        .fetch_one(pool)
        .await?;

    if count == 0 {
        // Create default institution
        sqlx::query(
            r#"
            INSERT INTO institutions (name, code, domain, status)
            VALUES (?, ?, ?, ?)
            "#,
        )
        .bind("Default Institution")
        .bind("default")
        .bind("localhost")
        .bind("active")
        .execute(pool)
        .await?;

        tracing::info!("Default institution created");
    }

    Ok(())
}

pub async fn seed_admin_user(pool: &DbPool, institution_id: i64, email: &str, password_hash: &str) -> Result<()> {
    // Check if admin user exists
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users WHERE email = ?")
        .bind(email)
        .fetch_one(pool)
        .await?;

    if count == 0 {
        // Create admin user
        sqlx::query(
            r#"
            INSERT INTO users (institution_id, email, password_hash, role, first_name, last_name, status)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(institution_id)
        .bind(email)
        .bind(password_hash)
        .bind("admin")
        .bind("System")
        .bind("Administrator")
        .bind("active")
        .execute(pool)
        .await?;

        tracing::info!("Admin user created: {}", email);
    }

    Ok(())
}
