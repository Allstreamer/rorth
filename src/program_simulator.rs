use std::string::String;
use thiserror::Error;
use crate::token::{Token, TokenValue};

#[derive(Error, Debug)]
pub enum ProgramSimulationError {
    #[error("The Stack did not contain enougth Values to perform the Operation")]
    StackFillError {
        file: String,
        token: TokenValue,
        line: usize
    }
}

pub struct ProgramSimulator {
    stack: Vec<u64>,
    program: Vec<Token>
}

impl ProgramSimulator {
    pub fn new(program: Vec<Token>) -> Self {
        Self {
            program,
            stack: Vec::new()
        }
    }

    pub fn simulate_program(&mut self) -> Result<(), ProgramSimulationError> {
        let program = self.program.clone();
        for op in program {
            match op.token_value {
                TokenValue::PUSH(x) => {
                    self.stack.push(x);
                },
                TokenValue::PLUS => {
                    let a = self.pop_stack(&op)?;
                    let b = self.pop_stack(&op)?;
                    self.stack.push(a+b);
                },
                TokenValue::MINUS => {
                    let a = self.pop_stack(&op)?;
                    let b = self.pop_stack(&op)?;
                    self.stack.push(b-a);
                },
                TokenValue::DUMP => {
                    let value = self.pop_stack(&op)?;
                    println!("{}", value);
                }
            }
        }
        Ok(())
    }

    pub fn pop_stack(&mut self, token: &Token) -> Result<u64, ProgramSimulationError> {
        return match self.stack.pop() {
            Some(v) => Ok(v),
            None => Err(ProgramSimulationError::StackFillError {
                file: token.file.clone(),
                token: token.token_value,
                line: token.line
            }),
        }
    }
}