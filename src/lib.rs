pub mod model;
pub mod request;

use crate::model::User;
use httpclient::{InMemoryResponseExt, InMemoryResult, RequestBuilder};
use httpclient_oauth2::RefreshData;
use std::borrow::Cow;
use std::sync::{Arc, OnceLock};
// pub use odata_params::filters::*;

static SHARED_OAUTH2FLOW: OnceLock<httpclient_oauth2::OAuth2Flow> = OnceLock::new();

static SHARED_HTTPCLIENT: OnceLock<httpclient::Client> = OnceLock::new();

pub fn init_oauth2_flow(init: httpclient_oauth2::OAuth2Flow) {
    let _ = SHARED_OAUTH2FLOW.set(init);
}

pub fn default_http_client() -> httpclient::Client {
    httpclient::Client::new()
        .no_default_headers()
        .base_url("https://graph.microsoft.com/v1.0")
}

pub fn shared_oauth2_flow() -> &'static httpclient_oauth2::OAuth2Flow {
    SHARED_OAUTH2FLOW.get_or_init(|| httpclient_oauth2::OAuth2Flow {
        client_id: std::env::var("MICROSOFT_CLIENT_ID").expect("MICROSOFT_CLIENT_ID must be set"),
        client_secret: std::env::var("MICROSOFT_CLIENT_SECRET").expect("MICROSOFT_CLIENT_SECRET must be set"),
        init_endpoint: "https://login.microsoftonline.com/common/oauth2/v2.0/authorize".to_string(),
        exchange_endpoint: "https://login.microsoftonline.com/common/oauth2/v2.0/token".to_string(),
        refresh_endpoint: "https://login.microsoftonline.com/common/oauth2/v2.0/token".to_string(),
        redirect_uri: std::env::var("MICROSOFT_REDIRECT_URI").expect("MICROSOFT_REDIRECT_URI must be set"),
    })
}

pub fn init_http_client(init: httpclient::Client) {
    let _ = SHARED_HTTPCLIENT.set(init);
}

pub fn shared_http_client() -> Cow<'static, httpclient::Client> {
    Cow::Borrowed(SHARED_HTTPCLIENT.get_or_init(default_http_client))
}

#[derive(Clone)]
pub struct FluentRequest<'a, T> {
    pub(crate) client: &'a MicrosoftClient,
    pub params: T,
}

pub enum MicrosoftAuth {
    OAuth2 { middleware: Arc<httpclient_oauth2::OAuth2> },
}

impl MicrosoftAuth {
    pub fn oauth2(
        access: impl Into<String>, refresh: impl Into<String>, callback: Option<Box<dyn Fn(RefreshData) + Send + Sync + 'static>>,
    ) -> Self {
        let mut mw = shared_oauth2_flow().bearer_middleware(access.into(), refresh.into());
        if let Some(cb) = callback {
            mw.callback(cb);
        }
        Self::OAuth2 {
            middleware: Arc::new(mw),
        }
    }
}

pub struct MicrosoftClient {
    client: Cow<'static, httpclient::Client>,
    authentication: MicrosoftAuth,
}

impl MicrosoftClient {
    pub fn with_auth(auth: MicrosoftAuth) -> Self {
        Self {
            client: shared_http_client(),
            authentication: auth,
        }
    }

    pub async fn me(&self) -> InMemoryResult<User> {
        let mut r = self.client.get("/me");
        r = self.authorize(r);
        r.await?.json().map_err(Into::into)
    }

    fn authorize<'a>(&self, mut req: RequestBuilder<'a>) -> RequestBuilder<'a> {
        match &self.authentication {
            MicrosoftAuth::OAuth2 { middleware } => {
                req.middlewares.insert(0, middleware.clone());
            }
        }
        // see https://learn.microsoft.com/en-us/graph/outlook-immutable-id
        req = req.header("Prefer", r#"IdType="ImmutableId""#);
        req
    }
}

#[cfg(test)]
mod tests {}
