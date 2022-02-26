use clap::Parser;

/// Reverse Polish Notation (RPN) Calculator
#[derive(Parser, Debug)]
#[clap(about, long_about = None)]
pub struct Args {
    /// Formulas written in RPN
    #[clap(name = "FILE")]
    pub filename: Option<String>,
}
