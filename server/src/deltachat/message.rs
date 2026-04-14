//! DeltaChat message types and handling

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents a DeltaChat message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeltaChatMessage {
    /// Unique message identifier
    pub id: String,
    
    /// Sender email address
    pub from: String,
    
    /// Recipient email address
    pub to: String,
    
    /// Message subject (used as chat title for first message)
    pub subject: String,
    
    /// Message body (plain text)
    pub body: String,
    
    /// HTML body (if present)
    pub html_body: Option<String>,
    
    /// Timestamp when message was sent
    pub timestamp: DateTime<Utc>,
    
    /// Whether the message is encrypted with Autocrypt
    pub is_encrypted: bool,
    
    /// Whether the sender is verified
    pub is_verified: bool,
    
    /// Attachments included in the message
    pub attachments: Vec<MessageAttachment>,
    
    /// Chat group ID (for group messages)
    pub chat_id: Option<String>,
    
    /// Message type (text, image, file, etc.)
    pub message_type: MessageType,
}

/// Attachment in a DeltaChat message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageAttachment {
    /// Filename
    pub filename: String,
    
    /// MIME type
    pub mime_type: String,
    
    /// File size in bytes
    pub size: u64,
    
    /// Content ID for inline attachments
    pub content_id: Option<String>,
    
    /// File data (base64 encoded)
    pub data: Vec<u8>,
}

/// Type of DeltaChat message
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageType {
    /// Plain text message
    Text,
    
    /// Image message
    Image,
    
    /// Video message
    Video,
    
    /// Audio message
    Audio,
    
    /// File attachment
    File,
    
    /// Voice message
    Voice,
    
    /// System message (e.g., member added to group)
    System,
}

impl Default for MessageType {
    fn default() -> Self {
        MessageType::Text
    }
}

/// Chat room/group in DeltaChat
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeltaChatRoom {
    /// Room ID
    pub id: String,
    
    /// Room name (derived from subject or group name)
    pub name: String,
    
    /// Whether this is a group chat
    pub is_group: bool,
    
    /// Group members (email addresses)
    pub members: Vec<String>,
    
    /// Last message timestamp
    pub last_message_time: Option<DateTime<Utc>>,
    
    /// Unread message count
    pub unread_count: u32,
}

/// Parse email headers to detect Autocrypt setup messages
pub fn is_autocrypt_setup_message(headers: &std::collections::HashMap<String, String>) -> bool {
    headers.get("autocrypt-setup-message") == Some(&"true".to_string())
}

/// Check if message has Autocrypt header
pub fn has_autocrypt_header(headers: &std::collections::HashMap<String, String>) -> bool {
    headers.contains_key("autocrypt")
}

/// Extract Chat-Group-ID header for group messages
pub fn extract_chat_group_id(headers: &std::collections::HashMap<String, String>) -> Option<String> {
    headers.get("chat-group-id").cloned()
}
