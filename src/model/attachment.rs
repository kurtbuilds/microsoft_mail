use std::fmt::{Debug, Formatter};
use serde::{Deserialize, Serialize};
use base64::{engine::general_purpose::STANDARD, Engine};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
    #[serde(rename = "@odata.mediaContentType")]
    pub odata_media_content_type: String,
    #[serde(rename = "@odata.type")]
    pub odata_type: String,
    /// base64 encoded
    pub content_bytes: String,
    pub content_id: Option<String>,
    pub content_location: Option<String>,
    pub content_type: String,
    pub id: String,
    pub is_inline: bool,
    pub last_modified_date_time: DateTime<Utc>,
    pub name: String,
    pub size: u64,
}

impl Debug for Attachment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Attachment")
            .field("odata_media_content_type", &self.odata_media_content_type)
            .field("odata_type", &self.odata_type)
            // .field("content_bytes", &self.content_bytes)
            .field("content_id", &self.content_id)
            .field("content_location", &self.content_location)
            .field("content_type", &self.content_type)
            .field("id", &self.id)
            .field("is_inline", &self.is_inline)
            .field("last_modified_date_time", &self.last_modified_date_time)
            .field("name", &self.name)
            .field("size", &self.size)
            .finish()
    }
}

impl Attachment {
    pub fn bytes(&self) -> Vec<u8> {
        STANDARD.decode(&self.content_bytes).unwrap()
    }
}