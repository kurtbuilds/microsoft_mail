use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Page<T> {
    #[serde(rename = "@odata.context")]
    pub context: String,
    #[serde(default, rename = "@odata.nextLink")]
    pub next_link: Option<String>,
    #[serde(rename = "value")]
    pub value: Vec<T>,
}

impl<T> std::ops::Deref for Page<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}