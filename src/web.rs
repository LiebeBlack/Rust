// Web Layer - Clean Architecture Presentation Layer
// REST API endpoints with Axum framework

use crate::auth::{AuthClaims, JwtService, PasswordService, has_required_role, is_admin};
use crate::domain::*;
use crate::error::{AppError, Result, validate_pagination};
use crate::state::AppState;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post, put, delete},
    Router,
};
use axum::response::Html;
use serde::Deserialize;

// ============================================================================
// QUERY PARAMS
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct StudentQuery {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub grade: Option<i32>,
}

// ============================================================================
// AUTH ENDPOINTS
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub institution_code: String,
    pub email: String,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub role: String,
}

pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<Session>> {
    
    let password_check = |password: &str, hash: &str| PasswordService::verify_password(password, hash);
    
    let session = state.services.auth.login(request, password_check).await?;
    
    // Generate JWT token
    let jwt_service = JwtService::new(state.jwt_config.clone());
    let token = jwt_service.generate_token(&session.user)?;
    
    // Create session
    state.session_manager.create_session(session.user.id).await?;
    
    let session_with_token = Session {
        token,
        user: session.user,
        institution: session.institution,
        expires_at: session.expires_at,
    };
    
    Ok(Json(session_with_token))
}

pub async fn register(
    State(state): State<AppState>,
    Json(request): Json<RegisterRequest>,
) -> Result<Json<User>> {
    // Find institution by code
    let institution = state.services.institution.get_institution_by_code(&request.institution_code).await?;
    
    // Hash password
    
    let password_hash = PasswordService::hash_password(&request.password)?;
    
    // Create user
    let user = CreateUser {
        institution_id: institution.id,
        email: request.email,
        password: request.password,
        role: request.role,
        first_name: request.first_name,
        last_name: request.last_name,
    };
    
    let created_user = state.services.auth.register(user, password_hash).await?;
    
    Ok(Json(created_user))
}

pub async fn logout(
    State(state): State<AppState>,
    auth: AuthClaims,
) -> Result<StatusCode> {
    state.session_manager.revoke_session(auth.claims.sub).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn get_me(
    State(state): State<AppState>,
    auth: AuthClaims,
) -> Result<Json<User>> {
    let user = state.services.auth.get_current_user(auth.claims.sub).await?;
    Ok(Json(user))
}

// ============================================================================
// INSTITUTION ENDPOINTS
// ============================================================================

pub async fn create_institution(
    State(state): State<AppState>,
    auth: AuthClaims,
    Json(institution): Json<CreateInstitution>,
) -> Result<Json<Institution>> {
    // Only admin can create institutions
    if !is_admin(&auth.claims.role) {
        return Err(AppError::forbidden("Only admins can create institutions"));
    }
    
    let created = state.services.institution.create_institution(institution).await?;
    Ok(Json(created))
}

pub async fn get_institutions(
    State(state): State<AppState>,
    Query(params): Query<PaginationQuery>,
) -> Result<Json<PaginatedResponse<Institution>>> {
    let (page, limit) = validate_pagination(params.page, params.limit)?;
    let institutions = state.services.institution.list_institutions(page, limit).await?;
    Ok(Json(institutions))
}

pub async fn get_institution(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Institution>> {
    let institution = state.services.institution.get_institution(id).await?;
    Ok(Json(institution))
}

pub async fn update_institution(
    State(state): State<AppState>,
    auth: AuthClaims,
    Path(id): Path<i64>,
    Json(institution): Json<UpdateInstitution>,
) -> Result<Json<Institution>> {
    if !is_admin(&auth.claims.role) {
        return Err(AppError::forbidden("Only admins can update institutions"));
    }
    
    let updated = state.services.institution.update_institution(id, institution).await?;
    Ok(Json(updated))
}

pub async fn delete_institution(
    State(state): State<AppState>,
    auth: AuthClaims,
    Path(id): Path<i64>,
) -> Result<StatusCode> {
    if !is_admin(&auth.claims.role) {
        return Err(AppError::forbidden("Only admins can delete institutions"));
    }
    
    state.services.institution.delete_institution(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

// ============================================================================
// STUDENT ENDPOINTS
// ============================================================================

pub async fn create_student(
    State(state): State<AppState>,
    auth: AuthClaims,
    Json(student): Json<CreateStudent>,
) -> Result<Json<Student>> {
    // Verify institution access
    crate::auth::verify_institution_access(auth.claims.institution_id, student.institution_id)?;
    
    // Check role
    if !has_required_role(&auth.claims.role, &["admin", "teacher", "staff"]) {
        return Err(AppError::forbidden("Insufficient permissions"));
    }
    
    let created = state.services.student.create_student(student).await?;
    Ok(Json(created))
}

pub async fn get_students(
    State(state): State<AppState>,
    auth: AuthClaims,
    Query(params): Query<StudentQuery>,
) -> Result<Json<PaginatedResponse<Student>>> {
    let (page, limit) = validate_pagination(params.page, params.limit)?;
    let students = state.services.student.list_students(
        auth.claims.institution_id,
        page,
        limit,
        params.grade,
    ).await?;
    Ok(Json(students))
}

pub async fn get_student(
    State(state): State<AppState>,
    auth: AuthClaims,
    Path(id): Path<i64>,
) -> Result<Json<Student>> {
    let student = state.services.student.get_student(id).await?;
    
    // Verify institution access
    crate::auth::verify_institution_access(auth.claims.institution_id, student.institution_id)?;
    
    Ok(Json(student))
}

pub async fn update_student(
    State(state): State<AppState>,
    auth: AuthClaims,
    Path(id): Path<i64>,
    Json(student): Json<UpdateStudent>,
) -> Result<Json<Student>> {
    if !has_required_role(&auth.claims.role, &["admin", "teacher", "staff"]) {
        return Err(AppError::forbidden("Insufficient permissions"));
    }
    
    let updated = state.services.student.update_student(id, student).await?;
    Ok(Json(updated))
}

pub async fn delete_student(
    State(state): State<AppState>,
    auth: AuthClaims,
    Path(id): Path<i64>,
) -> Result<StatusCode> {
    if !is_admin(&auth.claims.role) {
        return Err(AppError::forbidden("Only admins can delete students"));
    }
    
    state.services.student.delete_student(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

// ============================================================================
// COURSE ENDPOINTS
// ============================================================================

pub async fn create_course(
    State(state): State<AppState>,
    auth: AuthClaims,
    Json(course): Json<CreateCourse>,
) -> Result<Json<Course>> {
    crate::auth::verify_institution_access(auth.claims.institution_id, course.institution_id)?;
    
    if !has_required_role(&auth.claims.role, &["admin", "teacher"]) {
        return Err(AppError::forbidden("Insufficient permissions"));
    }
    
    let created = state.services.course.create_course(course).await?;
    Ok(Json(created))
}

pub async fn get_courses(
    State(state): State<AppState>,
    auth: AuthClaims,
    Query(params): Query<PaginationQuery>,
) -> Result<Json<PaginatedResponse<Course>>> {
    let (page, limit) = validate_pagination(params.page, params.limit)?;
    let courses = state.services.course.list_courses(auth.claims.institution_id, page, limit).await?;
    Ok(Json(courses))
}

pub async fn get_course(
    State(state): State<AppState>,
    auth: AuthClaims,
    Path(id): Path<i64>,
) -> Result<Json<Course>> {
    let course = state.services.course.get_course(id).await?;
    crate::auth::verify_institution_access(auth.claims.institution_id, course.institution_id)?;
    Ok(Json(course))
}

pub async fn update_course(
    State(state): State<AppState>,
    auth: AuthClaims,
    Path(id): Path<i64>,
    Json(course): Json<UpdateCourse>,
) -> Result<Json<Course>> {
    if !has_required_role(&auth.claims.role, &["admin", "teacher"]) {
        return Err(AppError::forbidden("Insufficient permissions"));
    }
    
    let updated = state.services.course.update_course(id, course).await?;
    Ok(Json(updated))
}

pub async fn delete_course(
    State(state): State<AppState>,
    auth: AuthClaims,
    Path(id): Path<i64>,
) -> Result<StatusCode> {
    if !is_admin(&auth.claims.role) {
        return Err(AppError::forbidden("Only admins can delete courses"));
    }
    
    state.services.course.delete_course(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

// ============================================================================
// MESSAGE ENDPOINTS
// ============================================================================

pub async fn send_message(
    State(state): State<AppState>,
    auth: AuthClaims,
    Json(message): Json<CreateMessage>,
) -> Result<Json<Message>> {
    let message_with_sender = CreateMessage {
        institution_id: auth.claims.institution_id,
        sender_id: auth.claims.sub,
        receiver_id: message.receiver_id,
        subject: message.subject,
        content: message.content,
    };
    
    let created = state.services.message.send_message(message_with_sender).await?;
    Ok(Json(created))
}

pub async fn get_messages(
    State(state): State<AppState>,
    auth: AuthClaims,
    Query(params): Query<PaginationQuery>,
) -> Result<Json<PaginatedResponse<Message>>> {
    let (page, limit) = validate_pagination(params.page, params.limit)?;
    let messages = state.services.message.list_messages(auth.claims.sub, page, limit).await?;
    Ok(Json(messages))
}

pub async fn get_message(
    State(state): State<AppState>,
    auth: AuthClaims,
    Path(id): Path<i64>,
) -> Result<Json<Message>> {
    let message = state.services.message.get_message(id).await?;
    
    // Verify user is the receiver
    if message.receiver_id != auth.claims.sub {
        return Err(AppError::forbidden("Access denied to this message"));
    }
    
    Ok(Json(message))
}

pub async fn mark_message_as_read(
    State(state): State<AppState>,
    auth: AuthClaims,
    Path(id): Path<i64>,
) -> Result<Json<Message>> {
    let message = state.services.message.get_message(id).await?;
    
    if message.receiver_id != auth.claims.sub {
        return Err(AppError::forbidden("Access denied to this message"));
    }
    
    let updated = state.services.message.mark_message_as_read(id).await?;
    Ok(Json(updated))
}

pub async fn delete_message(
    State(state): State<AppState>,
    auth: AuthClaims,
    Path(id): Path<i64>,
) -> Result<StatusCode> {
    let message = state.services.message.get_message(id).await?;
    
    if message.receiver_id != auth.claims.sub {
        return Err(AppError::forbidden("Access denied to this message"));
    }
    
    state.services.message.delete_message(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

// ============================================================================
// NOTIFICATION ENDPOINTS
// ============================================================================

pub async fn create_notification(
    State(state): State<AppState>,
    auth: AuthClaims,
    Json(notification): Json<CreateNotification>,
) -> Result<Json<Notification>> {
    if !has_required_role(&auth.claims.role, &["admin", "teacher", "staff"]) {
        return Err(AppError::forbidden("Insufficient permissions"));
    }
    
    let created = state.services.notification.create_notification(notification).await?;
    Ok(Json(created))
}

pub async fn get_notifications(
    State(state): State<AppState>,
    auth: AuthClaims,
    Query(params): Query<PaginationQuery>,
) -> Result<Json<PaginatedResponse<Notification>>> {
    let (page, limit) = validate_pagination(params.page, params.limit)?;
    let notifications = state.services.notification.list_notifications(auth.claims.sub, page, limit).await?;
    Ok(Json(notifications))
}

pub async fn mark_notification_as_read(
    State(state): State<AppState>,
    auth: AuthClaims,
    Path(id): Path<i64>,
) -> Result<Json<Notification>> {
    let notification = state.services.notification.get_notification(id).await?;
    
    if notification.user_id != auth.claims.sub {
        return Err(AppError::forbidden("Access denied to this notification"));
    }
    
    let updated = state.services.notification.mark_notification_as_read(id).await?;
    Ok(Json(updated))
}

pub async fn delete_notification(
    State(state): State<AppState>,
    auth: AuthClaims,
    Path(id): Path<i64>,
) -> Result<StatusCode> {
    let notification = state.services.notification.get_notification(id).await?;
    
    if notification.user_id != auth.claims.sub {
        return Err(AppError::forbidden("Access denied to this notification"));
    }
    
    state.services.notification.delete_notification(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

// ============================================================================
// HEALTH & SYSTEM ENDPOINTS
// ============================================================================

pub async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        timestamp: chrono::Utc::now(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

pub async fn detailed_health_check(
    State(state): State<AppState>,
) -> Result<Json<DetailedHealthResponse>> {
    let db_healthy = crate::db::health_check(&state.db_pool).await?;
    
    let status = if db_healthy { "healthy" } else { "unhealthy" };
    
    Ok(Json(DetailedHealthResponse {
        status: status.to_string(),
        timestamp: chrono::Utc::now(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        database: if db_healthy { "connected" } else { "disconnected" }.to_string(),
        uptime_seconds: 0, // Could track actual uptime
    }))
}

pub async fn get_system_stats(
    State(state): State<AppState>,
    auth: AuthClaims,
) -> Result<Json<SystemStats>> {
    if !is_admin(&auth.claims.role) {
        return Err(AppError::forbidden("Only admins can view system stats"));
    }
    
    let stats = state.services.system.get_system_stats().await?;
    Ok(Json(stats))
}

// ============================================================================
// STATIC FILE SERVING
// ============================================================================

pub async fn serve_index() -> Result<Html<&'static str>> {
    Ok(Html(include_str!("../assets/index.html")))
}

pub async fn serve_styles() -> Result<([(axum::http::header::HeaderName, &'static str); 1], &'static str)> {
    Ok(([
        (axum::http::header::CONTENT_TYPE, "text/css; charset=utf-8"),
    ], include_str!("../assets/styles.css")))
}

pub async fn serve_app_js() -> Result<([(axum::http::header::HeaderName, &'static str); 1], &'static str)> {
    Ok(([
        (axum::http::header::CONTENT_TYPE, "application/javascript; charset=utf-8"),
    ], include_str!("../assets/app.js")))
}

pub async fn serve_components_js() -> Result<([(axum::http::header::HeaderName, &'static str); 1], &'static str)> {
    Ok(([
        (axum::http::header::CONTENT_TYPE, "application/javascript; charset=utf-8"),
    ], include_str!("../assets/components-inline.js")))
}

pub async fn serve_sw_js() -> Result<([(axum::http::header::HeaderName, &'static str); 1], &'static str)> {
    Ok(([
        (axum::http::header::CONTENT_TYPE, "application/javascript; charset=utf-8"),
    ], include_str!("../assets/sw.js")))
}

pub async fn serve_manifest() -> Result<([(axum::http::header::HeaderName, &'static str); 1], &'static str)> {
    Ok(([
        (axum::http::header::CONTENT_TYPE, "application/json; charset=utf-8"),
    ], include_str!("../assets/manifest.json")))
}

// ============================================================================
// ROUTER BUILDER
// ============================================================================

pub fn create_router(state: AppState) -> Router {
    Router::new()
        // Static assets (SPA)
        .route("/", get(serve_index))
        .route("/index.html", get(serve_index))
        .route("/styles.css", get(serve_styles))
        .route("/app.js", get(serve_app_js))
        .route("/components-inline.js", get(serve_components_js))
        .route("/sw.js", get(serve_sw_js))
        .route("/manifest.json", get(serve_manifest))
        // Asset path aliases (for compatibility)
        .route("/assets/css/style.css", get(serve_styles))
        .route("/assets/js/app.js", get(serve_app_js))
        .route("/assets/js/components-inline.js", get(serve_components_js))
        
        // Health endpoints (public)
        .route("/health", get(health_check))
        .route("/health/detailed", get(detailed_health_check))
        
        // Auth endpoints (public)
        .route("/api/auth/login", post(login))
        .route("/api/auth/register", post(register))
        .route("/api/auth/logout", post(logout))
        .route("/api/auth/me", get(get_me))
        
        // Institution endpoints
        .route("/api/institutions", get(get_institutions).post(create_institution))
        .route("/api/institutions/:id", get(get_institution).put(update_institution).delete(delete_institution))
        
        // Student endpoints
        .route("/api/students", get(get_students).post(create_student))
        .route("/api/students/:id", get(get_student).put(update_student).delete(delete_student))
        
        // Course endpoints
        .route("/api/courses", get(get_courses).post(create_course))
        .route("/api/courses/:id", get(get_course).put(update_course).delete(delete_course))
        
        // Message endpoints
        .route("/api/messages", get(get_messages).post(send_message))
        .route("/api/messages/:id", get(get_message).put(mark_message_as_read).delete(delete_message))
        
        // Notification endpoints
        .route("/api/notifications", get(get_notifications).post(create_notification))
        .route("/api/notifications/:id/read", put(mark_notification_as_read))
        .route("/api/notifications/:id", delete(delete_notification))
        
        // System endpoints
        .route("/api/admin/stats", get(get_system_stats))
        
        .with_state(state)
}
