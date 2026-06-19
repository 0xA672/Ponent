use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ponent", about = "Posita compiler")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Tokenize a Posita source file and print tokens
    Lex {
        /// Input file path
        file: String,
    },
    /// Parse a Posita source file and print the AST (not yet implemented)
    Parse {
        /// Input file path
        file: String,
    },
}
