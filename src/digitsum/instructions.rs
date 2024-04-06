use halo2_proofs::{
    arithmetic::Field,
    circuit::{Chip, Layouter, Value},
    plonk::Error,
};

use crate::NUMBER_LENGTH;

/// Traits for the chip that computes digit sum
pub trait DigitSumInstructions<F: Field>: Chip<F> {
    /// The number type associated to the digit sum instruction
    type Num;

    /// Loads a private input to the circuit in a decimal format.
    fn load_private(
        &self,
        layouter: impl Layouter<F>,
        values: [Value<F>; NUMBER_LENGTH],
    ) -> Result<Vec<Self::Num>, Error>;

    /// Sums the digits of a number
    fn compute_digit_sum(
        &self,
        layouter: impl Layouter<F>,
        values: [Self::Num; NUMBER_LENGTH],
    ) -> Result<Self::Num, Error>;

    /// Exposes a number as a public input to the circuit.
    fn expose_public(
        &self,
        layouter: impl Layouter<F>,
        num: Self::Num,
        row: usize,
    ) -> Result<(), Error>;
}
