use crate::components::chain_info::ChainInfo;
use crate::message::Message;
use crate::model::{BaseLayerData, MarketplaceModule, Model, RollupData, Tab, TabType};

pub enum MenuItemChange {
    Index(usize),
    Increment,
    Decrement,
}

impl From<usize> for MenuItemChange {
    fn from(index: usize) -> Self {
        MenuItemChange::Index(index)
    }
}

pub fn update(model: &mut Model, message: Message) {
    match message {
        Message::Tick => {
            // Update any time-based state
        }
        Message::TabChanged(param) => {
            change_active_tab(model, param);
        }
        Message::AddBaseLayer => {
            add_baselayer(model);
        }
        Message::AddRollup(name) => {
            add_rollup(model, name);
        }
        Message::TransactionAdded(transaction) => todo!(),
        Message::ChainInfoUpdated(chain_info) => todo!(),
        Message::ContainersUpdated(vec) => todo!(),
        // Message::UpdateBaseLayerData(data) => {
        //     update_baselayer_data(model, data);
        // }
        // Message::UpdateRollupData(name, data) => {
        //     update_rollup_data(model, name, data);
        // }
        // Message::AddMarketplaceModule(module) => {
        //     add_marketplace_module(model, module);
        // }
        // Message::UpdateSetting(name, value) => {
        //     update_setting(model, name, value);
        // }
    }
}

fn change_active_tab(model: &mut Model, param: impl Into<MenuItemChange>) {
    match param.into() {
        MenuItemChange::Index(index) => {
            model.current_tab_index = index.saturating_sub(1);
        }
        MenuItemChange::Increment => {
            model.current_tab_index = (model.current_tab_index + 1) % model.tabs.len();
        }
        MenuItemChange::Decrement => {
            model.current_tab_index =
                (model.current_tab_index + model.tabs.len() - 1) % model.tabs.len();
        }
    }
}

fn add_baselayer(model: &mut Model) {
    model.tabs.insert(
        0,
        Tab {
            tab_type: TabType::Baselayer,
            title: "Baselayer".to_string(),
        },
    );
    model.baselayer_data = Some(BaseLayerData {
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

fn add_rollup(model: &mut Model, name: String) {
    model.tabs.insert(
        model.tabs.len() - 2,
        Tab {
            tab_type: TabType::Rollup(name.clone()),
            title: name.clone(),
        },
    );
    model.rollup_data.push((
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
