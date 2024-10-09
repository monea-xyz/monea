use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PipelineConfig {
    pub pre: Option<Vec<String>>,
    pub post: Option<Vec<String>>,
}
