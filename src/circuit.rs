//! The final circuit that uses one or more chips to implement the desired proof system.

use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    pasta::{group::ff::PrimeField, EqAffine, Fp},
    plonk::{
        create_proof, keygen_pk, keygen_vk, verify_proof, Circuit, ConstraintSystem, Error,
        ProvingKey, SingleVerifier, VerifyingKey,
    },
    poly::commitment::Params,
    transcript::{Blake2bRead, Blake2bWrite, Challenge255},
};
use rand_core::OsRng;

use crate::{
    Bytes, CircuitKeyGenerator, CircuitProver, CircuitVerifier, DigitSumChip, DigitSumConfig,
    DigitSumInstructions, DigitSumSecretWitness, StdResult, NUMBER_LENGTH,
};

/// The size parameter of the circuit: the circuit must fit into 2^k rows.
const DIGIT_SUM_CIRCUIT_SIZE_PARAMETER: u32 = 5;

/// The circuit implementation for digit sum
pub struct DigitSumCircuit<F: PrimeField> {
    /// The number with which to compute the digit sum in decimal representation
    pub number: [Value<F>; NUMBER_LENGTH],

    /// The size parameter of the circuit: the circuit must fit into 2^k rows.
    pub k: u32,
}

impl<F: PrimeField> Default for DigitSumCircuit<F> {
    fn default() -> Self {
        Self::new(0).unwrap()
    }
}

impl<F: PrimeField> DigitSumCircuit<F> {
    /// Creates a new digit sum circuit
    pub fn new(number: u64) -> StdResult<Self> {
        let k = DIGIT_SUM_CIRCUIT_SIZE_PARAMETER;
        let secret_witness_number = DigitSumSecretWitness::<F>::new(number);
        let number: [Value<F>; NUMBER_LENGTH] = secret_witness_number.try_into().unwrap();

        Ok(Self { number, k })
    }
}

impl<F: PrimeField> Circuit<F> for DigitSumCircuit<F> {
    type Config = DigitSumConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        let advice = (0..3).map(|_| meta.advice_column()).collect::<Vec<_>>();
        let instance = meta.instance_column();
        DigitSumChip::configure(meta, advice.try_into().unwrap(), instance)
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<F>,
    ) -> Result<(), Error> {
        let chip = DigitSumChip::construct(config);

        let sum = chip.load_private(layouter.namespace(|| "private number"), self.number)?;

        chip.expose_public(layouter.namespace(|| "expose digit sum"), sum, 0)
    }
}

impl CircuitKeyGenerator<EqAffine> for DigitSumCircuit<Fp> {
    fn generate_setup_params(&self) -> StdResult<Params<EqAffine>> {
        Ok(Params::<EqAffine>::new(self.k))
    }

    fn generate_keys(&self) -> StdResult<(ProvingKey<EqAffine>, VerifyingKey<EqAffine>)> {
        let params = self.generate_setup_params()?;
        let vk = keygen_vk(&params, self)?;
        let pk = keygen_pk(&params, vk.clone(), self)?;

        Ok((pk, vk))
    }
}

impl CircuitProver<EqAffine, Fp> for DigitSumCircuit<Fp> {
    fn prove(self, public_inputs: &[Fp]) -> StdResult<Bytes> {
        let params = self.generate_setup_params()?;
        let (pk, _) = self.generate_keys()?;
        let mut transcript = Blake2bWrite::<_, _, Challenge255<_>>::init(vec![]);
        create_proof(
            &params,
            &pk,
            &[self],
            &[&[public_inputs]],
            OsRng,
            &mut transcript,
        )?;

        Ok(transcript.finalize())
    }
}

impl CircuitVerifier<EqAffine, Fp> for DigitSumCircuit<Fp> {
    fn verify(self, public_inputs: &[Fp], proof: &Bytes) -> StdResult<()> {
        let params = self.generate_setup_params()?;
        let (_, vk) = self.generate_keys()?;
        let strategy = SingleVerifier::new(&params);
        let mut transcript = Blake2bRead::<_, _, Challenge255<_>>::init(proof.as_slice());
        verify_proof(&params, &vk, strategy, &[&[public_inputs]], &mut transcript)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use halo2_proofs::{dev::MockProver, pasta::Fp};

    use crate::DigitSumPublicInput;

    use super::*;

    #[test]
    fn test_digit_sum_circuit_proof_succeeds_if_valid_public_input() {
        let secret_witness_number = 12340000;
        let public_input_digitsum = 10;

        let circuit = DigitSumCircuit::<Fp>::new(secret_witness_number).unwrap();
        let prover = MockProver::run(
            circuit.k,
            &circuit,
            vec![vec![DigitSumPublicInput::new(public_input_digitsum).into()]],
        )
        .unwrap();

        prover.verify().expect("the proof should be valid");
    }

    #[test]
    fn test_digit_sum_circuit_proof_fails_if_invalid_public_input() {
        let secret_witness_number = 10000000;
        let public_input_digitsum = 2;

        let circuit = DigitSumCircuit::<Fp>::new(secret_witness_number).unwrap();
        let prover = MockProver::run(
            circuit.k,
            &circuit,
            vec![vec![DigitSumPublicInput::new(public_input_digitsum).into()]],
        )
        .unwrap();

        prover.verify().expect_err("the proof should be invalid");
    }
}
