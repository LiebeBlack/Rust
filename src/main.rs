// EduCore Ultra - Main Entry Point
// Clean Architecture Backend for AcademiaOS

#![allow(dead_code)]
mod auth;
mod cluster;
mod db;
mod domain;
mod error;
mod files;
mod repository;
mod service;
mod state;
mod web;

use axum::{
    http::{
        header::{AUTHORIZATION, CONTENT_TYPE},
        Method,
    },
};
use cluster::{ClusterCoordinator, ClusterConfig, get_node_id, get_node_name, get_seed_nodes, is_cluster_enabled};
use db::{initialize_database, DatabaseConfig, seed_default_institution, seed_admin_user};
use error::Result;
use state::{AppStateBuilder, ServerConfig};
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    init_tracing();

    // Load environment variables
    dotenv::dotenv().ok();

    tracing::info!("Starting EduCore Ultra v{}", env!("CARGO_PKG_VERSION"));

    // Get server configuration
    let server_config = ServerConfig::from_env();
    tracing::info!("Server config: {:?}", server_config);

    // Initialize database
    let db_config = DatabaseConfig::default();
    let db_pool = initialize_database(&db_config).await?;
    tracing::info!("Database initialized successfully");

    // Seed default data
    seed_default_institution(&db_pool).await?;
    let default_institution = repository::InstitutionRepository::new(db_pool.clone())
        .find_by_code("default")
        .await?;
    
    // Seed admin user if not exists
    let password_hash = auth::PasswordService::hash_password("admin123")?;
    seed_admin_user(&db_pool, default_institution.id, "admin@academiaos.com", &password_hash).await?;
    tracing::info!("Default data seeded successfully");

    // Build application state
    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "your-super-secret-jwt-key-change-in-production".to_string());
    
    let app_state = AppStateBuilder::new()
        .with_db_pool(db_pool)
        .with_jwt_secret(jwt_secret)
        .with_jwt_expiration_hours(24)
        .build()?;

    // Initialize cluster if enabled
    if is_cluster_enabled() {
        let node_id = get_node_id();
        let node_name = get_node_name();
        let seed_nodes = get_seed_nodes();
        
        let cluster_state = Arc::new(state::ClusterState::new(node_id.clone(), node_name.clone()));
        let cluster_config = ClusterConfig::default();
        let cluster_coordinator = ClusterCoordinator::new(cluster_state, seed_nodes, cluster_config);
        
        cluster_coordinator.start().await?;
        tracing::info!("Cluster coordinator started for node: {}", node_id);
    }

    // Create router
    let app = web::create_router(app_state);

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers([AUTHORIZATION, CONTENT_TYPE]);

    let app = app.layer(cors);

    // Start server
    let addr = server_config.bind_address();
    let listener = TcpListener::bind(&addr).await?;
    tracing::info!("Server listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}

fn init_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "educore_ultra=debug,tower_http=debug,axum=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}
