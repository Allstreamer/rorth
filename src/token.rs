

#[derive(Debug, Copy, Clone)]
pub enum TokenValue {
    PUSH(u64),
    PLUS,
    MINUS,
    DUMP,
    EQUAL,
    IF(Option<u64>),
    ELSE(Option<u64>),
    END
}

/// Token Value Wrapping Struct for error reporting
#[derive(Debug, Clone)]
pub struct Token {
    pub token_value: TokenValue,
    pub file: String,
    pub line: usize,
}

