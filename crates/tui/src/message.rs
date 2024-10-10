use crate::components::chain_info::ChainInfo;
use crate::components::transaction_table::Transaction;
use crate::components::urls::Container;
use crate::update::MenuItemChange;

pub enum Message {
    Tick,
    TabChanged(MenuItemChange),
    TransactionAdded(Transaction),
    ChainInfoUpdated(ChainInfo),
    ContainersUpdated(Vec<Container>),
    AddRollup(String),
    AddBaseLayer,
}
