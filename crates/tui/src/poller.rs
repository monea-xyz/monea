use crate::message::Message;
use crate::model::Model;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::time::Duration;

pub fn start_polling(tx: Sender<Message>) {
    thread::spawn(move || {
        loop {
            // Poll for updates (e.g., new transactions, chain info, containers)
            // Send messages to update the model
            // For example:
            // tx.send(Message::TransactionAdded(new_transaction)).unwrap();
            // tx.send(Message::ChainInfoUpdated(new_chain_info)).unwrap();
            // tx.send(Message::ContainersUpdated(new_containers)).unwrap();

            thread::sleep(Duration::from_secs(5));
        }
    });
}
