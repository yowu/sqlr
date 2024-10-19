use super::token::Token;

struct Tokenizer<'a> {
    chars: std::iter::Peekable<std::str::Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    fn new(statement: &'a str) -> Self {
        Tokenizer {
            chars: statement.chars().peekable(),
        }
    }

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
                | "INTO"
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
                | "BOOL"
                | "BOOLEAN"
                | "FLOAT"
                | "DOUBLE"
                | "DECIMAL"
        )
    }

    fn is_next_char_digit(&mut self) -> bool {
        if let Some(&next_ch) = self.chars.peek() {
            next_ch.is_numeric()
        } else {
            false
        }
    }

    fn is_bool_literal(word: &str) -> bool {
        matches!(word, "TRUE" | "FALSE")
    }

    fn parse_numeric(&mut self, initial_char: Option<char>) -> String {
        let mut number = String::new();
        if let Some(ch) = initial_char {
            number.push(ch);
        }
        while let Some(&ch) = self.chars.peek() {
            if ch.is_numeric() || ch == '.' {
                number.push(ch);
                self.chars.next();
            } else {
                break;
            }
        }
        number
    }

    fn parse_identifier_or_keyword(&mut self) -> String {
        let mut ident = String::new();
        while let Some(&ch) = self.chars.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                ident.push(ch);
                self.chars.next();
            } else {
                break;
            }
        }
        ident
    }

    fn parse_literal(&mut self, quote: char) -> String {
        let mut literal = String::new();
        while let Some(&ch) = self.chars.peek() {
            if ch == quote {
                self.chars.next(); // consume the closing quote
                break;
            } else {
                literal.push(ch);
                self.chars.next();
            }
        }
        literal
    }

    fn parse_operator(&mut self, initial_char: char) -> String {
        let mut op = String::new();
        op.push(initial_char);
        self.chars.next();
        if let Some(&next_ch) = self.chars.peek() {
            if next_ch == '=' {
                op.push(next_ch);
                self.chars.next();
            }
        }
        op
    }

    fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(&ch) = self.chars.peek() {
            match ch {
                ' ' | '\t' | '\n' => {
                    self.chars.next();
                }
                '(' | ')' | ',' | ';' | '.' => {
                    tokens.push(Token::Punctuation(ch));
                    self.chars.next();
                }
                '=' | '<' | '>' | '!' => {
                    let op = self.parse_operator(ch);
                    tokens.push(Token::Operator(op));
                }
                '\'' | '"' => {
                    self.chars.next(); // consume the opening quote
                    let literal = self.parse_literal(ch);
                    tokens.push(Token::Literal(literal));
                }
                '-' => {
                    self.chars.next(); // consume the '-'
                    if self.is_next_char_digit() {
                        let number = self.parse_numeric(Some('-'));
                        tokens.push(Token::Numeric(number));
                    } else {
                        tokens.push(Token::Operator(String::from("-")));
                    }
                }
                _ if ch.is_alphabetic() => {
                    let ident = self.parse_identifier_or_keyword();
                    let upper_ident = ident.to_uppercase();
                    if Tokenizer::is_keyword(&upper_ident) {
                        tokens.push(Token::Keyword(ident));
                    } else if Tokenizer::is_data_type(&upper_ident) {
                        tokens.push(Token::DataType(ident));
                    } else if Tokenizer::is_bool_literal(&upper_ident) {
                        tokens.push(Token::Literal(ident));
                    } else {
                        tokens.push(Token::Identifier(ident));
                    }
                }
                _ if ch.is_numeric() => {
                    let number = self.parse_numeric(None);
                    tokens.push(Token::Numeric(number));
                }
                _ => {
                    self.chars.next();
                }
            }
        }

        tokens
    }
}

pub fn tokenize(statement: &str) -> Vec<Token> {
    let mut tokenizer = Tokenizer::new(statement);
    tokenizer.tokenize()
}
