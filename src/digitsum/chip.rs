use std::marker::PhantomData;

use halo2_proofs::{
    arithmetic::Field,
    circuit::{AssignedCell, Chip, Layouter, Value},
    plonk::{Advice, Column, ConstraintSystem, Error, Instance},
    poly::Rotation,
};

use crate::{DigitSumConfig, DigitSumInstructions, NUMBER_LENGTH};

/// The upper bound (exclusive) for a valid decimal digit
const DIGIT_RANGE_UPPER_BOUND: usize = 10;

/// The chip that implements the digit sum computation instructions
pub struct DigitSumChip<F: Field> {
    config: DigitSumConfig,
    _marker: PhantomData<F>,
}

impl<F: Field> DigitSumChip<F> {
    /// Creates a new instance of the digit sum chip
    pub fn construct(config: DigitSumConfig) -> Self {
        Self {
            config,
            _marker: PhantomData,
        }
    }

    /// Configures the digit sum chip
    pub fn configure(
        meta: &mut ConstraintSystem<F>,
        advice: [Column<Advice>; 3],
        instance: Column<Instance>,
    ) -> <Self as Chip<F>>::Config {
        meta.enable_equality(instance);
        for column in &advice {
            meta.enable_equality(*column);
        }
        let s_sum = meta.selector();
        let s_lookup = meta.complex_selector();
        let digit_table = meta.lookup_table_column();

        // When s_lookup is enabled, the lookup constrains digit values to {0..9}.
        // When s_lookup is disabled, the expression evaluates to 0 which is in the
        // table, so unused rows pass the lookup without additional constraints.
        meta.lookup(|meta| {
            let s_lookup = meta.query_selector(s_lookup);
            let digit = meta.query_advice(advice[0], Rotation::cur());
            vec![(s_lookup * digit, digit_table)]
        });

        meta.create_gate("digit_sum", |meta| {
            // This gate implements the sum of the digits of the provided number in decimal representation
            // Here is the arrangement of the cells of the gate
            //
            // | a0  | a1   | a2   | s_sum | s_lookup |
            // |-----|------|------|-------|----------|
            // | in0 | 0    | sum0 | s_sum | s_lookup |
            // | in1 | sum0 | sum1 | s_sum | s_lookup |
            // | in2 | sum1 | sum2 | s_sum | s_lookup |
            // | in3 | sum2 | sum3 | s_sum | s_lookup |
            // | in4 | sum3 | sum4 | s_sum | s_lookup |
            // | in5 | sum4 | sum5 | s_sum | s_lookup |
            // | in6 | sum5 | sum6 | s_sum | s_lookup |
            // | in7 | sum6 | sum7 | s_sum | s_lookup |
            //
            let input_lhs = meta.query_advice(advice[0], Rotation::cur());
            let input_rhs = meta.query_advice(advice[1], Rotation::cur());
            let output = meta.query_advice(advice[2], Rotation::cur());
            let s_sum = meta.query_selector(s_sum);

            vec![s_sum * (input_lhs + input_rhs - output)]
        });

        DigitSumConfig {
            advice,
            instance,
            s_sum,
            s_lookup,
            digit_table,
        }
    }
}

impl<F: Field> Chip<F> for DigitSumChip<F> {
    type Config = DigitSumConfig;
    type Loaded = ();

    fn config(&self) -> &Self::Config {
        &self.config
    }

    fn loaded(&self) -> &Self::Loaded {
        &()
    }
}

/// A number represented in the chip
#[derive(Clone, Debug)]
pub struct DigitSumNumber<F: Field>(AssignedCell<F, F>);

impl<F: Field> DigitSumInstructions<F> for DigitSumChip<F> {
    type Num = DigitSumNumber<F>;

    fn load_table(&self, mut layouter: impl Layouter<F>) -> Result<(), Error> {
        let config = self.config();

        layouter.assign_table(
            || "digit range table",
            |mut table| {
                let mut value = F::ZERO;
                for digit in 0..DIGIT_RANGE_UPPER_BOUND {
                    table.assign_cell(
                        || format!("digit {digit}"),
                        config.digit_table,
                        digit,
                        || Value::known(value),
                    )?;
                    value += F::ONE;
                }
                Ok(())
            },
        )
    }

    fn load_private(
        &self,
        mut layouter: impl Layouter<F>,
        values: [Value<F>; NUMBER_LENGTH],
    ) -> Result<Self::Num, Error> {
        let config = self.config();

        layouter
            .assign_region(
                || "digits sum",
                |mut region| {
                    let mut previous_value = region.assign_advice(
                        || "zero",
                        config.advice[1],
                        0,
                        || Value::known(F::ZERO),
                    )?;
                    for (i, value) in values.into_iter().enumerate() {
                        config.s_sum.enable(&mut region, i)?;
                        config.s_lookup.enable(&mut region, i)?;

                        // First advice column of ith row is the ith witness digit
                        region
                            .assign_advice(
                                || format!("witness {}", i),
                                config.advice[0],
                                i,
                                || value,
                            )
                            .map(DigitSumNumber)?;

                        // Second advice column of ith row is the sum of the first i-1 digits
                        if i > 0 {
                            previous_value.copy_advice(
                                || format!("digit sum[{}]", i - 1),
                                &mut region,
                                config.advice[1],
                                i,
                            )?;
                        }

                        // Third advice column of ith row is the sum of the first i digits
                        previous_value = region.assign_advice(
                            || format!("digit sum [{i}]"),
                            config.advice[2],
                            i,
                            || value + previous_value.value(),
                        )?;
                    }

                    Ok(previous_value)
                },
            )
            .map(DigitSumNumber)
    }

    fn expose_public(
        &self,
        mut layouter: impl Layouter<F>,
        num: Self::Num,
        row: usize,
    ) -> Result<(), Error> {
        let config = self.config();

        layouter.constrain_instance(num.0.cell(), config.instance, row)
    }
}
