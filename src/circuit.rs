//! The final circuit that uses one or more chips to implement the desired proof system.

use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    pasta::group::ff::PrimeField,
    plonk::{Circuit, ConstraintSystem, Error},
};

use crate::{
    DigitSumChip, DigitSumConfig, DigitSumInstructions, DigitSumSecretWitness, StdResult,
    NUMBER_LENGTH,
};

/// The circuit implementation for digit sum
#[derive(Default)]
pub struct DigitSumCircuit<F: PrimeField> {
    /// The number with which to compute the digit sum in decimal representation
    pub number: [Value<F>; NUMBER_LENGTH],
}

impl<F: PrimeField> DigitSumCircuit<F> {
    /// Creates a new digit sum circuit
    pub fn new(number: u64) -> StdResult<Self> {
        let secret_witness_number = DigitSumSecretWitness::<F>::new(number);
        let number: [Value<F>; NUMBER_LENGTH] = secret_witness_number.try_into().unwrap();

        Ok(Self { number })
    }
}

impl<F: PrimeField> Circuit<F> for DigitSumCircuit<F> {
    type Config = DigitSumConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        let advice = (0..NUMBER_LENGTH)
            .map(|_| meta.advice_column())
            .collect::<Vec<_>>();
        let instance = meta.instance_column();
        DigitSumChip::configure(meta, advice.try_into().unwrap(), instance)
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<F>,
    ) -> Result<(), Error> {
        let chip = DigitSumChip::construct(config);

        let private_number =
            chip.load_private(layouter.namespace(|| "private number"), self.number)?;

        let sum = chip.compute_digit_sum(
            layouter.namespace(|| "digit sum"),
            private_number.try_into().unwrap(),
        )?;

        chip.expose_public(layouter.namespace(|| "expose digit sum"), sum, 0)
    }
}

#[cfg(test)]
mod tests {
    use halo2_proofs::{dev::MockProver, pasta::Fp};

    use crate::{DigitSumPublicStatement, DigitSumSecretWitness};

    use super::*;

    #[test]
    fn test_digit_sum_circuit_proof_succeeds_if_valid_statement() {
        let secret_witness_number = DigitSumSecretWitness::<Fp>::new(12340000);
        let public_statement_digitsum = DigitSumPublicStatement::new(10);

        let k = 4;
        let circuit = DigitSumCircuit {
            number: secret_witness_number.try_into().unwrap(),
        };
        let prover =
            MockProver::run(k, &circuit, vec![vec![public_statement_digitsum.into()]]).unwrap();

        prover.verify().expect("the proof should be valid");
    }

    #[test]
    fn test_digit_sum_circuit_proof_fails_if_invalid_statement() {
        let secret_witness_number = DigitSumSecretWitness::<Fp>::new(10000000);
        let public_statement_digitsum = DigitSumPublicStatement::new(2);

        let k = 4;
        let circuit = DigitSumCircuit {
            number: secret_witness_number.try_into().unwrap(),
        };
        let prover =
            MockProver::run(k, &circuit, vec![vec![public_statement_digitsum.into()]]).unwrap();

        prover.verify().expect_err("the proof should be invalid");
    }
}
