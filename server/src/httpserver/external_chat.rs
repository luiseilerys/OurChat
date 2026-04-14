//! External Chat HTTP API endpoints
//! 
//! This module provides HTTP endpoints for creating and managing external chat accounts
//! (DeltaChat and ArcaneChat) within the OurChat application.

use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::{Cfg, SharedData};
use base::database::DbPool;

/// External Chat service state
pub struct ExternalChatState {
    pub db_pool: DbPool,
    pub shared_data: Arc<SharedData>,
}

/// Request to create a DeltaChat account via chatmail server
#[derive(Debug, Deserialize)]
pub struct CreateDeltaChatRequest {
    pub chatmail_server: String,
    pub username: String,
    pub display_name: String,
}

/// Response for DeltaChat account creation
#[derive(Debug, Serialize)]
pub struct CreateDeltaChatResponse {
    pub account_id: i64,
    pub email: String,
    pub status: String,
}

/// Request to create an ArcaneChat account via chatmail server
#[derive(Debug, Deserialize)]
pub struct CreateArcaneChatRequest {
    pub chatmail_server: String,
    pub username: String,
    pub display_name: String,
}

/// Response for ArcaneChat account creation
#[derive(Debug, Serialize)]
pub struct CreateArcaneChatResponse {
    pub account_id: i64,
    pub chatmail_address: String,
    pub status: String,
}

/// Request to link an existing external chat account
#[derive(Debug, Deserialize)]
pub struct LinkExternalAccountRequest {
    pub account_type: String, // "deltachat" or "arcanechat"
    pub address: String,      // email or chatmail address
    pub credentials: String,  // password or access token
    pub display_name: String,
}

/// Response for linking external account
#[derive(Debug, Serialize)]
pub struct LinkExternalAccountResponse {
    pub account_id: i64,
    pub account_type: String,
    pub address: String,
    pub status: String,
}

/// Configure external chat routes
pub fn config() -> Router<Arc<ExternalChatState>> {
    axum::Router::new()
        .route("/external/deltachat/create", post(create_deltachat_account))
        .route("/external/arcanechat/create", post(create_arcanechat_account))
        .route("/external/link", post(link_external_account))
        .route("/external/list", get(list_external_accounts))
}

/// Create a new DeltaChat account using chatmail protocol
async fn create_deltachat_account(
    State(state): State<Arc<ExternalChatState>>,
    Json(req): Json<CreateDeltaChatRequest>,
) -> Result<Json<CreateDeltaChatResponse>, StatusCode> {
    tracing::info!(
        "Creating DeltaChat account: {}@{}",
        req.username,
        req.chatmail_server
    );

    // TODO: Implement actual chatmail account creation logic
    // This would involve:
    // 1. Making HTTP POST request to chatmail server's /register endpoint
    // 2. Receiving generated email credentials
    // 3. Storing credentials securely in OurChat database
    // 4. Initializing DeltaChat core RPC instance
    
    // Placeholder implementation
    let email = format!("{}@{}", req.username, req.chatmail_server.replace("https://", ""));
    
    Ok(Json(CreateDeltaChatResponse {
        account_id: 0, // Will be set by database insertion
        email,
        status: "active".to_string(),
    }))
}

/// Create a new ArcaneChat account using chatmail protocol
async fn create_arcanechat_account(
    State(state): State<Arc<ExternalChatState>>,
    Json(req): Json<CreateArcaneChatRequest>,
) -> Result<Json<CreateArcaneChatResponse>, StatusCode> {
    tracing::info!(
        "Creating ArcaneChat account: {}@{}",
        req.username,
        req.chatmail_server
    );

    // TODO: Implement actual chatmail account creation for ArcaneChat
    // Similar to DeltaChat but may use different server endpoints
    
    // Placeholder implementation
    let chatmail_address = format!("{}@{}", req.username, req.chatmail_server.replace("https://", ""));
    
    Ok(Json(CreateArcaneChatResponse {
        account_id: 0,
        chatmail_address,
        status: "active".to_string(),
    }))
}

/// Link an existing external chat account
async fn link_external_account(
    State(state): State<Arc<ExternalChatState>>,
    Json(req): Json<LinkExternalAccountRequest>,
) -> Result<Json<LinkExternalAccountResponse>, StatusCode> {
    tracing::info!(
        "Linking external {} account: {}",
        req.account_type,
        req.address
    );

    // TODO: Implement account linking logic
    // Validate credentials and store them securely
    
    Ok(Json(LinkExternalAccountResponse {
        account_id: 0,
        account_type: req.account_type,
        address: req.address,
        status: "active".to_string(),
    }))
}

/// List all linked external accounts for current user
async fn list_external_accounts(
    State(state): State<Arc<ExternalChatState>>,
) -> Result<Json<Vec<LinkExternalAccountResponse>>, StatusCode> {
    // TODO: Query database for external accounts linked to current user
    
    Ok(Json(vec![]))
}
