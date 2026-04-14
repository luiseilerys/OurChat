//! DeltaChat configuration

use serde::{Deserialize, Serialize};

/// Configuration for DeltaChat integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeltaChatConfig {
    /// Enable DeltaChat integration
    pub enabled: bool,
    
    /// Email address for DeltaChat account
    pub email_address: String,
    
    /// IMAP server configuration
    pub imap: ImapConfig,
    
    /// SMTP server configuration
    pub smtp: SmtpConfig,
    
    /// PGP key configuration for Autocrypt
    pub pgp: PgpConfig,
    
    /// Polling interval for new messages (in seconds)
    #[serde(default = "default_poll_interval")]
    pub poll_interval_secs: u64,
}

fn default_poll_interval() -> u64 {
    30
}

/// IMAP server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImapConfig {
    /// IMAP server address
    pub server: String,
    
    /// IMAP server port
    pub port: u16,
    
    /// Use TLS/SSL connection
    pub use_tls: bool,
    
    /// Username for authentication (if different from email)
    pub username: Option<String>,
    
    /// Password for authentication
    pub password: String,
}

/// SMTP server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmtpConfig {
    /// SMTP server address
    pub server: String,
    
    /// SMTP server port
    pub port: u16,
    
    /// Use TLS/SSL connection
    pub use_tls: bool,
    
    /// Username for authentication (if different from email)
    pub username: Option<String>,
    
    /// Password for authentication
    pub password: String,
}

/// PGP key configuration for Autocrypt encryption
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PgpConfig {
    /// Path to PGP private key file
    pub private_key_path: Option<String>,
    
    /// Path to PGP public key file
    pub public_key_path: Option<String>,
    
    /// PGP passphrase for private key
    pub passphrase: Option<String>,
    
    /// Auto-generate keys if not present
    #[serde(default = "default_true")]
    pub auto_generate: bool,
}

fn default_true() -> bool {
    true
}

impl DeltaChatConfig {
    /// Create a default configuration (disabled)
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            email_address: String::new(),
            imap: ImapConfig {
                server: String::new(),
                port: 993,
                use_tls: true,
                username: None,
                password: String::new(),
            },
            smtp: SmtpConfig {
                server: String::new(),
                port: 587,
                use_tls: true,
                username: None,
                password: String::new(),
            },
            pgp: PgpConfig {
                private_key_path: None,
                public_key_path: None,
                passphrase: None,
                auto_generate: true,
            },
            poll_interval_secs: 30,
        }
    }
}
