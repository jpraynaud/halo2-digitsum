use clap::Parser;
use halo2_proofs::pasta::Fp;
use std::{fs::File, io::Write, path::PathBuf};

use crate::{CircuitProver, DigitSumCircuit, StdResult};

#[derive(Parser, Debug, Clone)]
pub struct ProveCommand {
    /// Secret number that Alice knows (a.k.a. the witness).
    #[clap(long, short = 'w')]
    witness: u64,

    /// Public number that Bob knows and which represents the sum of the digits of the witness (a.k.a. the statement).
    #[clap(long, short = 's')]
    statement: u64,

    /// Proof export filename.
    #[clap(long, default_value = "proof.hex")]
    proof_file_name: PathBuf,

    /// Proof export directory.
    #[clap(long, default_value = "./")]
    proof_export_dir: PathBuf,
}

impl ProveCommand {
    /// Main command execution
    pub fn execute(&self) -> StdResult<()> {
        let secret_witness_number = self.witness;
        let circuit = DigitSumCircuit::<Fp>::new(secret_witness_number)?;
        let proof = circuit.prove(&[self.statement.into()])?;

        let proof_hex = hex::encode(proof);
        let proof_export_path = self.proof_export_dir.join(&self.proof_file_name);
        let mut proof_file = File::create(&proof_export_path)?;
        write!(proof_file, "{proof_hex}")?;
        println!(">> Proof generated to {:?}", proof_export_path);

        Ok(())
    }
}
