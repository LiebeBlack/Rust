// Service Layer - Clean Architecture Business Logic
// Use cases and business logic implementation

use crate::domain::*;
use crate::error::{AppError, Result, validate_email, validate_password, validate_required};
use crate::repository::*;
use chrono::{Utc, Duration};
use std::sync::Arc;

// ============================================================================
// AUTH SERVICE
// ============================================================================

pub struct AuthService {
    user_repo: Arc<UserRepository>,
    institution_repo: Arc<InstitutionRepository>,
}

impl AuthService {
    pub fn new(
        user_repo: Arc<UserRepository>,
        institution_repo: Arc<InstitutionRepository>,
    ) -> Self {
        Self {
            user_repo,
            institution_repo,
        }
    }

    pub async fn login(&self, request: LoginRequest, password_hash_check: impl Fn(&str, &str) -> Result<bool>) -> Result<Session> {
        // Validate input
        validate_email(&request.email)?;
        validate_password(&request.password)?;

        // Find user by email
        let user = self.user_repo.find_by_email(&request.email).await?;

        // Verify password
        let is_valid = password_hash_check(&request.password, &user.password_hash)?;
        if !is_valid {
            return Err(AppError::auth("Invalid credentials"));
        }

        // Verify user is active
        if user.status != "active" {
            return Err(AppError::auth("User account is not active"));
        }

        // Get institution
        let institution = self.institution_repo.find_by_id(user.institution_id).await?;

        // Check institution code if provided
        if let Some(code) = &request.institution_code {
            if &institution.code != code {
                return Err(AppError::auth("Invalid institution code"));
            }
        }

        // Generate token (will be done in auth.rs with JWT)
        let token = format!("jwt_token_placeholder_{}", user.id);

        let expires_at = Utc::now() + Duration::hours(24);

        Ok(Session {
            token,
            user,
            institution,
            expires_at,
        })
    }

    pub async fn register(&self, user: CreateUser, password_hash: String) -> Result<User> {
        // Validate input
        validate_email(&user.email)?;
        validate_password(&user.password)?;

        // Validate role
        let valid_roles = vec!["admin", "teacher", "student", "staff"];
        if !valid_roles.contains(&user.role.as_str()) {
            return Err(AppError::validation("Invalid role"));
        }

        // Check if institution exists
        self.institution_repo.find_by_id(user.institution_id).await?;

        // Check if email already exists
        match self.user_repo.find_by_email(&user.email).await {
            Ok(_) => return Err(AppError::conflict("Email already registered")),
            Err(_) => {} // Email doesn't exist, continue
        }

        // Create user
        self.user_repo.create(user, password_hash).await
    }

    pub async fn get_current_user(&self, user_id: i64) -> Result<User> {
        self.user_repo.find_by_id(user_id).await
    }
}

// ============================================================================
// INSTITUTION SERVICE
// ============================================================================

pub struct InstitutionService {
    institution_repo: Arc<InstitutionRepository>,
}

impl InstitutionService {
    pub fn new(institution_repo: Arc<InstitutionRepository>) -> Self {
        Self { institution_repo }
    }

    pub async fn create_institution(&self, institution: CreateInstitution) -> Result<Institution> {
        // Validate input
        validate_required("name", Some(&institution.name))?;
        validate_required("code", Some(&institution.code))?;

        if institution.name.len() < 3 {
            return Err(AppError::validation("Name must be at least 3 characters"));
        }

        if institution.code.len() < 2 {
            return Err(AppError::validation("Code must be at least 2 characters"));
        }

        // Check if code already exists
        match self.institution_repo.find_by_code(&institution.code).await {
            Ok(_) => return Err(AppError::conflict("Institution code already exists")),
            Err(_) => {} // Code doesn't exist, continue
        }

        self.institution_repo.create(institution).await
    }

    pub async fn get_institution(&self, id: i64) -> Result<Institution> {
        self.institution_repo.find_by_id(id).await
    }

    pub async fn get_institution_by_code(&self, code: &str) -> Result<Institution> {
        self.institution_repo.find_by_code(code).await
    }

    pub async fn list_institutions(&self, page: i64, limit: i64) -> Result<PaginatedResponse<Institution>> {
        self.institution_repo.find_all(page, limit).await
    }

    pub async fn update_institution(&self, id: i64, institution: UpdateInstitution) -> Result<Institution> {
        self.institution_repo.update(id, institution).await
    }

    pub async fn delete_institution(&self, id: i64) -> Result<()> {
        self.institution_repo.delete(id).await
    }
}

// ============================================================================
// STUDENT SERVICE
// ============================================================================

pub struct StudentService {
    student_repo: Arc<StudentRepository>,
    user_repo: Arc<UserRepository>,
}

impl StudentService {
    pub fn new(
        student_repo: Arc<StudentRepository>,
        user_repo: Arc<UserRepository>,
    ) -> Self {
        Self {
            student_repo,
            user_repo,
        }
    }

    pub async fn create_student(&self, student: CreateStudent) -> Result<Student> {
        // Validate input
        validate_required("student_id", Some(&student.student_id))?;

        if student.student_id.len() < 3 {
            return Err(AppError::validation("Student ID must be at least 3 characters"));
        }

        // Check if institution exists
        if let Some(user_id) = student.user_id {
            self.user_repo.find_by_id(user_id).await?;
        }

        // Check if student_id already exists in institution
        match self.student_repo.find_by_student_id(student.institution_id, &student.student_id).await {
            Ok(_) => return Err(AppError::conflict("Student ID already exists in this institution")),
            Err(_) => {} // Student ID doesn't exist, continue
        }

        self.student_repo.create(student).await
    }

    pub async fn get_student(&self, id: i64) -> Result<Student> {
        self.student_repo.find_by_id(id).await
    }

    pub async fn get_student_by_id(&self, institution_id: i64, student_id: &str) -> Result<Student> {
        self.student_repo.find_by_student_id(institution_id, student_id).await
    }

    pub async fn list_students(
        &self,
        institution_id: i64,
        page: i64,
        limit: i64,
        grade_level: Option<i32>,
    ) -> Result<PaginatedResponse<Student>> {
        self.student_repo.find_by_institution(institution_id, page, limit, grade_level).await
    }

    pub async fn update_student(&self, id: i64, student: UpdateStudent) -> Result<Student> {
        self.student_repo.update(id, student).await
    }

    pub async fn delete_student(&self, id: i64) -> Result<()> {
        self.student_repo.delete(id).await
    }
}

// ============================================================================
// COURSE SERVICE
// ============================================================================

pub struct CourseService {
    course_repo: Arc<CourseRepository>,
    user_repo: Arc<UserRepository>,
}

impl CourseService {
    pub fn new(
        course_repo: Arc<CourseRepository>,
        user_repo: Arc<UserRepository>,
    ) -> Self {
        Self {
            course_repo,
            user_repo,
        }
    }

    pub async fn create_course(&self, course: CreateCourse) -> Result<Course> {
        // Validate input
        validate_required("name", Some(&course.name))?;

        if course.name.len() < 3 {
            return Err(AppError::validation("Course name must be at least 3 characters"));
        }

        // Check if institution exists
        // (We'll assume institution exists for now, could add validation)

        // Check if teacher exists if provided
        if let Some(teacher_id) = course.teacher_id {
            self.user_repo.find_by_id(teacher_id).await?;
        }

        self.course_repo.create(course).await
    }

    pub async fn get_course(&self, id: i64) -> Result<Course> {
        self.course_repo.find_by_id(id).await
    }

    pub async fn list_courses(
        &self,
        institution_id: i64,
        page: i64,
        limit: i64,
    ) -> Result<PaginatedResponse<Course>> {
        self.course_repo.find_by_institution(institution_id, page, limit).await
    }

    pub async fn update_course(&self, id: i64, course: UpdateCourse) -> Result<Course> {
        self.course_repo.update(id, course).await
    }

    pub async fn delete_course(&self, id: i64) -> Result<()> {
        self.course_repo.delete(id).await
    }
}

// ============================================================================
// MESSAGE SERVICE
// ============================================================================

pub struct MessageService {
    message_repo: Arc<MessageRepository>,
    user_repo: Arc<UserRepository>,
}

impl MessageService {
    pub fn new(
        message_repo: Arc<MessageRepository>,
        user_repo: Arc<UserRepository>,
    ) -> Self {
        Self {
            message_repo,
            user_repo,
        }
    }

    pub async fn send_message(&self, message: CreateMessage) -> Result<Message> {
        // Validate input
        validate_required("content", Some(&message.content))?;

        if message.content.len() < 1 {
            return Err(AppError::validation("Message content cannot be empty"));
        }

        if message.content.len() > 5000 {
            return Err(AppError::validation("Message content is too long (max 5000 characters)"));
        }

        // Check if sender exists
        self.user_repo.find_by_id(message.sender_id).await?;

        // Check if receiver exists
        self.user_repo.find_by_id(message.receiver_id).await?;

        // Check if sender and receiver are in the same institution
        let sender = self.user_repo.find_by_id(message.sender_id).await?;
        let receiver = self.user_repo.find_by_id(message.receiver_id).await?;

        if sender.institution_id != receiver.institution_id {
            return Err(AppError::validation("Sender and receiver must be in the same institution"));
        }

        self.message_repo.create(message).await
    }

    pub async fn get_message(&self, id: i64) -> Result<Message> {
        self.message_repo.find_by_id(id).await
    }

    pub async fn list_messages(
        &self,
        receiver_id: i64,
        page: i64,
        limit: i64,
    ) -> Result<PaginatedResponse<Message>> {
        self.message_repo.find_by_receiver(receiver_id, page, limit).await
    }

    pub async fn mark_message_as_read(&self, id: i64) -> Result<Message> {
        self.message_repo.mark_as_read(id).await
    }

    pub async fn delete_message(&self, id: i64) -> Result<()> {
        self.message_repo.delete(id).await
    }
}

// ============================================================================
// NOTIFICATION SERVICE
// ============================================================================

pub struct NotificationService {
    notification_repo: Arc<NotificationRepository>,
    user_repo: Arc<UserRepository>,
}

impl NotificationService {
    pub fn new(
        notification_repo: Arc<NotificationRepository>,
        user_repo: Arc<UserRepository>,
    ) -> Self {
        Self {
            notification_repo,
            user_repo,
        }
    }

    pub async fn create_notification(&self, notification: CreateNotification) -> Result<Notification> {
        // Validate input
        validate_required("title", Some(&notification.title))?;
        validate_required("message", Some(&notification.message))?;

        if notification.title.len() < 1 {
            return Err(AppError::validation("Title cannot be empty"));
        }

        if notification.message.len() < 1 {
            return Err(AppError::validation("Message cannot be empty"));
        }

        if notification.title.len() > 200 {
            return Err(AppError::validation("Title is too long (max 200 characters)"));
        }

        if notification.message.len() > 1000 {
            return Err(AppError::validation("Message is too long (max 1000 characters)"));
        }

        // Check if user exists
        self.user_repo.find_by_id(notification.user_id).await?;

        self.notification_repo.create(notification).await
    }

    pub async fn get_notification(&self, id: i64) -> Result<Notification> {
        self.notification_repo.find_by_id(id).await
    }

    pub async fn list_notifications(
        &self,
        user_id: i64,
        page: i64,
        limit: i64,
    ) -> Result<PaginatedResponse<Notification>> {
        self.notification_repo.find_by_user(user_id, page, limit).await
    }

    pub async fn mark_notification_as_read(&self, id: i64) -> Result<Notification> {
        self.notification_repo.mark_as_read(id).await
    }

    pub async fn delete_notification(&self, id: i64) -> Result<()> {
        self.notification_repo.delete(id).await
    }
}

// ============================================================================
// SYSTEM SERVICE
// ============================================================================

pub struct SystemService {
    user_repo: Arc<UserRepository>,
    student_repo: Arc<StudentRepository>,
    course_repo: Arc<CourseRepository>,
    institution_repo: Arc<InstitutionRepository>,
}

impl SystemService {
    pub fn new(
        user_repo: Arc<UserRepository>,
        student_repo: Arc<StudentRepository>,
        course_repo: Arc<CourseRepository>,
        institution_repo: Arc<InstitutionRepository>,
    ) -> Self {
        Self {
            user_repo,
            student_repo,
            course_repo,
            institution_repo,
        }
    }

    pub async fn get_system_stats(&self) -> Result<SystemStats> {
        let total_users: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
            .fetch_one(&self.user_repo.pool)
            .await?;

        let total_students: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM students")
            .fetch_one(&self.student_repo.pool)
            .await?;

        let total_courses: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM courses")
            .fetch_one(&self.course_repo.pool)
            .await?;

        let total_institutions: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM institutions")
            .fetch_one(&self.institution_repo.pool)
            .await?;

        Ok(SystemStats {
            total_users,
            total_students,
            total_courses,
            total_institutions,
            active_sessions: 0, // Could be implemented with session tracking
        })
    }
}
