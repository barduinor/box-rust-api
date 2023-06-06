use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ItemType {
    #[serde(rename = "web_link")]
    WebLink,
    #[serde(rename = "folder")]
    Folder,
    #[serde(rename = "file")]
    File,
    #[serde(rename = "user")]
    User,
}

impl Default for ItemType {
    fn default() -> ItemType {
        Self::File
    }
}
