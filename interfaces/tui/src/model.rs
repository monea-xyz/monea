use crate::components::chain_info::ChainInfo;
use crate::components::transaction_table::Transaction;
use crate::components::urls::Container;

#[derive(Clone, PartialEq, Eq)]
pub enum TabType {
    Baselayer,
    Rollup(String),
    Marketplace,
    Settings,
}

pub struct Tab {
    pub tab_type: TabType,
    pub title: String,
}

pub struct BaseLayerData {
    pub transactions: Vec<Transaction>,
    pub chain_info: ChainInfo,
    pub containers: Vec<Container>,
    pub logs: Vec<String>, // Add more specific log types if needed
}

pub struct RollupData {
    pub transactions: Vec<Transaction>,
    pub chain_info: ChainInfo,
    pub containers: Vec<Container>,
    pub logs: Vec<String>,
    pub rollup_specific_data: String, // Add more rollup-specific fields as needed
}

pub struct MarketplaceModule {
    pub name: String,
    pub description: String,
    pub version: String,
    // Add more metadata fields as needed
}

pub struct Model {
    pub tabs: Vec<Tab>,
    pub current_tab_index: usize,
    pub baselayer_data: Option<BaseLayerData>,
    pub rollup_data: Vec<(String, RollupData)>, // (rollup_name, RollupData)
    pub marketplace_modules: Vec<MarketplaceModule>,
    pub settings: Vec<(String, String)>, // (setting_name, setting_value)
}

impl Model {
    pub fn new() -> Self {
        Model {
            tabs: vec![
                Tab {
                    tab_type: TabType::Marketplace,
                    title: "Marketplace".to_string(),
                },
                Tab {
                    tab_type: TabType::Settings,
                    title: "Settings".to_string(),
                },
            ],
            current_tab_index: 0,
            baselayer_data: None,
            rollup_data: Vec::new(),
            marketplace_modules: Vec::new(),
            settings: vec![
                ("Theme".to_string(), "Dark".to_string()),
                ("Language".to_string(), "English".to_string()),
                // Add more default settings as needed
            ],
        }
    }

    pub fn add_baselayer(&mut self) {
        self.tabs.insert(
            0,
            Tab {
                tab_type: TabType::Baselayer,
                title: "Baselayer".to_string(),
            },
        );
        self.baselayer_data = Some(BaseLayerData {
            transactions: Vec::new(),
            chain_info: ChainInfo::new(
                "Ethereum".to_string(),
                "1".to_string(),
                "ETH".to_string(),
                "18".to_string(),
                "Mainnet".to_string(),
                "L1".to_string(),
                "On-chain".to_string(),
                "EVM".to_string(),
                "L1".to_string(),
                "12 seconds".to_string(),
                "30,000,000".to_string(),
                "15,000,000".to_string(),
            ),
            containers: Vec::new(),
            logs: Vec::new(),
        });
    }

    pub fn add_rollup(&mut self, name: String) {
        self.tabs.insert(
            self.tabs.len() - 2,
            Tab {
                tab_type: TabType::Rollup(name.clone()),
                title: name.clone(),
            },
        );
        self.rollup_data.push((
            name.clone(),
            RollupData {
                transactions: Vec::new(),
                chain_info: ChainInfo::new(
                    name,
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "L2".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "L2".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                ),
                containers: Vec::new(),
                logs: Vec::new(),
                rollup_specific_data: String::new(),
            },
        ));
    }

    pub fn get_tabs(&self) -> &Vec<Tab> {
        &self.tabs
    }

    pub fn active_tab(&self) -> &Tab {
        &self.tabs[self.current_tab_index]
    }

    pub fn active_tab_index(&self) -> usize {
        self.current_tab_index
    }

    pub fn set_current_tab_index(&mut self, index: usize) {
        if index < self.tabs.len() {
            self.current_tab_index = index;
        }
    }

    pub fn update_baselayer_data(&mut self, data: BaseLayerData) {
        self.baselayer_data = Some(data);
    }

    pub fn update_rollup_data(&mut self, name: &str, data: RollupData) {
        if let Some(rollup) = self.rollup_data.iter_mut().find(|(n, _)| n == name) {
            rollup.1 = data;
        }
    }

    pub fn add_marketplace_module(&mut self, module: MarketplaceModule) {
        self.marketplace_modules.push(module);
    }

    pub fn update_setting(&mut self, name: &str, value: String) {
        if let Some(setting) = self.settings.iter_mut().find(|(n, _)| n == name) {
            setting.1 = value;
        } else {
            self.settings.push((name.to_string(), value));
        }
    }
}
