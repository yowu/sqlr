#[derive(Debug, PartialEq)]
pub enum Token {
    Keyword(String),
    Identifier(String),
    DataType(String),
    Punctuation(char),
    Literal(String),
    Operator(String),
    Numeric(String),
}
