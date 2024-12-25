pub mod api;
pub mod config;
pub mod error;
pub mod event;
pub mod gui;
pub mod network;
pub mod state;
pub mod utils;

pub enum ControlMessage {
    Start,
    Stop,
}

pub const VERSION: &str = env!("CARGO_PKG_VERSION");