#![deny(unsafe_code)]

pub mod cli;
pub mod config;
pub mod dns;
pub mod error;
pub mod exit_codes;
pub mod output;

pub use error::Error;
