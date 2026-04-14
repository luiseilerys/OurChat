//! ArcaneChat message types and handling

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents an ArcaneChat message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArcaneChatMessage {
    /// Unique message identifier
    pub id: String,
    
    /// Sender address/ID
    pub from: String,
    
    /// Recipient address/ID
    pub to: String,
    
    /// Message content
    pub content: String,
    
    /// Timestamp when message was sent
    pub timestamp: DateTime<Utc>,
    
    /// Whether the message is encrypted
    pub is_encrypted: bool,
    
    /// Whether the sender is verified
    pub is_verified: bool,
    
    /// Attachments included in the message
    pub attachments: Vec<MessageAttachment>,
    
    /// Chat room/channel ID (for group messages)
    pub room_id: Option<String>,
    
    /// Message type
    pub message_type: MessageType,
}

/// Attachment in an ArcaneChat message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageAttachment {
    /// Filename
    pub filename: String,
    
    /// MIME type
    pub mime_type: String,
    
    /// File size in bytes
    pub size: u64,
    
    /// URL or path to access the file
    pub url: Option<String>,
    
    /// File data (base64 encoded)
    pub data: Option<Vec<u8>>,
}

/// Type of ArcaneChat message
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
    
    /// System message
    System,
}

impl Default for MessageType {
    fn default() -> Self {
        MessageType::Text
    }
}

/// Chat room/channel in ArcaneChat
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArcaneChatRoom {
    /// Room ID
    pub id: String,
    
    /// Room name
    pub name: String,
    
    /// Room description
    pub description: Option<String>,
    
    /// Whether this is a private or public room
    pub is_public: bool,
    
    /// Room members
    pub members: Vec<String>,
    
    /// Last message timestamp
    pub last_message_time: Option<DateTime<Utc>>,
    
    /// Unread message count
    pub unread_count: u32,
}
