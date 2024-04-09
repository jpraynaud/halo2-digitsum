use std::{fs::read, path::PathBuf};

use clap::Parser;
use halo2_proofs::pasta::Fp;

use crate::{CircuitVerifier, DigitSumCircuit, StdResult};

#[derive(Parser, Debug, Clone)]
pub struct VerifyCommand {
    /// Public number that Bob knows and which represents the sum of the digits of the witness (a.k.a. the statement).
    #[clap(long, short = 's')]
    statement: u64,

    /// Proof import filename.
    #[clap(long, default_value = "proof.hex")]
    proof_file_name: PathBuf,

    /// Proof import directory.
    #[clap(long, default_value = "./")]
    proof_import_dir: PathBuf,
}

impl VerifyCommand {
    /// Main command execution
    pub fn execute(&self) -> StdResult<()> {
        let proof_import_path = self.proof_import_dir.join(&self.proof_file_name);
        let proof = read(proof_import_path)?;
        let proof = hex::decode(proof)?;

        let circuit = DigitSumCircuit::<Fp>::default();
        circuit.verify(&[self.statement.into()], &proof)?;
        println!(">> Proof verified!");

        Ok(())
    }
}
