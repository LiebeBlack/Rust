// Application State - Clean Architecture Shared State
// Shared application state for dependency injection and configuration

use crate::error::Result;
use crate::auth::{JwtConfig, SessionManager};
use crate::db::DbPool;
use crate::repository::*;
use crate::service::*;
use axum::extract::FromRef;
use std::sync::Arc;
use tokio::sync::RwLock;

// ============================================================================
// APPLICATION STATE
// ============================================================================

#[derive(Clone)]
pub struct AppState {
    pub db_pool: DbPool,
    pub jwt_config: JwtConfig,
    pub session_manager: Arc<SessionManager>,
    pub repositories: Arc<Repositories>,
    pub services: Arc<Services>,
}

// ============================================================================
// REPOSITORIES
// ============================================================================

#[derive(Clone)]
pub struct Repositories {
    pub institution: Arc<InstitutionRepository>,
    pub user: Arc<UserRepository>,
    pub student: Arc<StudentRepository>,
    pub course: Arc<CourseRepository>,
    pub message: Arc<MessageRepository>,
    pub notification: Arc<NotificationRepository>,
}

impl Repositories {
    pub fn new(db_pool: DbPool) -> Self {
        Self {
            institution: Arc::new(InstitutionRepository::new(db_pool.clone())),
            user: Arc::new(UserRepository::new(db_pool.clone())),
            student: Arc::new(StudentRepository::new(db_pool.clone())),
            course: Arc::new(CourseRepository::new(db_pool.clone())),
            message: Arc::new(MessageRepository::new(db_pool.clone())),
            notification: Arc::new(NotificationRepository::new(db_pool.clone())),
        }
    }
}

// ============================================================================
// SERVICES
// ============================================================================

#[derive(Clone)]
pub struct Services {
    pub auth: Arc<AuthService>,
    pub institution: Arc<InstitutionService>,
    pub student: Arc<StudentService>,
    pub course: Arc<CourseService>,
    pub message: Arc<MessageService>,
    pub notification: Arc<NotificationService>,
    pub system: Arc<SystemService>,
}

impl Services {
    pub fn new(repositories: Arc<Repositories>) -> Self {
        Self {
            auth: Arc::new(AuthService::new(
                repositories.user.clone(),
                repositories.institution.clone(),
            )),
            institution: Arc::new(InstitutionService::new(
                repositories.institution.clone(),
            )),
            student: Arc::new(StudentService::new(
                repositories.student.clone(),
                repositories.user.clone(),
            )),
            course: Arc::new(CourseService::new(
                repositories.course.clone(),
                repositories.user.clone(),
            )),
            message: Arc::new(MessageService::new(
                repositories.message.clone(),
                repositories.user.clone(),
            )),
            notification: Arc::new(NotificationService::new(
                repositories.notification.clone(),
                repositories.user.clone(),
            )),
            system: Arc::new(SystemService::new(
                repositories.user.clone(),
                repositories.student.clone(),
                repositories.course.clone(),
                repositories.institution.clone(),
            )),
        }
    }
}

// ============================================================================
// APP STATE BUILDER
// ============================================================================

pub struct AppStateBuilder {
    db_pool: Option<DbPool>,
    jwt_secret: Option<String>,
    jwt_expiration_hours: Option<i64>,
}

impl AppStateBuilder {
    pub fn new() -> Self {
        Self {
            db_pool: None,
            jwt_secret: None,
            jwt_expiration_hours: None,
        }
    }

    pub fn with_db_pool(mut self, pool: DbPool) -> Self {
        self.db_pool = Some(pool);
        self
    }

    pub fn with_jwt_secret(mut self, secret: String) -> Self {
        self.jwt_secret = Some(secret);
        self
    }

    pub fn with_jwt_expiration_hours(mut self, hours: i64) -> Self {
        self.jwt_expiration_hours = Some(hours);
        self
    }

    pub fn build(self) -> Result<AppState> {
        let db_pool = self.db_pool.ok_or_else(|| {
            crate::error::AppError::internal("Database pool is required")
        })?;

        let jwt_secret = self.jwt_secret.ok_or_else(|| {
            crate::error::AppError::internal("JWT secret is required")
        })?;

        let jwt_expiration_hours = self.jwt_expiration_hours.unwrap_or(24);

        let jwt_config = JwtConfig::new(jwt_secret, jwt_expiration_hours);
        let session_manager = Arc::new(SessionManager::new());
        let repositories = Arc::new(Repositories::new(db_pool.clone()));
        let services = Arc::new(Services::new(repositories.clone()));

        Ok(AppState {
            db_pool,
            jwt_config,
            session_manager,
            repositories,
            services,
        })
    }
}

impl Default for AppStateBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// IMPLEMENT FROMREF FOR AXUM
// ============================================================================

impl FromRef<AppState> for DbPool {
    fn from_ref(state: &AppState) -> Self {
        state.db_pool.clone()
    }
}

impl FromRef<AppState> for JwtConfig {
    fn from_ref(state: &AppState) -> Self {
        state.jwt_config.clone()
    }
}

impl FromRef<AppState> for Arc<Repositories> {
    fn from_ref(state: &AppState) -> Self {
        state.repositories.clone()
    }
}

impl FromRef<AppState> for Arc<Services> {
    fn from_ref(state: &AppState) -> Self {
        state.services.clone()
    }
}

// ============================================================================
// CLUSTER STATE (for multi-server coordination)
// ============================================================================

#[derive(Clone)]
pub struct ClusterState {
    pub node_id: String,
    pub node_name: String,
    pub is_leader: Arc<RwLock<bool>>,
    pub nodes: Arc<RwLock<Vec<crate::domain::Node>>>,
    pub sequence: Arc<RwLock<i64>>,
}

impl ClusterState {
    pub fn new(node_id: String, node_name: String) -> Self {
        Self {
            node_id,
            node_name,
            is_leader: Arc::new(RwLock::new(false)),
            nodes: Arc::new(RwLock::new(Vec::new())),
            sequence: Arc::new(RwLock::new(0)),
        }
    }

    pub async fn set_leader(&self, is_leader: bool) {
        let mut leader = self.is_leader.write().await;
        *leader = is_leader;
    }

    pub async fn is_leader(&self) -> bool {
        let leader = self.is_leader.read().await;
        *leader
    }

    pub async fn increment_sequence(&self) -> i64 {
        let mut seq = self.sequence.write().await;
        *seq += 1;
        *seq
    }

    pub async fn get_sequence(&self) -> i64 {
        let seq = self.sequence.read().await;
        *seq
    }

    pub async fn update_nodes(&self, nodes: Vec<crate::domain::Node>) {
        let mut nodes_state = self.nodes.write().await;
        *nodes_state = nodes;
    }

    pub async fn get_nodes(&self) -> Vec<crate::domain::Node> {
        let nodes_state = self.nodes.read().await;
        nodes_state.clone()
    }
}

// ============================================================================
// CONFIGURATION STATE
// ============================================================================

#[derive(Clone, Debug)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub enable_cluster: bool,
    pub node_id: Option<String>,
    pub node_name: Option<String>,
    pub seed_nodes: Vec<String>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 3000,
            enable_cluster: false,
            node_id: None,
            node_name: None,
            seed_nodes: Vec::new(),
        }
    }
}

impl ServerConfig {
    pub fn from_env() -> Self {
        Self {
            host: std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: std::env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .unwrap_or(3000),
            enable_cluster: std::env::var("ENABLE_CLUSTER")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),
            node_id: std::env::var("NODE_ID").ok(),
            node_name: std::env::var("NODE_NAME").ok(),
            seed_nodes: std::env::var("SEED_NODES")
                .unwrap_or_else(|_| String::new())
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect(),
        }
    }

    pub fn bind_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
