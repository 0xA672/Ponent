mod ast;
mod cli;
mod lexer;
mod parser;

use clap::Parser;
use logos::Logos;
use std::fs;
use std::process;

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
        cli::Command::Parse { file, ast } => {
            let source = fs::read_to_string(&file).expect("failed to read file");
            let mut parser = parser::Parser::new(&source);
            match parser.parse_program() {
                Ok(program) => {
                    if ast {
                        println!("{:#?}", program);
                    } else {
                        println!("Parsed successfully.");
                    }
                }
                Err(diagnostics) => {
                    for diag in diagnostics {
                        eprintln!("error at {}: {}", diag.span, diag.message);
                    }
                    process::exit(1);
                }
            }
        }
    }
}
