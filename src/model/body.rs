use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum BodyType {
    Text,
    Html,
}

impl fmt::Display for BodyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BodyType::Text => write!(f, "text/plain"),
            BodyType::Html => write!(f, "text/html"),
        }
    }
}

// impl Display for BodyType {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             BodyType::Text => write!(f, "text"),
//             BodyType::Html => write!(f, "html"),
//         }
//     }
// }

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Body {
    pub content_type: BodyType,
    pub content: String,
}
