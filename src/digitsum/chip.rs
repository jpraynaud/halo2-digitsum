use std::marker::PhantomData;

use halo2_proofs::{
    arithmetic::Field,
    circuit::{AssignedCell, Chip, Layouter, Region, Value},
    plonk::{Advice, Column, ConstraintSystem, Error, Expression, Instance},
    poly::Rotation,
};

use crate::{DigitSumConfig, DigitSumInstructions, NUMBER_LENGTH};

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
        advice: [Column<Advice>; NUMBER_LENGTH],
        instance: Column<Instance>,
    ) -> <Self as Chip<F>>::Config {
        meta.enable_equality(instance);
        for column in &advice {
            meta.enable_equality(*column);
        }
        let s_sum = meta.selector();
        meta.create_gate("digit_sum", |meta| {
            // This gate implements the sum of the digits of the provided number in decimal representation
            // Here is the arrangement of the cells of the gate
            //
            // | a0  | a1  | a2  | a3  | a4  | a5  | a6  | a7  | s_sum |
            // |-----|-----|-----|-----|-----|-----|-----|-----|-------|
            // | in0 | in1 | in2 | in3 | in4 | in5 | in6 | in7 | s_sum |
            // | out |     |     |     |     |     |     |     |       |
            //
            let inputs = advice
                .iter()
                .map(|adv| meta.query_advice(*adv, Rotation::cur()))
                .collect::<Vec<_>>();
            let sum_inputs = inputs
                .into_iter()
                .fold(Expression::Constant(F::ZERO), |acc, input| acc + input);
            let output = meta.query_advice(advice[0], Rotation::next());
            let s_sum = meta.query_selector(s_sum);

            vec![s_sum * (sum_inputs - output)]
        });

        DigitSumConfig {
            advice,
            instance,
            s_sum,
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

/// A number in the represented in the chip
#[derive(Clone, Debug)]
pub struct DigitSumNumber<F: Field>(AssignedCell<F, F>);

impl<F: Field> DigitSumInstructions<F> for DigitSumChip<F> {
    type Num = DigitSumNumber<F>;

    fn load_private(
        &self,
        mut layouter: impl Layouter<F>,
        values: [Value<F>; NUMBER_LENGTH],
    ) -> Result<Vec<Self::Num>, Error> {
        let config = self.config();
        // TODO: verify that the number is in decimal representation
        layouter.assign_region(
            || "assign private value",
            |mut region| {
                values
                    .iter()
                    .enumerate()
                    .map(|(i, value)| {
                        let cell = region.assign_advice(
                            || format!("load private value {}", i),
                            config.advice[i],
                            i,
                            || *value,
                        )?;
                        Ok(DigitSumNumber(cell))
                    })
                    .collect()
            },
        )
    }

    fn compute_digit_sum(
        &self,
        mut layouter: impl Layouter<F>,
        values: [Self::Num; NUMBER_LENGTH],
    ) -> Result<Self::Num, Error> {
        let config = self.config();

        layouter.assign_region(
            || "sum digits",
            |mut region: Region<'_, F>| {
                config.s_sum.enable(&mut region, 0)?;

                for (i, value) in values.iter().enumerate() {
                    value.0.copy_advice(
                        || format!("value[{i}]"),
                        &mut region,
                        config.advice[i],
                        0,
                    )?;
                }

                let sum_value = values
                    .iter()
                    .map(|v| v.0.value().copied())
                    .fold(Value::known(F::ZERO), |acc, e| acc + e);

                region
                    .assign_advice(|| "sum digits", config.advice[0], 1, || sum_value)
                    .map(DigitSumNumber)
            },
        )
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
