mod program_simulator;
mod program_parser;
mod token;

use std::path::PathBuf;
use clap::Parser;
use program_simulator::{ProgramSimulator, ProgramSimulationError};
use program_parser::{ProgramParsingError, ProgramParser};
use thiserror::Error;

#[derive(Parser, Debug)]
#[clap(author = "Allstreamer")]
#[clap(version = "1.0")]
#[clap(about = "Porth simulator written in rust", long_about = None)]
struct Args {
    input: Option<PathBuf>,
}

#[derive(Error, Debug)]
pub enum ProgramError {
    #[error("User Forgot to pass a file to the program!")]
    NoFilePassedError(),
    #[error("")]
    SimError {
        #[from]
        source: ProgramSimulationError
    },
    #[error("")]
    ParseError {
        #[from]
        source: ProgramParsingError
    }
}

fn main() -> Result<(), ProgramError>{
    let args: Args = Args::parse();

    let input_program_path = match args.input {
        Some(v) => v,
        None => return Err(ProgramError::NoFilePassedError())
    };
    let tokens =
        ProgramParser::new(input_program_path)
        .load_program()?;

    ProgramSimulator::new(tokens)
        .simulate_program()?;
    Ok(())
}