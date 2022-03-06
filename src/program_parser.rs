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

        let mut tokens: Vec<Token> = Vec::new();
        for (index,line) in reader.lines().enumerate() {
            for word in line?.split_whitespace() {
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

    fn word_to_token(&self, word: &str) -> Result<TokenValue, ProgramParsingError> {
        Ok(match word {
            "+" => {TokenValue::PLUS},
            "-" => {TokenValue::MINUS},
            "." => {TokenValue::DUMP},
            "=" => {TokenValue::EQUAL},
            _ => {
                if !word.chars().all(|x| x.is_numeric()) {
                    return Err(ProgramParsingError::InvalidToken(String::new(), 0));
                }
                TokenValue::PUSH(u64::from_str(word)?)
            }
        })
    }
}