use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum FrameworkType {
    OpStack,
    // ArbitrumNitro,
    // Stackr,
    Custom,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FrameworkConfig {
    OpStack(OpStackConfig),
    Custom(CustomConfig),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpStackConfig {
    pub op_config_file_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomConfig {
    pub custom_config_file_path: String,
}
