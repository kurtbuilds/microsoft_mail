use crate::model::{Attachment, Page};
use crate::{FluentRequest, MicrosoftClient};
use futures::future::BoxFuture;
use httpclient::{InMemoryResponseExt, InMemoryResult};
use std::future::IntoFuture;

#[derive(Debug, Clone, Default)]
pub struct ListAttachmentsRequest {
    id: String,
    next: Option<String>,
}

impl<'a> FluentRequest<'a, ListAttachmentsRequest> {
    pub fn next(mut self, next: String) -> Self {
        self.params.next = Some(next);
        self
    }
}

impl MicrosoftClient {
    pub fn list_attachments(&self, message_id: &str) -> FluentRequest<ListAttachmentsRequest> {
        FluentRequest {
            client: self,
            params: ListAttachmentsRequest {
                id: message_id.to_string(),
                next: None,
            },
        }
    }
}

impl<'a> IntoFuture for FluentRequest<'a, ListAttachmentsRequest> {
    type Output = InMemoryResult<Page<Attachment>>;
    type IntoFuture = BoxFuture<'a, Self::Output>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = if let Some(next) = self.params.next {
                next
            } else {
                format!("/me/messages/{}/attachments", self.params.id)
            };
            let mut r = self.client.client.get(url);
            r = self.client.authorize(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
