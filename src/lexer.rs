use regex::Regex;
use std::error::Error;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenType {
    KeywordInt,
    KeywordVoid,
    KeywordReturn,
    Identifier,
    Constant,
    LParens,
    RParens,
    LBrace,
    RBrace,
    Semicolon,
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenType,
    pub value: String,
}

pub struct Lexer {
    token_patterns: [(TokenType, regex::Regex); 10],
    source: String,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            source: source.trim().to_owned(),
            token_patterns: [
                (TokenType::KeywordInt, Regex::new(r"^int\b").unwrap()),
                (TokenType::KeywordVoid, Regex::new(r"^void\b").unwrap()),
                (TokenType::KeywordReturn, Regex::new(r"^return\b").unwrap()),
                (
                    TokenType::Identifier,
                    Regex::new(r"^[a-zA-Z_]\w*\b").unwrap(),
                ),
                (TokenType::Constant, Regex::new(r"^[0-9]+\b").unwrap()),
                (TokenType::LParens, Regex::new(r"^\(").unwrap()),
                (TokenType::RParens, Regex::new(r"^\)").unwrap()),
                (TokenType::LBrace, Regex::new(r"^\{").unwrap()),
                (TokenType::RBrace, Regex::new(r"^\}").unwrap()),
                (TokenType::Semicolon, Regex::new(r"^;").unwrap()),
            ],
        }
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, Box<dyn Error>> {
        let mut tokens: Vec<Token> = vec![];

        while !self.source.is_empty() {
            let mut matched = false;
            let matched_kind: TokenType;

            for (kind, regex) in self.token_patterns.iter() {
                // let regex = Regex::new(pattern)?;

                if let Some(key) = regex.find(&self.source) {
                    matched_kind = kind.clone();

                    tokens.push(Token {
                        kind: matched_kind,
                        value: key.as_str().to_string(),
                    });

                    self.source = self.source[key.end()..].trim_start().to_owned();

                    matched = true;

                    break;
                }
            }

            if !matched {
                return Err("No match found".into());
            }
        }

        Ok(tokens)
    }
}
