use std::fs::File;
use std::{io, io::{BufRead, BufReader}};
use std::num::ParseIntError;
use std::path::PathBuf;
use std::str::FromStr;
use thiserror::Error;
use crate::Token;

#[derive(Error, Debug)]
pub enum ProgramParsingError {
    #[error("")]
    Io {
        #[from]
        source: io::Error
    },
    #[error("")]
    IntError {
        #[from]
        source: ParseIntError
    },
    #[error("")]
    InvalidToken(String)
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
        for line in reader.lines() {
            for word in line.unwrap().split_whitespace() {
                tokens.push(self.word_to_token(word)?);
            }
        }

        Ok(tokens)
    }

    fn word_to_token(&self, word: &str) -> Result<Token, ProgramParsingError> {
        Ok(match word {
            "+" => {Token::PLUS},
            "-" => {Token::MINUS},
            "." => {Token::DUMP},
            _ => {
                if !word.chars().all(|x| x.is_numeric()) {
                    return Err(
                        ProgramParsingError::InvalidToken(
                            String::from(word)
                        )
                    );
                }
                Token::PUSH(u64::from_str(word)?)
            }
        })
    }
}