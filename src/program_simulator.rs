use thiserror::Error;

#[derive(Debug, Copy, Clone)]
pub enum Token {
    PUSH(u64),
    PLUS,
    MINUS,
    DUMP,
}

#[derive(Error, Debug)]
pub enum ProgramSimulationError {
    #[error("The Stack did not contain enougth Values to perform the Operation")]
    StackFillError()
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
            match op {
                Token::PUSH(x) => {
                    self.stack.push(x);
                },
                Token::PLUS => {
                    let a = self.pop_stack()?;
                    let b = self.pop_stack()?;
                    self.stack.push(a+b);
                },
                Token::MINUS => {
                    let a = self.pop_stack()?;
                    let b = self.pop_stack()?;
                    self.stack.push(b-a);
                },
                Token::DUMP => {
                    let value = self.pop_stack()?;
                    println!("{}", value);
                }
            }
        }
        Ok(())
    }

    pub fn pop_stack(&mut self) -> Result<u64, ProgramSimulationError> {
        return match self.stack.pop() {
            Some(v) => Ok(v),
            None => Err(ProgramSimulationError::StackFillError()),
        }
    }
}