// use std::future::IntoFuture;
// use futures::future::BoxFuture;
// use httpclient::InMemoryResult;
// use serde::{Deserialize, Serialize};
// use crate::{FluentRequest, MicrosoftClient};

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct MessageRef {
//
// }
//
// pub struct ListMessagesRequest {
//
// }

// impl MicrosoftClient {
// pub async fn get_message(&self, id: &str) -> FluentRequest<ListMessagesRequest> {
//     FluentRequest {
//         client: self,
//         params: vec![],
//     }
// }
// }

// impl<'a> IntoFuture for FluentRequest<'a, ListMessagesRequest> {
//     type Output = InMemoryResult<Vec<MessageRef>>;
//     type IntoFuture = BoxFuture<'a, Self::Output>;
//
//     fn into_future(self) -> Self::IntoFuture {
//         Box::pin(async move {
//             let url = &format!("/v1.0/me/messages");
//
//         })
//     }
// }
