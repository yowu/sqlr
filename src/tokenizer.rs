use crate::token::Token;

fn is_keyword(word: &str) -> bool {
    matches!(
        word,
        "CREATE"
            | "TABLE"
            | "SELECT"
            | "INSERT"
            | "UPDATE"
            | "DELETE"
            | "FROM"
            | "WHERE"
            | "AND"
            | "OR"
            | "NOT"
            | "IN"
            | "VALUES"
            | "SET"
            | "JOIN"
            | "ON"
            | "AS"
            | "ORDER"
            | "BY"
            | "GROUP"
            | "HAVING"
            | "LIMIT"
            | "OFFSET"
            | "DISTINCT"
            | "ALTER"
            | "DROP"
            | "ADD"
            | "COLUMN"
            | "INDEX"
            | "VIEW"
            | "TRIGGER"
            | "PROCEDURE"
            | "FUNCTION"
            | "DATABASE"
            | "SCHEMA"
            | "USE"
            | "SHOW"
            | "DESCRIBE"
            | "EXPLAIN"
    )
}

fn is_data_type(word: &str) -> bool {
    matches!(
        word,
        "INT"
            | "INTEGER"
            | "VARCHAR"
            | "CHAR"
            | "TEXT"
            | "DATE"
            | "TIMESTAMP"
            | "BOOLEAN"
            | "FLOAT"
            | "DOUBLE"
            | "DECIMAL"
    )
}

pub fn tokenize(statement: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = statement.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            ' ' | '\t' | '\n' => {
                chars.next();
            }
            '(' | ')' | ',' | ';' | '.' => {
                tokens.push(Token::Punctuation(ch));
                chars.next();
            }
            '=' | '<' | '>' | '!' => {
                let mut op = String::new();
                op.push(ch);
                chars.next();
                if let Some(&next_ch) = chars.peek() {
                    if next_ch == '=' {
                        op.push(next_ch);
                        chars.next();
                    }
                }
                tokens.push(Token::Operator(op));
            }
            '\'' => {
                chars.next(); // consume the opening quote
                let mut literal = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch == '\'' {
                        chars.next(); // consume the closing quote
                        break;
                    } else {
                        literal.push(ch);
                        chars.next();
                    }
                }
                tokens.push(Token::Literal(literal));
            }
            _ if ch.is_alphabetic() => {
                let mut ident = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_alphanumeric() || ch == '_' {
                        ident.push(ch);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let upper_ident = ident.to_uppercase();
                if is_keyword(&upper_ident) {
                    tokens.push(Token::Keyword(ident));
                } else if is_data_type(&upper_ident) {
                    tokens.push(Token::DataType(ident));
                } else {
                    tokens.push(Token::Identifier(ident));
                }
            }
            _ if ch.is_numeric() => {
                let mut number = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_numeric() {
                        number.push(ch);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Number(number));
            }
            _ => {
                chars.next();
            }
        }
    }

    tokens
}
