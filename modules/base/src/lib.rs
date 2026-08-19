#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(clippy::arithmetic_side_effects)]

pub mod entities;
pub mod rpc;
pub mod services;
pub mod events;
pub mod hooks;
pub mod config;
pub mod utils;