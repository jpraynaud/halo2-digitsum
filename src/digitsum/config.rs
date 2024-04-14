use halo2_proofs::plonk::{Advice, Column, Instance, Selector};

/// The configuration for the digit sum circuit
#[derive(Debug, Clone)]
pub struct DigitSumConfig {
    /// Advice columns of the chip
    pub advice: [Column<Advice>; 3],

    /// Instance column of the chip
    pub instance: Column<Instance>,

    /// Sum selector of the chip
    pub s_sum: Selector,
}
