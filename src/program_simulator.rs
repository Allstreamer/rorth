use std::string::String;
use thiserror::Error;
use crate::program_simulator::ProgramAction::Jump;
use crate::token::{Token, TokenValue};

#[derive(Error, Debug)]
pub enum ProgramSimulationError {
    #[error("The Stack did not contain enougth Values to perform the Operation")]
    StackFillError {
        file: String,
        token: TokenValue,
        line: usize
    },
    #[error("If did not have an End at runtime")]
    IfHasNoEndAtRuntimeError()
}

pub struct ProgramSimulator {
    stack: Vec<u64>,
    program: Vec<Token>
}

#[derive(Debug)]
pub enum ProgramAction {
    Step,
    Jump(usize)
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
        let mut pos: usize = 0;
        while pos < program.len() {
            let op = &program[pos];

            let action: ProgramAction = match op.token_value {
                TokenValue::PUSH(x) => {
                    self.stack.push(x);
                    ProgramAction::Step
                },
                TokenValue::PLUS => {
                    let a = self.pop_stack(&op)?;
                    let b = self.pop_stack(&op)?;
                    self.stack.push(a+b);
                    ProgramAction::Step
                },
                TokenValue::MINUS => {
                    let a = self.pop_stack(&op)?;
                    let b = self.pop_stack(&op)?;
                    self.stack.push(b-a);
                    ProgramAction::Step
                },
                TokenValue::DUMP => {
                    let value = self.pop_stack(&op)?;
                    println!("{}", value);
                    ProgramAction::Step
                },
                TokenValue::EQUAL => {
                    let a = self.pop_stack(&op)?;
                    let b = self.pop_stack(&op)?;
                    self.stack.push((a == b) as u64);
                    ProgramAction::Step
                },
                TokenValue::IF(jump_positon) => {
                    // Value to check if against
                    let a = self.pop_stack(&op)?;

                    // If the if evaluates to false skip code block
                    if a == 0 {
                        Jump(match jump_positon {
                            Some(v) => v,
                            None => return Err(ProgramSimulationError::IfHasNoEndAtRuntimeError()),
                        } as usize)
                    }else {
                        ProgramAction::Step
                    }
                },
                TokenValue::ELSE(jump_positon) => {
                    ProgramAction::Jump(match jump_positon {
                        Some(v) => v,
                        None => return Err(ProgramSimulationError::IfHasNoEndAtRuntimeError()),
                    } as usize)
                },
                TokenValue::END => {
                    ProgramAction::Step
                },
            };

            match action {
                ProgramAction::Step => pos += 1,
                ProgramAction::Jump(v) => pos = v,
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