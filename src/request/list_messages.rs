use crate::model::{EmailMessage, Page};
use crate::{FluentRequest, MicrosoftClient};
use futures::future::BoxFuture;
use httpclient::{InMemoryResponseExt, InMemoryResult};
use std::future::IntoFuture;
use std_ext::default;

#[derive(Debug, Clone, Default)]
pub struct ListMessagesRequest {
    /// pseudo parameter. it's a next url. if it exists, the other parameters are ignored
    next: Option<String>,
    filter: Option<String>,
    select: Vec<String>,
    top: Option<u32>,
    skip: Option<u32>,
    order_by: Option<String>,
    mailbox: Option<String>,
}

impl MicrosoftClient {
    /// Returns a [`Page<EmailMessage>`]
    /// PRO TIP:
    /// Microsoft seems to not have any notion of "state" of the query. i.e. if you do date gt last week, and then add a $top and $skip,
    /// if a message comes in while you're querying, I **think** that means your offset will become wrong, and you might miss messages.
    /// I haven't confirmed whether that's the case. (Gmail's API has a notion of "next" tokens instead of top/skip to avoid this problem).
    /// As a solution that I think mostly works, generally make sure the filter also has date lt now, where now is fixed at the start of your loop
    /// and then, modulo distributed system shenanigans (a message is delivered late while you're querying), you should be good.
    /// Example filter/query syntax:
    /// https://graph.microsoft.com/v1.0/me/messages?$filter=subject eq '{subject}' and sender/emailAddress/address eq '{sender email address}' and sentDateTime ge 2023-05-17T07:28:08Z
    pub fn list_messages(&self) -> FluentRequest<ListMessagesRequest> {
        FluentRequest {
            client: self,
            params: default(),
        }
    }
}

impl<'a> FluentRequest<'a, ListMessagesRequest> {
    pub fn filter(mut self, filter: impl Into<String>) -> Self {
        self.params.filter = Some(filter.into());
        self
    }

    pub fn select(mut self, select: impl Into<Vec<String>>) -> Self {
        self.params.select = select.into();
        self
    }
    pub fn top(mut self, top: u32) -> Self {
        self.params.top = Some(top);
        self
    }
    pub fn skip(mut self, skip: u32) -> Self {
        self.params.skip = Some(skip);
        self
    }
    pub fn order_by(mut self, order_by: impl Into<String>) -> Self {
        self.params.order_by = Some(order_by.into());
        self
    }
    pub fn next(mut self, next: impl Into<String>) -> Self {
        self.params.next = Some(next.into());
        self
    }
}

impl<'a> IntoFuture for FluentRequest<'a, ListMessagesRequest> {
    type Output = InMemoryResult<Page<EmailMessage>>;
    type IntoFuture = BoxFuture<'a, Self::Output>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let mut r = if let Some(next) = self.params.next {
                self.client.client.get(next)
            } else {
                let url = if let Some(m) = self.params.mailbox {
                    format!("/users/{m}/messages")
                } else {
                    "/me/messages".to_string()
                };
                let mut r = self.client.client.get(url);
                if !self.params.select.is_empty() {
                    r = r.query("$select", &self.params.select.join(","));
                }
                if let Some(f) = self.params.filter {
                    // let filter = odata_params::filters::to_query_string(&f).unwrap();
                    r = r.query("$filter", &f);
                }
                if let Some(top) = self.params.top {
                    r = r.query("$top", &top.to_string());
                }
                if let Some(skip) = self.params.skip {
                    r = r.query("$skip", &skip.to_string());
                }
                if let Some(order_by) = self.params.order_by {
                    r = r.query("$orderby", &order_by);
                }
                r
            };
            r = self.client.authorize(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
