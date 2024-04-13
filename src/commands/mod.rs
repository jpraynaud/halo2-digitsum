//! The command module contains the command line interface for the digit sum circuit:
//! - `prove` generates a proof for a given number (witness) and digit sum (public input)
//! - `verify` verifies a proof for a given number (witness) and digit sum (public input)
//! - `graph` generates a graph representation of the circuit

mod graph;
mod prove;
mod verify;

use clap::{Parser, Subcommand};
use slog::Level;

use crate::StdResult;

use self::{graph::GraphCommand, prove::ProveCommand, verify::VerifyCommand};

/// The main CLI arguments
#[derive(Parser, Debug, Clone)]
#[clap(name = "digitsum")]
#[clap(
about = "This program proves and verifies the computation of the sum of the digits of a number.",
long_about = None
)]
#[command(version)]
pub struct Args {
    /// Available commands
    #[clap(subcommand)]
    command: CliCommands,

    /// Verbosity level (-v=warning, -vv=info, -vvv=debug).
    #[clap(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

impl Args {
    /// Execute command
    pub fn execute(&self) -> StdResult<()> {
        self.command.execute()
    }

    /// Log level from command line arguments
    pub fn log_level(&self) -> Level {
        match self.verbose {
            0 => Level::Error,
            1 => Level::Warning,
            2 => Level::Info,
            3 => Level::Debug,
            _ => Level::Trace,
        }
    }
}

/// The available sub-commands of the CLI
#[derive(Subcommand, Debug, Clone)]
pub enum CliCommands {
    /// Run the prover for the digit sum circuit
    #[clap(arg_required_else_help = true)]
    Prove(ProveCommand),

    /// Run the verifier for the digit sum circuit
    #[clap(arg_required_else_help = true)]
    Verify(VerifyCommand),

    /// Run the graph exporter for the digit sum circuit
    #[clap(arg_required_else_help = false)]
    Graph(GraphCommand),
}

impl CliCommands {
    /// Execute the command
    pub fn execute(&self) -> StdResult<()> {
        match self {
            Self::Prove(cmd) => cmd.execute(),
            Self::Verify(cmd) => cmd.execute(),
            Self::Graph(cmd) => cmd.execute(),
        }
    }
}
