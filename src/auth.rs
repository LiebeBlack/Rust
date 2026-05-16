// Authentication Layer - Clean Architecture Security
// JWT multi-tenant authentication and authorization

use crate::domain::{Claims, User};
use crate::error::{AppError, Result};
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
    RequestPartsExt,
};
use axum_extra::TypedHeader;
use headers::{authorization::Bearer, Authorization};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use chrono::{Utc, Duration};
use std::sync::Arc;

// ============================================================================
// JWT CONFIGURATION
// ============================================================================

#[derive(Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub expiration_hours: i64,
}

impl JwtConfig {
    pub fn new(secret: String, expiration_hours: i64) -> Self {
        Self {
            secret,
            expiration_hours,
        }
    }

    pub fn encoding_key(&self) -> EncodingKey {
        EncodingKey::from_secret(self.secret.as_ref())
    }

    pub fn decoding_key(&self) -> DecodingKey {
        DecodingKey::from_secret(self.secret.as_ref())
    }
}

// ============================================================================
// JWT SERVICE
// ============================================================================

pub struct JwtService {
    config: JwtConfig,
}

impl JwtService {
    pub fn new(config: JwtConfig) -> Self {
        Self { config }
    }

    pub fn generate_token(&self, user: &User) -> Result<String> {
        let now = Utc::now();
        let exp = now + Duration::hours(self.config.expiration_hours);

        let claims = Claims {
            sub: user.id,
            institution_id: user.institution_id,
            role: user.role.clone(),
            email: user.email.clone(),
            exp: exp.timestamp() as usize,
            iat: now.timestamp() as usize,
        };

        encode(
            &Header::default(),
            &claims,
            &self.config.encoding_key(),
        )
        .map_err(|e| AppError::jwt(format!("Failed to generate token: {}", e)))
    }

    pub fn decode_token(&self, token: &str) -> Result<Claims> {
        let token_data = decode::<Claims>(
            token,
            &self.config.decoding_key(),
            &Validation::default(),
        )
        .map_err(|e| AppError::jwt(format!("Failed to decode token: {}", e)))?;

        Ok(token_data.claims)
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims> {
        let claims = self.decode_token(token)?;

        // Check if token is expired
        let now = Utc::now().timestamp() as usize;
        if claims.exp < now {
            return Err(AppError::jwt("Token has expired"));
        }

        Ok(claims)
    }
}

// ============================================================================
// PASSWORD HASHING
// ============================================================================

pub struct PasswordService;

impl PasswordService {
    pub fn hash_password(password: &str) -> Result<String> {
        bcrypt::hash(password, bcrypt::DEFAULT_COST)
            .map_err(|e| AppError::internal(format!("Failed to hash password: {}", e)))
    }

    pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
        bcrypt::verify(password, hash)
            .map_err(|e| AppError::internal(format!("Failed to verify password: {}", e)))
    }
}

// ============================================================================
// AXUM JWT EXTRACTOR
// ============================================================================

#[derive(Debug, Clone)]
pub struct AuthClaims {
    pub claims: Claims,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthClaims
where
    S: Send + Sync,
    JwtConfig: FromRef<S>,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self> {
        // Extract JWT config from state
        let jwt_config = JwtConfig::from_ref(state);

        // Extract Authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AppError::Unauthorized)?;

        // Verify token
        let jwt_service = JwtService::new(jwt_config);
        let claims = jwt_service.verify_token(bearer.token())?;

        Ok(AuthClaims { claims })
    }
}

// ============================================================================
// ROLE-BASED AUTHORIZATION
// ============================================================================

#[derive(Debug, Clone)]
pub struct RequireRole {
    pub roles: Vec<String>,
}

#[async_trait]
impl<S> FromRequestParts<S> for RequireRole
where
    S: Send + Sync,
    AuthClaims: FromRequestParts<S, Rejection = AppError>,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self> {
        let auth_claims = AuthClaims::from_request_parts(parts, state).await?;

        // For now, return empty RequireRole (role checking will be done in handlers)
        Ok(RequireRole {
            roles: vec![auth_claims.claims.role],
        })
    }
}

pub fn has_required_role(user_role: &str, required_roles: &[&str]) -> bool {
    required_roles.contains(&user_role)
}

pub fn is_admin(user_role: &str) -> bool {
    user_role == "admin"
}

pub fn is_teacher(user_role: &str) -> bool {
    user_role == "teacher" || user_role == "admin"
}

pub fn is_student(user_role: &str) -> bool {
    user_role == "student" || user_role == "admin"
}

pub fn is_staff(user_role: &str) -> bool {
    user_role == "staff" || user_role == "admin"
}

// ============================================================================
// MULTI-TENANT VERIFICATION
// ============================================================================

pub fn verify_institution_access(user_institution_id: i64, requested_institution_id: i64) -> Result<()> {
    if user_institution_id != requested_institution_id {
        return Err(AppError::forbidden("Access denied to this institution"));
    }
    Ok(())
}

// ============================================================================
// AUTH MIDDLEWARE HELPERS
// ============================================================================

pub fn extract_user_id_from_claims(claims: &Claims) -> i64 {
    claims.sub
}

pub fn extract_institution_id_from_claims(claims: &Claims) -> i64 {
    claims.institution_id
}

pub fn extract_role_from_claims(claims: &Claims) -> &str {
    &claims.role
}

// ============================================================================
// SESSION MANAGEMENT
// ============================================================================

#[derive(Clone)]
pub struct SessionManager {
    // In a production system, this would use Redis or similar
    // For now, we'll use in-memory storage
    active_sessions: Arc<tokio::sync::RwLock<std::collections::HashMap<i64, i64>>>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            active_sessions: Arc::new(tokio::sync::RwLock::new(std::collections::HashMap::new())),
        }
    }

    pub async fn create_session(&self, user_id: i64) -> Result<()> {
        let mut sessions = self.active_sessions.write().await;
        sessions.insert(user_id, Utc::now().timestamp());
        Ok(())
    }

    pub async fn validate_session(&self, user_id: i64) -> Result<bool> {
        let sessions = self.active_sessions.read().await;
        Ok(sessions.contains_key(&user_id))
    }

    pub async fn revoke_session(&self, user_id: i64) -> Result<()> {
        let mut sessions = self.active_sessions.write().await;
        sessions.remove(&user_id);
        Ok(())
    }

    pub async fn get_active_session_count(&self) -> usize {
        let sessions = self.active_sessions.read().await;
        sessions.len()
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}
