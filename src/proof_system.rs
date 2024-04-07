//! The traits that must be implemented by the prover and the verifier.

use halo2_proofs::{
    arithmetic::CurveAffine,
    pasta::group::ff::PrimeField,
    plonk::{ProvingKey, VerifyingKey},
    poly::commitment::Params,
};

use crate::{Bytes, StdResult};

/// A trait for a circuit proving and verification key generator.
pub trait CircuitKeyGenerator<C: CurveAffine> {
    /// Generate the public parameters of the circuit.
    fn generate_setup_params(&self) -> StdResult<Params<C>>;

    /// Generate the proving and verifying keys for the circuit.
    fn generate_keys(&self) -> StdResult<(ProvingKey<C>, VerifyingKey<C>)>;
}

/// A trait for a circuit prover.
pub trait CircuitProver<C: CurveAffine, F: PrimeField>: CircuitKeyGenerator<C> {
    /// Prove the circuit for the public inputs.
    fn prove(self, public_inputs: &[F]) -> StdResult<Bytes>;
}

/// A trait for a circuit verifier.
pub trait CircuitVerifier<C: CurveAffine, F: PrimeField>: CircuitKeyGenerator<C> {
    /// Verify the circuit with a proof and the public inputs.
    fn verify(self, public_inputs: &[F], proof: &Bytes) -> StdResult<()>;
}
