//! The digit sum chip that provably computes the sum of the decimal digits of a fixed length number
//! The module is splitted into three files:
//! - `chip.rs` contains the implementation of the chip that computes the digit sum
//! - `config.rs` contains the configuration for the digit sum circuit
//! - `instructions.rs` contains the instructions interface for the digit sum chip

mod chip;
mod config;
mod instructions;

pub use chip::*;
pub use config::*;
pub use instructions::*;

/// The number of digits in the input number
pub const NUMBER_LENGTH: usize = 8;
