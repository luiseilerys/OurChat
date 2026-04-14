//! ArcaneChat configuration

use serde::{Deserialize, Serialize};

/// Configuration for ArcaneChat integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArcaneChatConfig {
    /// Enable ArcaneChat integration
    pub enabled: bool,
    
    /// ArcaneChat node/server address
    pub server_address: String,
    
    /// Server port
    pub server_port: u16,
    
    /// Use TLS/SSL connection
    #[serde(default = "default_true")]
    pub use_tls: bool,
    
    /// Authentication token (if required)
    pub auth_token: Option<String>,
    
    /// Connection timeout in seconds
    #[serde(default = "default_timeout")]
    pub timeout_secs: u64,
    
    /// Reconnection interval in seconds
    #[serde(default = "default_reconnect_interval")]
    pub reconnect_interval_secs: u64,
}

fn default_true() -> bool {
    true
}

fn default_timeout() -> u64 {
    30
}

fn default_reconnect_interval() -> u64 {
    60
}

impl ArcaneChatConfig {
    /// Create a default configuration (disabled)
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            server_address: String::new(),
            server_port: 0,
            use_tls: true,
            auth_token: None,
            timeout_secs: 30,
            reconnect_interval_secs: 60,
        }
    }

    /// Get the full server URL
    pub fn server_url(&self) -> String {
        let protocol = if self.use_tls { "wss" } else { "ws" };
        format!("{}://{}:{}", protocol, self.server_address, self.server_port)
    }
}
