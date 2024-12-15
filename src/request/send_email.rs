use crate::model::{Body, BodyType, EmailMessage, Recipient};
use crate::{FluentRequest, MicrosoftClient};
use base64::engine::Engine;
use base64::prelude::BASE64_STANDARD;
use email::Email;
use file::File;
use futures::future::BoxFuture;
use html_escape::encode_text;
use httpclient::{InMemoryResponseExt, InMemoryResult, ProtocolError, Retry};
use serde::{de::Error, Deserialize, Serialize};
use std::future::IntoFuture;
use std::sync::{Arc, LazyLock};
use std::time::Duration;
use std_ext::VecExt;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Bool {
    True,
    False,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SendEmailRequestBody {
    pub message: SendEmailRequestMessage,
    pub save_to_sent_items: Bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SendEmailRequestMessage {
    pub subject: String,
    pub body: Body,
    pub from: Recipient,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub to_recipients: Vec<Recipient>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cc_recipients: Vec<Recipient>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub bcc_recipients: Vec<Recipient>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attachments: Vec<SendEmailRequestAttachment>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct PatchEmailRequestMessage {
    pub body: Option<Body>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub to_recipients: Vec<Recipient>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cc_recipients: Vec<Recipient>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub bcc_recipients: Vec<Recipient>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(
    rename_all = "camelCase",
    tag = "@odata.type",
    rename = "#microsoft.graph.fileAttachment"
)]
struct SendEmailRequestAttachment {
    pub name: String,
    /// base64
    pub content_bytes: String,
    /// mime
    pub content_type: String,
}

impl From<File> for SendEmailRequestAttachment {
    fn from(file: File) -> Self {
        let content_type = file.mime_type();
        let content_bytes = BASE64_STANDARD.encode(file.content);
        let name = file.name;
        SendEmailRequestAttachment {
            content_type,
            name,
            content_bytes,
        }
    }
}

impl MicrosoftClient {
    pub fn send_email(&self, email: Email) -> FluentRequest<Email> {
        FluentRequest {
            client: self,
            params: email,
        }
    }
}

static RETRY: LazyLock<Arc<Retry>> = LazyLock::new(|| {
    Arc::new(
        Retry::new()
            .backoff_delay(Duration::from_secs(1))
            .max_retries(10)
            .retry_codes(vec![429, 408, 425, 404]),
    )
});

impl<'a> IntoFuture for FluentRequest<'a, Email> {
    type Output = InMemoryResult<EmailMessage>;
    type IntoFuture = BoxFuture<'a, Self::Output>;

    fn into_future(self) -> Self::IntoFuture {
        use crate::model::Body as ModelBody;
        use ::email::Body;
        Box::pin(async move {
            let attachments = self.params.attachments.recollect();
            let email_message: EmailMessage = if let Some(id) = &self.params.reply_to_message_id {
                let url = format!("/me/messages/{id}/createReply", id = id);
                let mut draft = self.client.client.post(url);
                draft = self.client.authorize(draft);
                let mut draft: EmailMessage = draft.await?.json()?;
                // upload any attachments
                let attachment_upload_url =
                    format!("/me/messages/{id}/attachments", id = &draft.id);
                for attachment in attachments {
                    let mut r = self.client.client.post(&attachment_upload_url);
                    r = self.client.authorize(r);
                    r = r.json(attachment);
                    _ = r.await?;
                }
                // update the body & meta of the email
                assert_eq!(draft.body.content_type, BodyType::Html);
                let body: String = match self.params.body {
                    Body::Text(content) => encode_text(&content).into(),
                    Body::Html(content) => content,
                    Body::Combined { html, .. } => html,
                };
                let tag = "<body>";
                let Some(idx) = draft.body.content.find(tag) else {
                    return Err(httpclient::InMemoryError::Protocol(
                        ProtocolError::JsonError(serde_json::Error::custom("no body tag")),
                    ));
                };
                let idx = idx + tag.len();
                draft.body.content.insert_str(idx, &body);
                let mut data = PatchEmailRequestMessage::default();
                data.body = Some(ModelBody {
                    content_type: BodyType::Html,
                    content: draft.body.content,
                });
                data.to_recipients = self.params.to.recollect();
                data.cc_recipients = self.params.cc.recollect();
                data.bcc_recipients = self.params.bcc.recollect();
                let url = format!("/me/messages/{id}", id = &draft.id);
                // request the damn thing
                let mut r = self.client.client.patch(url);
                r = self.client.authorize(r);
                r = r.json(data);
                let res = r.await?;
                res.json()?
            } else {
                let mut r = self.client.client.post("/me/messages");
                let body = match self.params.body {
                    Body::Text(content) => ModelBody {
                        content_type: BodyType::Text,
                        content,
                    },
                    Body::Html(content) => ModelBody {
                        content_type: BodyType::Html,
                        content,
                    },
                    Body::Combined { html, .. } => ModelBody {
                        content_type: BodyType::Html,
                        content: html,
                    },
                };
                let body = SendEmailRequestMessage {
                    subject: self.params.subject,
                    from: Recipient {
                        email_address: self.params.from,
                    },
                    body,
                    to_recipients: self.params.to.recollect(),
                    cc_recipients: self.params.cc.recollect(),
                    bcc_recipients: self.params.bcc.recollect(),
                    attachments,
                };
                r = r.json(body);
                r = self.client.authorize(r);
                // for this to work, we need to set Prefer Immutable IDs, but we're already setting it at the lib level.
                // see https://learn.microsoft.com/en-us/graph/outlook-immutable-id
                let res = r.await?;
                res.json()?
            };
            let url = format!("/me/messages/{id}/send", id = &email_message.id);
            let mut r = self.client.client.post(url);
            r = self.client.authorize(r);
            r.middlewares.insert(0, RETRY.clone());
            _ = r.await?;
            Ok(email_message)
        })
    }
}
