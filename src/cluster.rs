// Cluster Coordination - Clean Architecture Distributed System
// Leader election, heartbeat, data synchronization for multi-server setup

use crate::domain::{Node, HeartbeatPayload, LoadMetrics, SyncResult, ClusterStatus};
use crate::error::{AppError, Result};
use crate::state::ClusterState;
use chrono::{Utc, Duration};
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::interval;

// ============================================================================
// CLUSTER CONFIGURATION
// ============================================================================

#[derive(Clone, Debug)]
pub struct ClusterConfig {
    pub heartbeat_interval_seconds: u64,
    pub election_timeout_seconds: u64,
    pub sync_interval_seconds: u64,
}

impl Default for ClusterConfig {
    fn default() -> Self {
        Self {
            heartbeat_interval_seconds: 10,
            election_timeout_seconds: 30,
            sync_interval_seconds: 60,
        }
    }
}

// ============================================================================
// CLUSTER COORDINATOR
// ============================================================================

pub struct ClusterCoordinator {
    state: Arc<ClusterState>,
    config: ClusterConfig,
    seed_nodes: Vec<String>,
    is_running: Arc<RwLock<bool>>,
}

impl ClusterCoordinator {
    pub fn new(state: Arc<ClusterState>, seed_nodes: Vec<String>, config: ClusterConfig) -> Self {
        Self {
            state,
            config,
            seed_nodes,
            is_running: Arc::new(RwLock::new(false)),
        }
    }

    pub async fn start(&self) -> Result<()> {
        let mut is_running = self.is_running.write().await;
        if *is_running {
            return Ok(());
        }
        *is_running = true;
        drop(is_running);

        // Start heartbeat task
        self.start_heartbeat_task().await;

        // Start leader election task
        self.start_election_task().await;

        // Start sync task
        self.start_sync_task().await;

        tracing::info!("Cluster coordinator started for node: {}", self.state.node_id);

        Ok(())
    }

    pub async fn stop(&self) -> Result<()> {
        let mut is_running = self.is_running.write().await;
        *is_running = false;
        tracing::info!("Cluster coordinator stopped");
        Ok(())
    }

    async fn start_heartbeat_task(&self) {
        let state = self.state.clone();
        let seed_nodes = self.seed_nodes.clone();
        let interval_seconds = self.config.heartbeat_interval_seconds;
        let is_running = self.is_running.clone();

        tokio::spawn(async move {
            let mut interval = interval(std::time::Duration::from_secs(interval_seconds));
            
            loop {
                interval.tick().await;

                let running = *is_running.read().await;
                if !running {
                    break;
                }

                // Send heartbeat to seed nodes
                for seed_node in &seed_nodes {
                    if let Err(e) = Self::send_heartbeat_to_node(
                        &state,
                        seed_node,
                    ).await {
                        tracing::warn!("Failed to send heartbeat to {}: {}", seed_node, e);
                    }
                }
            }
        });
    }

    async fn start_election_task(&self) {
        let state = self.state.clone();
        let seed_nodes = self.seed_nodes.clone();
        let timeout_seconds = self.config.election_timeout_seconds;
        let is_running = self.is_running.clone();

        tokio::spawn(async move {
            let mut interval = interval(std::time::Duration::from_secs(timeout_seconds));
            
            loop {
                interval.tick().await;

                let running = *is_running.read().await;
                if !running {
                    break;
                }

                // Check if we need to elect a new leader
                if let Err(e) = Self::check_and_elect_leader(
                    &state,
                    &seed_nodes,
                ).await {
                    tracing::error!("Leader election failed: {}", e);
                }
            }
        });
    }

    async fn start_sync_task(&self) {
        let state = self.state.clone();
        let seed_nodes = self.seed_nodes.clone();
        let interval_seconds = self.config.sync_interval_seconds;
        let is_running = self.is_running.clone();

        tokio::spawn(async move {
            let mut interval = interval(std::time::Duration::from_secs(interval_seconds));
            
            loop {
                interval.tick().await;

                let running = *is_running.read().await;
                if !running {
                    break;
                }

                // Sync data with other nodes
                for seed_node in &seed_nodes {
                    if let Err(e) = Self::sync_with_node(
                        &state,
                        seed_node,
                    ).await {
                        tracing::warn!("Failed to sync with {}: {}", seed_node, e);
                    }
                }
            }
        });
    }

    async fn send_heartbeat_to_node(state: &ClusterState, node_address: &str) -> Result<()> {
        let metrics = LoadMetrics {
            cpu_usage: Self::get_cpu_usage(),
            memory_usage: Self::get_memory_usage(),
            active_connections: 0, // Could be tracked
        };

        let payload = HeartbeatPayload {
            node_id: state.node_id.clone(),
            timestamp: Utc::now(),
            load_metrics: metrics,
            last_sequence: state.get_sequence().await,
        };

        // In a real implementation, this would be an HTTP request
        // For now, we'll just log it
        tracing::debug!("Sending heartbeat to {}: {:?}", node_address, payload);

        Ok(())
    }

    async fn check_and_elect_leader(state: &ClusterState, seed_nodes: &[String]) -> Result<()> {
        let nodes = state.get_nodes().await;
        
        // Check if current leader is still alive
        let current_leader = nodes.iter().find(|n| n.role == "leader");
        
        if let Some(leader) = current_leader {
            if let Some(last_heartbeat) = leader.last_heartbeat {
                let timeout = Duration::seconds(30);
                if Utc::now() - last_heartbeat > timeout {
                    // Leader is dead, need new election
                    tracing::warn!("Leader {} is dead, starting election", leader.id);
                    Self::elect_leader(state, seed_nodes).await?;
                }
            }
        } else {
            // No leader, elect one
            Self::elect_leader(state, seed_nodes).await?;
        }

        Ok(())
    }

    async fn elect_leader(state: &ClusterState, seed_nodes: &[String]) -> Result<()> {
        let mut all_nodes = state.get_nodes().await;
        
        // Add current node if not in list
        if !all_nodes.iter().any(|n| &n.id == &state.node_id) {
            all_nodes.push(Node {
                id: state.node_id.clone(),
                name: state.node_name.clone(),
                address: "localhost".to_string(), // Would be actual address
                role: "follower".to_string(),
                status: "active".to_string(),
                last_heartbeat: Some(Utc::now()),
                sequence: state.get_sequence().await,
            });
        }

        // Add seed nodes
        for seed in seed_nodes {
            if !all_nodes.iter().any(|n| &n.address == seed) {
                all_nodes.push(Node {
                    id: format!("node-{}", seed),
                    name: format!("Node-{}", seed),
                    address: seed.clone(),
                    role: "follower".to_string(),
                    status: "active".to_string(),
                    last_heartbeat: Some(Utc::now()),
                    sequence: 0,
                });
            }
        }

        // Elect leader (node with smallest ID)
        let leader_id = all_nodes
            .iter()
            .map(|n| n.id.clone())
            .min()
            .ok_or_else(|| AppError::cluster("No nodes available for election"))?;

        // Update roles
        for node in &mut all_nodes {
            if &node.id == &leader_id {
                node.role = "leader".to_string();
            } else {
                node.role = "follower".to_string();
            }
        }

        // Update state
        state.update_nodes(all_nodes).await;

        // Set leader status for current node
        let is_leader = leader_id == state.node_id;
        state.set_leader(is_leader).await;

        tracing::info!("Leader elected: {}, current node is leader: {}", leader_id, is_leader);

        Ok(())
    }

    async fn sync_with_node(state: &ClusterState, node_address: &str) -> Result<SyncResult> {
        let current_sequence = state.get_sequence().await;
        
        // In a real implementation, this would:
        // 1. Fetch changes from the other node since current_sequence
        // 2. Apply those changes locally
        // 3. Update local sequence
        
        // For now, we'll just simulate it
        tracing::debug!("Syncing with {} from sequence {}", node_address, current_sequence);

        Ok(SyncResult {
            applied: 0,
            sequence: current_sequence,
        })
    }

    fn get_cpu_usage() -> f64 {
        // In a real implementation, this would use system metrics
        0.0
    }

    fn get_memory_usage() -> f64 {
        // In a real implementation, this would use system metrics
        0.0
    }

    pub async fn handle_heartbeat(&self, payload: HeartbeatPayload) -> Result<()> {
        let mut nodes = self.state.get_nodes().await;
        
        // Update or add node
        if let Some(node) = nodes.iter_mut().find(|n| &n.id == &payload.node_id) {
            node.last_heartbeat = Some(payload.timestamp);
            node.sequence = payload.last_sequence;
        } else {
            nodes.push(Node {
                id: payload.node_id.clone(),
                name: format!("Node-{}", payload.node_id),
                address: "unknown".to_string(),
                role: "follower".to_string(),
                status: "active".to_string(),
                last_heartbeat: Some(payload.timestamp),
                sequence: payload.last_sequence,
            });
        }

        self.state.update_nodes(nodes).await;

        Ok(())
    }

    pub async fn get_cluster_status(&self) -> ClusterStatus {
        let nodes = self.state.get_nodes().await;
        let leader_id = nodes
            .iter()
            .find(|n| n.role == "leader")
            .map(|n| n.id.clone())
            .unwrap_or_else(|| "none".to_string());

        let total_nodes = nodes.len();
        let active_nodes = nodes.iter().filter(|n| n.status == "active").count();

        ClusterStatus {
            nodes,
            leader_id,
            total_nodes,
            active_nodes,
        }
    }
}

// ============================================================================
// CLUSTER HELPERS
// ============================================================================

pub fn is_cluster_enabled() -> bool {
    std::env::var("ENABLE_CLUSTER")
        .unwrap_or_else(|_| "false".to_string())
        .parse()
        .unwrap_or(false)
}

pub fn get_node_id() -> String {
    std::env::var("NODE_ID").unwrap_or_else(|_| {
        format!("node-{}", uuid::Uuid::new_v4())
    })
}

pub fn get_node_name() -> String {
    std::env::var("NODE_NAME").unwrap_or_else(|_| {
        format!("Node-{}", gethostname::gethostname().to_string_lossy())
    })
}

pub fn get_seed_nodes() -> Vec<String> {
    std::env::var("SEED_NODES")
        .unwrap_or_else(|_| String::new())
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}
