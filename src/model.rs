use std::marker::PhantomData;

use anyhow::anyhow;
use halo2_proofs::{
    arithmetic::Field,
    circuit::Value,
    pasta::{group::ff::PrimeField, Fp},
};

use crate::NUMBER_LENGTH;

/// Generic error type
pub type StdError = anyhow::Error;

/// Generic result type
pub type StdResult<T> = anyhow::Result<T, StdError>;

/// The secret witness for the digit sum circuit
#[derive(Clone, Debug)]
pub struct DigitSumSecretWitness<F: Field> {
    number: u64,
    _marker: PhantomData<F>,
}

impl<F: Field> DigitSumSecretWitness<F> {
    /// Creates a new secret witness
    pub fn new(number: u64) -> Self {
        let _marker = PhantomData;
        Self { number, _marker }
    }
}

impl<F: Field> TryFrom<DigitSumSecretWitness<F>> for [u64; NUMBER_LENGTH] {
    type Error = StdError;

    fn try_from(other: DigitSumSecretWitness<F>) -> Result<[u64; NUMBER_LENGTH], Self::Error> {
        let values = other
            .number
            .to_string()
            .chars()
            .map(|d| d.to_digit(10).unwrap() as u64) // safe unwrap
            .collect::<Vec<_>>();
        if values.len() > NUMBER_LENGTH {
            return Err(anyhow!(
                "Number is too long. Expected {} digits, got {}.",
                NUMBER_LENGTH,
                values.len()
            ));
        }
        let padded_values = std::iter::repeat(0_u64)
            .take(NUMBER_LENGTH - values.len())
            .chain(values)
            .collect::<Vec<_>>();

        padded_values.try_into().map_err(|_| {
            anyhow!(format!(
                "Failed to convert witness to {NUMBER_LENGTH} digits"
            ))
        })
    }
}

impl<F: PrimeField> TryFrom<DigitSumSecretWitness<F>> for [Value<F>; NUMBER_LENGTH] {
    type Error = StdError;

    fn try_from(other: DigitSumSecretWitness<F>) -> Result<[Value<F>; NUMBER_LENGTH], Self::Error> {
        let values_u64: [u64; NUMBER_LENGTH] = other.try_into()?;
        values_u64
            .into_iter()
            .map(|v| Value::known(F::from(v)))
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|_| {
                anyhow!(format!(
                    "Failed to convert witness to {NUMBER_LENGTH} digits"
                ))
            })
    }
}

/// The public statement for the digit sum circuit
#[derive(Clone, Debug)]
pub struct DigitSumPublicStatement {
    number: u64,
}

impl DigitSumPublicStatement {
    /// Creates a new public statement
    pub fn new(number: u64) -> Self {
        Self { number }
    }
}

impl From<DigitSumPublicStatement> for Fp {
    fn from(other: DigitSumPublicStatement) -> Fp {
        Fp::from(other.number)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn secret_witness_should_convert_to_array_with_valid_number_exact_digits() {
        let secret_number = 12345678;
        assert!(secret_number.to_string().len() == NUMBER_LENGTH);
        let secret_witness = DigitSumSecretWitness::new(secret_number);
        let expected_known_values = [1, 2, 3, 4, 5, 6, 7, 8];
        let known_values: [u64; NUMBER_LENGTH] = secret_witness.clone().try_into().unwrap();

        assert_eq!(expected_known_values, known_values);
        let _known_values: [Value<Fp>; NUMBER_LENGTH] = secret_witness.try_into().unwrap();
    }

    #[test]
    fn secret_witness_should_convert_to_array_with_valid_number_less_digits() {
        let secret_number = 123456;
        assert!(secret_number.to_string().len() < NUMBER_LENGTH);
        let secret_witness = DigitSumSecretWitness::new(secret_number);
        let expected_known_values = [0, 0, 1, 2, 3, 4, 5, 6];
        let known_values: [u64; NUMBER_LENGTH] = secret_witness.clone().try_into().unwrap();

        assert_eq!(expected_known_values, known_values);
        let _known_values: [Value<Fp>; NUMBER_LENGTH] = secret_witness.try_into().unwrap();
    }

    #[test]
    fn secret_witness_should_convert_to_array_with_valid_number_more_digits() {
        let secret_number = 123456789;
        assert!(secret_number.to_string().len() > NUMBER_LENGTH);
        let secret_witness = DigitSumSecretWitness::new(secret_number);
        let known_values: Result<[Value<Fp>; NUMBER_LENGTH], _> = secret_witness.try_into();

        assert!(known_values.is_err());
    }
}
