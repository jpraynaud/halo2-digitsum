use halo2_proofs::plonk::{Advice, Column, Instance, Selector, TableColumn};

/// The configuration for the digit sum circuit
#[derive(Debug, Clone)]
pub struct DigitSumConfig {
    /// Advice columns of the chip
    pub advice: [Column<Advice>; 3],

    /// Instance column of the chip
    pub instance: Column<Instance>,

    /// Sum selector of the chip
    pub s_sum: Selector,

    /// Lookup selector for the digit range check
    pub s_lookup: Selector,

    /// Table column for the digit range check lookup
    pub digit_table: TableColumn,
}
