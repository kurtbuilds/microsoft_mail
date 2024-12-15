use crate::model::{Body, BodyType, Recipient};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std_ext::VecExt;

#[derive(Serialize, Deserialize, Debug)]
pub struct Flag {
    #[serde(rename = "flagStatus")]
    pub flag_status: String,
}

/// API object for microsoft email
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EmailMessage {
    #[serde(rename = "@odata.etag")]
    pub etag: String,
    pub body: Body,
    pub body_preview: String,
    pub categories: Vec<String>,
    pub to_recipients: Vec<Recipient>,
    pub cc_recipients: Vec<Recipient>,
    pub bcc_recipients: Vec<Recipient>,
    pub change_key: String,
    pub conversation_id: String,
    // no idea what this is, but it's a string, not a number, so it's not what we call the "sequence" number
    pub conversation_index: String,
    pub created_date_time: DateTime<Utc>,
    pub flag: Flag,
    pub from: Option<Recipient>,
    pub has_attachments: bool,
    pub id: String,
    pub importance: String,
    pub inference_classification: String,
    pub internet_message_id: String,
    pub is_delivery_receipt_requested: Option<bool>,
    pub is_draft: bool,
    pub is_read: bool,
    pub is_read_receipt_requested: bool,
    pub last_modified_date_time: DateTime<Utc>,
    pub parent_folder_id: String,
    pub received_date_time: DateTime<Utc>,
    pub reply_to: Vec<Recipient>,
    pub sender: Option<Recipient>,
    pub sent_date_time: DateTime<Utc>,
    pub subject: String,
    pub web_link: String,
}

impl Into<::email::Email> for EmailMessage {
    fn into(self) -> ::email::Email {
        ::email::Email {
            from: self.from.map(|c| c.into()).unwrap(),
            to: self.to_recipients.recollect(),
            cc: self.cc_recipients.recollect(),
            bcc: self.bcc_recipients.recollect(),
            subject: self.subject,
            body: match self.body.content_type {
                BodyType::Text => ::email::Body::Text(self.body.content),
                BodyType::Html => ::email::Body::Html(self.body.content),
            },
            attachments: Vec::new(),
            reply_to_message_id: None,
            thread_id: None,
        }
    }
}
