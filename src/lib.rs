#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! The digit sum circuit is an example that provably computes the sum of the digits of a fixed length number.
//!
//! This means that someone who knows the private inputs to the circuit (i.e. the number) can create
//! a succinct zero knowledge proof that its digits sum to a specific value (i.e. public inputs) without
//! having to reveal the number itself. This example is implemented with the [Halo 2](https://github.com/zcash/halo2) proof system.
//!
//! The development of the circuit is split into two modules:
//! - `digitsum` contains the implementation of the digit sum circuit.
//! - `circuit` contains the final circuit that uses one or more chips to implement the desired proof system.
//! - `model` contains the public statement and secret witness types for the digit sum circuit.
//! - `command` contains the command line interface for the digit sum circuit.
//! - `proof_system` contains the proof system implementation for the digit sum circuit.

mod circuit;
mod commands;
mod digitsum;
mod model;
mod proof_system;

pub use circuit::*;
pub use commands::*;
pub use digitsum::*;
pub use model::*;
pub use proof_system::*;
