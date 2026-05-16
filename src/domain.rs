// Domain Models - Clean Architecture Layer
// Pure data structures representing business entities

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// ============================================================================
// INSTITUTION (Tenant)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Institution {
    pub id: i64,
    pub name: String,
    pub code: String,
    pub domain: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInstitution {
    pub name: String,
    pub code: String,
    pub domain: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateInstitution {
    pub name: Option<String>,
    pub domain: Option<String>,
    pub status: Option<String>,
}

// ============================================================================
// USER
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub institution_id: i64,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub role: String, // 'admin', 'teacher', 'student', 'staff'
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub avatar_url: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUser {
    pub institution_id: i64,
    pub email: String,
    pub password: String,
    pub role: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUser {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub avatar_url: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
    pub institution_code: Option<String>,
}

// ============================================================================
// STUDENT
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Student {
    pub id: i64,
    pub institution_id: i64,
    pub user_id: Option<i64>,
    pub student_id: String,
    pub grade_level: Option<i32>,
    pub section: Option<String>,
    pub enrollment_date: Option<String>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateStudent {
    pub institution_id: i64,
    pub user_id: Option<i64>,
    pub student_id: String,
    pub grade_level: Option<i32>,
    pub section: Option<String>,
    pub enrollment_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateStudent {
    pub grade_level: Option<i32>,
    pub section: Option<String>,
    pub status: Option<String>,
}

// ============================================================================
// COURSE
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Course {
    pub id: i64,
    pub institution_id: i64,
    pub name: String,
    pub code: Option<String>,
    pub teacher_id: Option<i64>,
    pub grade_level: Option<i32>,
    pub schedule: Option<String>, // JSON
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCourse {
    pub institution_id: i64,
    pub name: String,
    pub code: Option<String>,
    pub teacher_id: Option<i64>,
    pub grade_level: Option<i32>,
    pub schedule: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCourse {
    pub name: Option<String>,
    pub teacher_id: Option<i64>,
    pub grade_level: Option<i32>,
    pub schedule: Option<String>,
    pub status: Option<String>,
}

// ============================================================================
// GRADE
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Grade {
    pub id: i64,
    pub institution_id: i64,
    pub student_id: i64,
    pub course_id: i64,
    pub period: Option<String>,
    pub score: Option<f64>,
    pub letter_grade: Option<String>,
    pub comments: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateGrade {
    pub institution_id: i64,
    pub student_id: i64,
    pub course_id: i64,
    pub period: Option<String>,
    pub score: Option<f64>,
    pub letter_grade: Option<String>,
    pub comments: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateGrade {
    pub period: Option<String>,
    pub score: Option<f64>,
    pub letter_grade: Option<String>,
    pub comments: Option<String>,
}

// ============================================================================
// ATTENDANCE
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Attendance {
    pub id: i64,
    pub institution_id: i64,
    pub student_id: i64,
    pub course_id: Option<i64>,
    pub date: String,
    pub status: String, // 'present', 'absent', 'late', 'excused'
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAttendance {
    pub institution_id: i64,
    pub student_id: i64,
    pub course_id: Option<i64>,
    pub date: String,
    pub status: String,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAttendance {
    pub status: Option<String>,
    pub notes: Option<String>,
}

// ============================================================================
// MESSAGE
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Message {
    pub id: i64,
    pub institution_id: i64,
    pub sender_id: i64,
    pub receiver_id: i64,
    pub subject: Option<String>,
    pub content: String,
    pub is_read: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMessage {
    pub institution_id: i64,
    pub sender_id: i64,
    pub receiver_id: i64,
    pub subject: Option<String>,
    pub content: String,
}

// ============================================================================
// NOTIFICATION
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Notification {
    pub id: i64,
    pub institution_id: i64,
    pub user_id: i64,
    pub title: String,
    pub message: String,
    pub notification_type: Option<String>, // 'info', 'warning', 'success', 'error'
    pub is_read: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateNotification {
    pub institution_id: i64,
    pub user_id: i64,
    pub title: String,
    pub message: String,
    pub notification_type: Option<String>,
}

// ============================================================================
// EMPLOYEE (HR)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Employee {
    pub id: i64,
    pub institution_id: i64,
    pub user_id: Option<i64>,
    pub employee_id: String,
    pub department: Option<String>,
    pub position: Option<String>,
    pub salary: Option<f64>,
    pub hire_date: Option<String>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEmployee {
    pub institution_id: i64,
    pub user_id: Option<i64>,
    pub employee_id: String,
    pub department: Option<String>,
    pub position: Option<String>,
    pub salary: Option<f64>,
    pub hire_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateEmployee {
    pub department: Option<String>,
    pub position: Option<String>,
    pub salary: Option<f64>,
    pub status: Option<String>,
}

// ============================================================================
// FILE
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct File {
    pub id: i64,
    pub institution_id: i64,
    pub original_name: String,
    pub stored_name: String, // SHA256 hash
    pub mime_type: Option<String>,
    pub size_bytes: Option<i64>,
    pub uploaded_by: Option<i64>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFile {
    pub institution_id: i64,
    pub original_name: String,
    pub stored_name: String,
    pub mime_type: Option<String>,
    pub size_bytes: Option<i64>,
    pub uploaded_by: Option<i64>,
}

// ============================================================================
// CERTIFICATE
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Certificate {
    pub id: i64,
    pub institution_id: i64,
    pub student_id: i64,
    pub certificate_type: String, // 'enrollment', 'grades', 'attendance'
    pub qr_code: String, // MD5 hash
    pub issue_date: Option<String>,
    pub valid_until: Option<String>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCertificate {
    pub institution_id: i64,
    pub student_id: i64,
    pub certificate_type: String,
    pub qr_code: String,
    pub issue_date: Option<String>,
    pub valid_until: Option<String>,
}

// ============================================================================
// JWT CLAIMS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64, // user_id
    pub institution_id: i64,
    pub role: String,
    pub email: String,
    pub exp: usize,
    pub iat: usize,
}

// ============================================================================
// PAGINATION
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
    pub pages: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationParams {
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

impl PaginationParams {
    pub fn new(page: Option<i64>, limit: Option<i64>) -> Self {
        Self {
            page: Some(page.unwrap_or(1)),
            limit: Some(limit.unwrap_or(20)),
        }
    }

    pub fn offset(&self) -> i64 {
        (self.page.unwrap_or(1) - 1) * self.limit.unwrap_or(20)
    }
}

// ============================================================================
// SESSION
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub token: String,
    pub user: User,
    pub institution: Institution,
    pub expires_at: DateTime<Utc>,
}

// ============================================================================
// HEALTH & SYSTEM
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: DateTime<Utc>,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedHealthResponse {
    pub status: String,
    pub timestamp: DateTime<Utc>,
    pub version: String,
    pub database: String,
    pub uptime_seconds: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStats {
    pub total_users: i64,
    pub total_students: i64,
    pub total_courses: i64,
    pub total_institutions: i64,
    pub active_sessions: i64,
}

// ============================================================================
// CLUSTER
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub name: String,
    pub address: String,
    pub role: String, // 'leader', 'follower'
    pub status: String, // 'active', 'inactive', 'failed'
    pub last_heartbeat: Option<DateTime<Utc>>,
    pub sequence: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeartbeatPayload {
    pub node_id: String,
    pub timestamp: DateTime<Utc>,
    pub load_metrics: LoadMetrics,
    pub last_sequence: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadMetrics {
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub active_connections: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncResult {
    pub applied: usize,
    pub sequence: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterStatus {
    pub nodes: Vec<Node>,
    pub leader_id: String,
    pub total_nodes: usize,
    pub active_nodes: usize,
}
