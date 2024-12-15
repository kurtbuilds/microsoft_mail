use serde::{Deserialize, Serialize};
use ::email::EmailAddress;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Recipient {
    pub email_address: EmailAddress,
}

impl From<EmailAddress> for Recipient {
    fn from(email_address: EmailAddress) -> Self {
        Recipient {
            email_address,
        }
    }
}