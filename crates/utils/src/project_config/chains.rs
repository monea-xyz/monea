use serde::{Deserialize, Serialize};

use super::frameworks::{FrameworkConfig, FrameworkType};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChainConfig {
    pub name: String,

    pub chain_id: u64,

    pub block_time: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework_type: Option<FrameworkType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework_config: Option<FrameworkConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub deploy: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement: Option<SettlementConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_availability: Option<DataAvailabilityConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SettlementConfig {
    pub network_id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DataAvailabilityTypes {
    Blobs,
    // Celestia,
    // EigenDA,
    Calldata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataAvailabilityConfig {
    #[serde(rename = "type")]
    pub da_type: DataAvailabilityTypes,
}
