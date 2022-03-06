use std::fs::File;
use std::{io, io::{BufRead, BufReader}};
use std::num::ParseIntError;
use std::path::PathBuf;
use std::str::FromStr;
use std::string::String;
use thiserror::Error;
use crate::token::{Token, TokenValue};

#[derive(Error, Debug)]
pub enum ProgramParsingError {
    #[error("Failed to Read File!")]
    Io {
        #[from]
        source: io::Error
    },
    #[error("Integer Value invalid!")]
    IntError {
        #[from]
        source: ParseIntError
    },
    #[error("Unknown token!")]
    InvalidToken(String, usize),

    #[error("Couldn't parse path")]
    InvalidFilePath(),
    #[error("Couldn't parse path")]
    IfMissingEnd(),
    #[error("If can only match else & end")]
    IfMatchError(),
}

pub struct ProgramParser {
    path: PathBuf
}

impl ProgramParser {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path
        }
    }

    pub fn load_program(&self) -> Result<Vec<Token>, ProgramParsingError> {
        let reader = BufReader::new(
            File::open(self.path.clone())?
        );
        Ok(self.cross_reference_blocks(self.lex_program(reader)?)?)
    }

    /// Convert BufReader into Tokens
    pub fn lex_program(&self, reader: BufReader<File>) -> Result<Vec<Token>, ProgramParsingError> {
        let mut tokens: Vec<Token> = Vec::new();
        for (index,line) in reader.lines().enumerate() {
            for word in line?.split_whitespace() {
                if word.starts_with("//") {
                    break;
                }
                let token = match self.word_to_token(word) {
                    Ok(v) => v,
                    Err(e) => {
                        return match e {
                            ProgramParsingError::InvalidToken(_, _) => {
                                Err(ProgramParsingError::InvalidToken(
                                    String::from(word),
                                    index+1))
                            }
                            _ => {
                                Err(e)
                            }
                        }
                    }
                };
                let file_path = match self.path.to_str() {
                    Some(v) => {
                        String::from(v)
                    },
                    None => {
                        return Err(ProgramParsingError::InvalidFilePath());
                    }
                };
                tokens.push(Token {
                    token_value: token,
                    file: file_path,
                    line: 0
                });
            }
        }

        Ok(tokens)
    }

    /// Fill out If values
    pub fn cross_reference_blocks(&self, mut program: Vec<Token>) -> Result<Vec<Token>, ProgramParsingError> {
        let mut stack: Vec<usize> = Vec::new();

        for pos in 0..program.len() {
            let op = &program[pos];
            match op.token_value {
                TokenValue::IF(_v) => {
                    stack.push(pos);
                },
                TokenValue::ELSE(_v) => {
                    let if_pos = match stack.pop() {
                        Some(v) => v,
                        None => {return Err(ProgramParsingError::IfMissingEnd())}
                    };
                    program[if_pos as usize].token_value =
                        TokenValue::IF(Some((pos + 1) as u64));
                    stack.push(pos);
                },
                TokenValue::END => {
                    let block_pos = match stack.pop() {
                        Some(v) => v,
                        None => {return Err(ProgramParsingError::IfMissingEnd())}
                    };
                    match program[block_pos as usize].token_value {
                        TokenValue::IF(_v) => {
                            program[block_pos as usize].token_value =
                                TokenValue::IF(Some((pos) as u64));
                        }
                        TokenValue::ELSE(_v) => {
                            program[block_pos as usize].token_value =
                                TokenValue::ELSE(Some((pos) as u64));
                        }
                        _ => return Err(ProgramParsingError::IfMatchError())
                    }
                },
                _ => {}
            }
        }
        Ok(program)
    }

    fn word_to_token(&self, word: &str) -> Result<TokenValue, ProgramParsingError> {
        Ok(match word {
            "+" => TokenValue::PLUS,
            "-" => TokenValue::MINUS,
            "." => TokenValue::DUMP,
            "=" => TokenValue::EQUAL,
            "if" => TokenValue::IF(None),
            "else" => TokenValue::ELSE(None),
            "end" => TokenValue::END,
            _ => {
                if !word.chars().all(|x| x.is_numeric()) {
                    return Err(ProgramParsingError::InvalidToken(String::new(), 0));
                }
                TokenValue::PUSH(u64::from_str(word)?)
            }
        })
    }
}