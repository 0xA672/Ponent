mod cli;
mod lexer;

use clap::Parser;
use logos::Logos;
use std::fs;

fn main() {
    let cli = cli::Cli::parse();
    match cli.command {
        cli::Command::Lex { file } => {
            let source = fs::read_to_string(&file).expect("failed to read file");
            let lexer = lexer::Token::lexer(&source);
            for result in lexer {
                match result {
                    Ok(token) => {
                        if token != lexer::Token::WhitespaceOrComment {
                            println!("{:?}", token);
                        }
                    }
                    Err(()) => eprintln!("ERROR: invalid token"),
                }
            }
        }
        cli::Command::Parse { file: _ } => {
            eprintln!("Parse command is not yet implemented.");
        }
    }
}
