use clap::Parser;
use halo2_proofs::pasta::Fp;
use plotters::prelude::*;
use std::path::PathBuf;

use crate::{DigitSumCircuit, StdResult};

#[derive(Parser, Debug, Clone)]
pub struct GraphCommand {
    /// Circuit layout export filename.
    #[clap(long, default_value = "circuit-layout.png")]
    graph_file_name: PathBuf,

    /// Circuit layout export directory.
    #[clap(long, default_value = "./")]
    graph_export_dir: PathBuf,

    /// Circuit layout with labels.
    #[clap(long, default_value = "true")]
    graph_with_labels: bool,

    /// Circuit layout width.
    #[clap(long, default_value = "1024")]
    graph_width: u32,

    /// Circuit layout height.
    #[clap(long, default_value = "768")]
    graph_height: u32,
}

impl GraphCommand {
    /// Main command execution
    pub fn execute(&self) -> StdResult<()> {
        let circuit = DigitSumCircuit::<Fp>::default();

        let graph_layout_title = "Digit Sum Circuit";
        let graph_layout_dimensions = (self.graph_width, self.graph_height);
        let graph_export_path = self.graph_export_dir.join(&self.graph_file_name);
        let root =
            BitMapBackend::new(&graph_export_path, graph_layout_dimensions).into_drawing_area();
        root.fill(&WHITE)?;
        let root = root.titled(graph_layout_title, ("sans-serif", 30))?;

        halo2_proofs::dev::CircuitLayout::default()
            .show_labels(self.graph_with_labels)
            .render(circuit.k, &circuit, &root)?;
        println!(">> Circuit layout generated to {:?}", graph_export_path);

        Ok(())
    }
}
