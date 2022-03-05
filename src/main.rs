mod program;

use std::path::PathBuf;
use clap::Parser;
use program::{Program, ProgramSimulationError, Token};

#[derive(Parser, Debug)]
#[clap(author = "Allstreamer")]
#[clap(version = "1.0")]
#[clap(about = "Porth simulator written in rust", long_about = None)]
struct Args {
    input: Option<PathBuf>,
}

fn main() -> Result<(), ProgramSimulationError>{
    //let args = Args::parse();
    let mut p = Program::new(
        vec![
            Token::PUSH(34),
            Token::PUSH(35),
            Token::PLUS,
            Token::DUMP,
            Token::PUSH(500),
            Token::PUSH(80),
            Token::MINUS,
            Token::DUMP
        ]);

    p.simulate_program()?;
    Ok(())
}