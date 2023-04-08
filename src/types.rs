use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RecordAttributes {
    pub url: String,
    pub r#type: String,
}
