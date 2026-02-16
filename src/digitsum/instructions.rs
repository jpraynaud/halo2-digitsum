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

    /// Loads the digit range lookup table into the circuit.
    fn load_table(&self, layouter: impl Layouter<F>) -> Result<(), Error>;

    /// Loads a private input to the circuit in a decimal format.
    fn load_private(
        &self,
        layouter: impl Layouter<F>,
        values: [Value<F>; NUMBER_LENGTH],
    ) -> Result<Self::Num, Error>;

    /// Exposes a number as a public input to the circuit.
    fn expose_public(
        &self,
        layouter: impl Layouter<F>,
        num: Self::Num,
        row: usize,
    ) -> Result<(), Error>;
}
