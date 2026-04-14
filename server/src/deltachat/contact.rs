//! DeltaChat contact management

use serde::{Deserialize, Serialize};

/// Represents a DeltaChat contact
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeltaChatContact {
    /// Email address (used as unique identifier)
    pub email: String,
    
    /// Display name
    pub display_name: Option<String>,
    
    /// Whether the contact is verified (Autocrypt verified)
    pub verified: bool,
    
    /// Contact's public PGP key (if available)
    pub public_key: Option<String>,
    
    /// Whether this contact is blocked
    pub is_blocked: bool,
    
    /// Last seen timestamp
    pub last_seen: Option<chrono::DateTime<chrono::Utc>>,
}

impl DeltaChatContact {
    /// Create a new unverified contact
    pub fn new(email: &str) -> Self {
        Self {
            email: email.to_string(),
            display_name: None,
            verified: false,
            public_key: None,
            is_blocked: false,
            last_seen: None,
        }
    }

    /// Create a new verified contact
    pub fn verified(email: &str, display_name: Option<&str>) -> Self {
        Self {
            email: email.to_string(),
            display_name: display_name.map(|s| s.to_string()),
            verified: true,
            public_key: None,
            is_blocked: false,
            last_seen: None,
        }
    }

    /// Check if contact is a valid email address
    pub fn is_valid_email(&self) -> bool {
        self.email.contains('@') && self.email.contains('.')
    }
}

/// Contact profile information from Autocrypt headers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutocryptProfile {
    /// Email address
    pub addr: String,
    
    /// Key preference (prefer-encrypt attribute)
    pub prefer_encrypt: Option<String>,
    
    /// Base64-encoded public key
    pub pubkey: String,
}

impl AutocryptProfile {
    /// Parse Autocrypt header value
    pub fn parse(header_value: &str) -> Option<Self> {
        let mut addr = None;
        let mut prefer_encrypt = None;
        let mut pubkey = None;

        for part in header_value.split(';') {
            let part = part.trim();
            if let Some(key_value) = part.split_once('=') {
                let key = key_value.0.trim();
                let value = key_value.1.trim().trim_matches('"');

                match key {
                    "addr" => addr = Some(value.to_string()),
                    "prefer-encrypt" => prefer_encrypt = Some(value.to_string()),
                    "keydata" => pubkey = Some(value.to_string()),
                    _ => {}
                }
            }
        }

        Some(Self {
            addr: addr?,
            prefer_encrypt,
            pubkey: pubkey?,
        })
    }
}
