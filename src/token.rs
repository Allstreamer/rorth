

#[derive(Debug, Copy, Clone)]
pub enum TokenValue {
    PUSH(u64),
    PLUS,
    MINUS,
    DUMP,
    EQUAL
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_value: TokenValue,
    pub file: String,
    pub line: usize,
}

