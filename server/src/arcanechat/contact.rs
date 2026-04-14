//! ArcaneChat contact management

use serde::{Deserialize, Serialize};

/// Represents an ArcaneChat contact
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArcaneChatContact {
    /// Unique address/ID
    pub address: String,
    
    /// Display name
    pub display_name: Option<String>,
    
    /// Whether the contact is verified
    pub verified: bool,
    
    /// Contact's public key (if available)
    pub public_key: Option<String>,
    
    /// Whether this contact is blocked
    pub is_blocked: bool,
    
    /// Last seen timestamp
    pub last_seen: Option<chrono::DateTime<chrono::Utc>>,
    
    /// Contact status/message
    pub status: Option<String>,
}

impl ArcaneChatContact {
    /// Create a new unverified contact
    pub fn new(address: &str) -> Self {
        Self {
            address: address.to_string(),
            display_name: None,
            verified: false,
            public_key: None,
            is_blocked: false,
            last_seen: None,
            status: None,
        }
    }

    /// Create a new verified contact
    pub fn verified(address: &str, display_name: Option<&str>) -> Self {
        Self {
            address: address.to_string(),
            display_name: display_name.map(|s| s.to_string()),
            verified: true,
            public_key: None,
            is_blocked: false,
            last_seen: None,
            status: None,
        }
    }

    /// Check if address is valid
    pub fn is_valid_address(&self) -> bool {
        !self.address.is_empty() && self.address.len() >= 3
    }
}

/// Contact profile information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactProfile {
    /// Address/ID
    pub address: String,
    
    /// Display name
    pub display_name: Option<String>,
    
    /// Status message
    pub status: Option<String>,
    
    /// Avatar URL or data
    pub avatar: Option<String>,
    
    /// Public encryption key
    pub public_key: Option<String>,
    
    /// Supported features/capabilities
    pub capabilities: Vec<String>,
}
