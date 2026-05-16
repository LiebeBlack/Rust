// Repository Pattern - Clean Architecture Data Layer
// Repository interfaces and implementations for data access

use crate::domain::*;
use crate::error::{AppError, Result};
use crate::db::DbPool;
use sqlx;
use chrono::Utc;

// ============================================================================
// INSTITUTION REPOSITORY
// ============================================================================

pub struct InstitutionRepository {
    pub(crate) pool: DbPool,
}

impl InstitutionRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, institution: CreateInstitution) -> Result<Institution> {
        let now = Utc::now();
        
        let id = sqlx::query(
            r#"
            INSERT INTO institutions (name, code, domain, status, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&institution.name)
        .bind(&institution.code)
        .bind(&institution.domain)
        .bind("active")
        .bind(now)
        .bind(now)
        .execute(&self.pool)
        .await?
        .last_insert_rowid();

        self.find_by_id(id).await
    }

    pub async fn find_by_id(&self, id: i64) -> Result<Institution> {
        sqlx::query_as("SELECT * FROM institutions WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                if e.to_string().contains("no rows") {
                    AppError::not_found(format!("Institution with id {} not found", id))
                } else {
                    AppError::Database(e)
                }
            })
    }

    pub async fn find_by_code(&self, code: &str) -> Result<Institution> {
        sqlx::query_as("SELECT * FROM institutions WHERE code = ?")
            .bind(code)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                if e.to_string().contains("no rows") {
                    AppError::not_found(format!("Institution with code {} not found", code))
                } else {
                    AppError::Database(e)
                }
            })
    }

    pub async fn find_all(&self, page: i64, limit: i64) -> Result<PaginatedResponse<Institution>> {
        let offset = (page - 1) * limit;

        let data = sqlx::query_as("SELECT * FROM institutions ORDER BY created_at DESC LIMIT ? OFFSET ?")
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?;

        let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM institutions")
            .fetch_one(&self.pool)
            .await?;

        let pages = (total + limit - 1) / limit;

        Ok(PaginatedResponse {
            data,
            total,
            page,
            limit,
            pages,
        })
    }

    pub async fn update(&self, id: i64, institution: UpdateInstitution) -> Result<Institution> {
        let now = Utc::now();
        
        let mut query = String::from("UPDATE institutions SET updated_at = ?");
        let mut params: Vec<String> = vec![now.to_rfc3339()];
        
        if institution.name.is_some() {
            query.push_str(&format!(", name = ?"));
            params.push(institution.name.unwrap());
        }
        if institution.domain.is_some() {
            query.push_str(&format!(", domain = ?"));
            params.push(institution.domain.unwrap());
        }
        if institution.status.is_some() {
            query.push_str(&format!(", status = ?"));
            params.push(institution.status.unwrap());
        }

        query.push_str(&format!(" WHERE id = ?"));

        let mut query_builder = sqlx::query(&query);
        for param in params {
            query_builder = query_builder.bind(param);
        }
        query_builder = query_builder.bind(id);

        query_builder.execute(&self.pool).await?;

        self.find_by_id(id).await
    }

    pub async fn delete(&self, id: i64) -> Result<()> {
        sqlx::query("DELETE FROM institutions WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}

// ============================================================================
// USER REPOSITORY
// ============================================================================

pub struct UserRepository {
    pub(crate) pool: DbPool,
}

impl UserRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, user: CreateUser, password_hash: String) -> Result<User> {
        let now = Utc::now();
        
        let id = sqlx::query(
            r#"
            INSERT INTO users (institution_id, email, password_hash, role, first_name, last_name, status, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(user.institution_id)
        .bind(&user.email)
        .bind(&password_hash)
        .bind(&user.role)
        .bind(&user.first_name)
        .bind(&user.last_name)
        .bind("active")
        .bind(now)
        .bind(now)
        .execute(&self.pool)
        .await?
        .last_insert_rowid();

        self.find_by_id(id).await
    }

    pub async fn find_by_id(&self, id: i64) -> Result<User> {
        sqlx::query_as("SELECT * FROM users WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                if e.to_string().contains("no rows") {
                    AppError::not_found(format!("User with id {} not found", id))
                } else {
                    AppError::Database(e)
                }
            })
    }

    pub async fn find_by_email(&self, email: &str) -> Result<User> {
        sqlx::query_as("SELECT * FROM users WHERE email = ?")
            .bind(email)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                if e.to_string().contains("no rows") {
                    AppError::not_found(format!("User with email {} not found", email))
                } else {
                    AppError::Database(e)
                }
            })
    }

    pub async fn find_by_institution(
        &self,
        institution_id: i64,
        page: i64,
        limit: i64,
    ) -> Result<PaginatedResponse<User>> {
        let offset = (page - 1) * limit;

        let data = sqlx::query_as(
            "SELECT * FROM users WHERE institution_id = ? ORDER BY created_at DESC LIMIT ? OFFSET ?",
        )
        .bind(institution_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users WHERE institution_id = ?")
            .bind(institution_id)
            .fetch_one(&self.pool)
            .await?;

        let pages = (total + limit - 1) / limit;

        Ok(PaginatedResponse {
            data,
            total,
            page,
            limit,
            pages,
        })
    }

    pub async fn update(&self, id: i64, user: UpdateUser) -> Result<User> {
        let now = Utc::now();
        
        let mut query = String::from("UPDATE users SET updated_at = ?");
        let mut params: Vec<String> = vec![now.to_rfc3339()];
        
        if user.first_name.is_some() {
            query.push_str(&format!(", first_name = ?"));
            params.push(user.first_name.unwrap());
        }
        if user.last_name.is_some() {
            query.push_str(&format!(", last_name = ?"));
            params.push(user.last_name.unwrap());
        }
        if user.avatar_url.is_some() {
            query.push_str(&format!(", avatar_url = ?"));
            params.push(user.avatar_url.unwrap());
        }
        if user.status.is_some() {
            query.push_str(&format!(", status = ?"));
            params.push(user.status.unwrap());
        }

        query.push_str(&format!(" WHERE id = ?"));

        let mut query_builder = sqlx::query(&query);
        for param in params {
            query_builder = query_builder.bind(param);
        }
        query_builder = query_builder.bind(id);

        query_builder.execute(&self.pool).await?;

        self.find_by_id(id).await
    }

    pub async fn delete(&self, id: i64) -> Result<()> {
        sqlx::query("DELETE FROM users WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn verify_active(&self, user_id: i64, institution_id: i64) -> Result<bool> {
        let user = self.find_by_id(user_id).await?;
        
        if user.institution_id != institution_id {
            return Err(AppError::forbidden("User does not belong to this institution"));
        }

        if user.status != "active" {
            return Err(AppError::auth("User account is not active"));
        }

        Ok(true)
    }
}

// ============================================================================
// STUDENT REPOSITORY
// ============================================================================

pub struct StudentRepository {
    pub(crate) pool: DbPool,
}

impl StudentRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, student: CreateStudent) -> Result<Student> {
        let id = sqlx::query(
            r#"
            INSERT INTO students (institution_id, user_id, student_id, grade_level, section, enrollment_date, status)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(student.institution_id)
        .bind(student.user_id)
        .bind(&student.student_id)
        .bind(student.grade_level)
        .bind(&student.section)
        .bind(&student.enrollment_date)
        .bind("active")
        .execute(&self.pool)
        .await?
        .last_insert_rowid();

        self.find_by_id(id).await
    }

    pub async fn find_by_id(&self, id: i64) -> Result<Student> {
        sqlx::query_as("SELECT * FROM students WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                if e.to_string().contains("no rows") {
                    AppError::not_found(format!("Student with id {} not found", id))
                } else {
                    AppError::Database(e)
                }
            })
    }

    pub async fn find_by_student_id(&self, institution_id: i64, student_id: &str) -> Result<Student> {
        sqlx::query_as("SELECT * FROM students WHERE institution_id = ? AND student_id = ?")
            .bind(institution_id)
            .bind(student_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                if e.to_string().contains("no rows") {
                    AppError::not_found(format!("Student with id {} not found", student_id))
                } else {
                    AppError::Database(e)
                }
            })
    }

    pub async fn find_by_institution(
        &self,
        institution_id: i64,
        page: i64,
        limit: i64,
        grade_level: Option<i32>,
    ) -> Result<PaginatedResponse<Student>> {
        let offset = (page - 1) * limit;

        let data = if let Some(grade) = grade_level {
            sqlx::query_as(
                "SELECT * FROM students WHERE institution_id = ? AND grade_level = ? ORDER BY created_at DESC LIMIT ? OFFSET ?",
            )
            .bind(institution_id)
            .bind(grade)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?
        } else {
            sqlx::query_as(
                "SELECT * FROM students WHERE institution_id = ? ORDER BY created_at DESC LIMIT ? OFFSET ?",
            )
            .bind(institution_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?
        };

        let total: i64 = if let Some(grade) = grade_level {
            sqlx::query_scalar("SELECT COUNT(*) FROM students WHERE institution_id = ? AND grade_level = ?")
                .bind(institution_id)
                .bind(grade)
                .fetch_one(&self.pool)
                .await?
        } else {
            sqlx::query_scalar("SELECT COUNT(*) FROM students WHERE institution_id = ?")
                .bind(institution_id)
                .fetch_one(&self.pool)
                .await?
        };

        let pages = (total + limit - 1) / limit;

        Ok(PaginatedResponse {
            data,
            total,
            page,
            limit,
            pages,
        })
    }

    pub async fn update(&self, id: i64, student: UpdateStudent) -> Result<Student> {
        let mut query = String::from("UPDATE students SET");
        let mut params: Vec<String> = vec![];
        let first = true;

        if student.grade_level.is_some() {
            if !first { query.push_str(","); }
            query.push_str(" grade_level = ?");
            params.push(student.grade_level.unwrap().to_string());
        }
        if student.section.is_some() {
            if !first { query.push_str(","); }
            query.push_str(" section = ?");
            params.push(student.section.unwrap());
        }
        if student.status.is_some() {
            if !first { query.push_str(","); }
            query.push_str(" status = ?");
            params.push(student.status.unwrap());
        }

        query.push_str(&format!(" WHERE id = ?"));

        let mut query_builder = sqlx::query(&query);
        for param in params {
            query_builder = query_builder.bind(param);
        }
        query_builder = query_builder.bind(id);

        query_builder.execute(&self.pool).await?;

        self.find_by_id(id).await
    }

    pub async fn delete(&self, id: i64) -> Result<()> {
        sqlx::query("DELETE FROM students WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}

// ============================================================================
// COURSE REPOSITORY
// ============================================================================

pub struct CourseRepository {
    pub(crate) pool: DbPool,
}

impl CourseRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, course: CreateCourse) -> Result<Course> {
        let now = Utc::now();
        
        let id = sqlx::query(
            r#"
            INSERT INTO courses (institution_id, name, code, teacher_id, grade_level, schedule, status, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(course.institution_id)
        .bind(&course.name)
        .bind(&course.code)
        .bind(course.teacher_id)
        .bind(course.grade_level)
        .bind(&course.schedule)
        .bind("active")
        .bind(now)
        .execute(&self.pool)
        .await?
        .last_insert_rowid();

        self.find_by_id(id).await
    }

    pub async fn find_by_id(&self, id: i64) -> Result<Course> {
        sqlx::query_as("SELECT * FROM courses WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                if e.to_string().contains("no rows") {
                    AppError::not_found(format!("Course with id {} not found", id))
                } else {
                    AppError::Database(e)
                }
            })
    }

    pub async fn find_by_institution(
        &self,
        institution_id: i64,
        page: i64,
        limit: i64,
    ) -> Result<PaginatedResponse<Course>> {
        let offset = (page - 1) * limit;

        let data = sqlx::query_as(
            "SELECT * FROM courses WHERE institution_id = ? ORDER BY created_at DESC LIMIT ? OFFSET ?",
        )
        .bind(institution_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM courses WHERE institution_id = ?")
            .bind(institution_id)
            .fetch_one(&self.pool)
            .await?;

        let pages = (total + limit - 1) / limit;

        Ok(PaginatedResponse {
            data,
            total,
            page,
            limit,
            pages,
        })
    }

    pub async fn update(&self, id: i64, course: UpdateCourse) -> Result<Course> {
        let mut query = String::from("UPDATE courses SET");
        let mut params: Vec<String> = vec![];
        let mut first = true;

        if course.name.is_some() {
            if !first { query.push_str(","); }
            query.push_str(" name = ?");
            params.push(course.name.unwrap());
            first = false;
        }
        if course.teacher_id.is_some() {
            if !first { query.push_str(","); }
            query.push_str(" teacher_id = ?");
            params.push(course.teacher_id.unwrap().to_string());
        }
        if course.grade_level.is_some() {
            if !first { query.push_str(","); }
            query.push_str(" grade_level = ?");
            params.push(course.grade_level.unwrap().to_string());
        }
        if course.schedule.is_some() {
            if !first { query.push_str(","); }
            query.push_str(" schedule = ?");
            params.push(course.schedule.unwrap());
        }
        if course.status.is_some() {
            if !first { query.push_str(","); }
            query.push_str(" status = ?");
            params.push(course.status.unwrap());
        }

        query.push_str(&format!(" WHERE id = ?"));

        let mut query_builder = sqlx::query(&query);
        for param in params {
            query_builder = query_builder.bind(param);
        }
        query_builder = query_builder.bind(id);

        query_builder.execute(&self.pool).await?;

        self.find_by_id(id).await
    }

    pub async fn delete(&self, id: i64) -> Result<()> {
        sqlx::query("DELETE FROM courses WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}

// ============================================================================
// MESSAGE REPOSITORY
// ============================================================================

pub struct MessageRepository {
    pub(crate) pool: DbPool,
}

impl MessageRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, message: CreateMessage) -> Result<Message> {
        let now = Utc::now();
        
        let id = sqlx::query(
            r#"
            INSERT INTO messages (institution_id, sender_id, receiver_id, subject, content, is_read, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(message.institution_id)
        .bind(message.sender_id)
        .bind(message.receiver_id)
        .bind(&message.subject)
        .bind(&message.content)
        .bind(false)
        .bind(now)
        .execute(&self.pool)
        .await?
        .last_insert_rowid();

        self.find_by_id(id).await
    }

    pub async fn find_by_id(&self, id: i64) -> Result<Message> {
        sqlx::query_as("SELECT * FROM messages WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                if e.to_string().contains("no rows") {
                    AppError::not_found(format!("Message with id {} not found", id))
                } else {
                    AppError::Database(e)
                }
            })
    }

    pub async fn find_by_receiver(
        &self,
        receiver_id: i64,
        page: i64,
        limit: i64,
    ) -> Result<PaginatedResponse<Message>> {
        let offset = (page - 1) * limit;

        let data = sqlx::query_as(
            "SELECT * FROM messages WHERE receiver_id = ? ORDER BY created_at DESC LIMIT ? OFFSET ?",
        )
        .bind(receiver_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM messages WHERE receiver_id = ?")
            .bind(receiver_id)
            .fetch_one(&self.pool)
            .await?;

        let pages = (total + limit - 1) / limit;

        Ok(PaginatedResponse {
            data,
            total,
            page,
            limit,
            pages,
        })
    }

    pub async fn mark_as_read(&self, id: i64) -> Result<Message> {
        sqlx::query("UPDATE messages SET is_read = 1 WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        self.find_by_id(id).await
    }

    pub async fn delete(&self, id: i64) -> Result<()> {
        sqlx::query("DELETE FROM messages WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}

// ============================================================================
// NOTIFICATION REPOSITORY
// ============================================================================

pub struct NotificationRepository {
    pub(crate) pool: DbPool,
}

impl NotificationRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, notification: CreateNotification) -> Result<Notification> {
        let now = Utc::now();
        
        let id = sqlx::query(
            r#"
            INSERT INTO notifications (institution_id, user_id, title, message, type, is_read, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(notification.institution_id)
        .bind(notification.user_id)
        .bind(&notification.title)
        .bind(&notification.message)
        .bind(&notification.notification_type)
        .bind(false)
        .bind(now)
        .execute(&self.pool)
        .await?
        .last_insert_rowid();

        self.find_by_id(id).await
    }

    pub async fn find_by_id(&self, id: i64) -> Result<Notification> {
        sqlx::query_as("SELECT * FROM notifications WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                if e.to_string().contains("no rows") {
                    AppError::not_found(format!("Notification with id {} not found", id))
                } else {
                    AppError::Database(e)
                }
            })
    }

    pub async fn find_by_user(
        &self,
        user_id: i64,
        page: i64,
        limit: i64,
    ) -> Result<PaginatedResponse<Notification>> {
        let offset = (page - 1) * limit;

        let data = sqlx::query_as(
            "SELECT * FROM notifications WHERE user_id = ? ORDER BY created_at DESC LIMIT ? OFFSET ?",
        )
        .bind(user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM notifications WHERE user_id = ?")
            .bind(user_id)
            .fetch_one(&self.pool)
            .await?;

        let pages = (total + limit - 1) / limit;

        Ok(PaginatedResponse {
            data,
            total,
            page,
            limit,
            pages,
        })
    }

    pub async fn mark_as_read(&self, id: i64) -> Result<Notification> {
        sqlx::query("UPDATE notifications SET is_read = 1 WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        self.find_by_id(id).await
    }

    pub async fn delete(&self, id: i64) -> Result<()> {
        sqlx::query("DELETE FROM notifications WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
