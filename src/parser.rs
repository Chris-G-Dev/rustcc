#![allow(dead_code)]
use crate::lexer::{Token, TokenType};
use std::error::Error;

// enum

#[derive(Debug)]
pub enum AST {
    Program(Vec<Function>),
}

#[derive(Debug, Clone)]
pub struct Function {
    name: String,
    body: Statement,
}

#[derive(Debug)]
pub struct Return {
    expression: Option<Expression>,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Return(Expression),
}

#[derive(Debug, Clone)]
pub enum Expression {
    Constant(i32),
}

pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens }
    }

    pub fn parse(&mut self) -> Result<AST, Box<dyn Error>> {
        let mut functions = vec![];

        while !self.tokens.is_empty() {
            functions.push(self.parse_function()?);
        }

        // TODO: Remove once pretty print functionality is implemented
        // let mut functions_debug = functions.clone();
        // println!("AST: {:?}", AST::Program(functions_debug));

        Ok(AST::Program(functions))
    }

    pub fn take_token(&mut self) -> Result<Token, Box<dyn Error>> {
        Ok(self.tokens.remove(0))
    }

    pub fn parse_function(&mut self) -> Result<Function, Box<dyn Error>> {
        let token = self.take_token()?;

        if token.kind != TokenType::KeywordInt {
            return Err("Expected type at function start".into());
        }

        let name = self.expect_identifier()?;

        self.expect_stream(&[
            TokenType::LParens,
            TokenType::KeywordVoid,
            TokenType::RParens,
            TokenType::LBrace,
        ])?;

        // TODO: Handle body
        let body = self.parse_statement()?; // assume function body is a single statement for now

        self.expect(TokenType::RBrace)?;

        Ok(Function { name, body })
    }

    pub fn parse_statement(&mut self) -> Result<Statement, Box<dyn Error>> {
        let statement = match self.take_token()?.kind {
            TokenType::KeywordReturn => Statement::Return(self.parse_expression()?),
            other => return Err(format!("Expected Statement; found {other:?}").into()),
        };

        self.expect(TokenType::Semicolon)?;

        Ok(statement)
    }

    pub fn parse_expression(&mut self) -> Result<Expression, Box<dyn Error>> {
        let expression = match self.take_token() {
            Ok(T) if T.kind == TokenType::Constant => Expression::Constant(T.value.parse::<i32>()?),
            other => return Err(format!("Expected Expression; found {other:?}").into()),
        };

        Ok(expression)
    }

    pub fn expect(&mut self, expected: TokenType) -> Result<(), Box<dyn Error>> {
        if let Ok(token) = self.take_token() {
            if token.kind != expected {
                return Err(format!("Unexpected token: {}", token.value).into());
            }
        }

        Ok(())
    }

    pub fn expect_stream(&mut self, expected_tokens: &[TokenType]) -> Result<(), Box<dyn Error>> {
        expected_tokens
            .iter()
            .try_for_each(|expected_token| self.expect(*expected_token))?;

        Ok(())
    }

    pub fn expect_identifier(&mut self) -> Result<String, Box<dyn Error>> {
        if let Some(token) = self.tokens.first() {
            if token.kind == TokenType::Identifier {
                return Ok(self.tokens.remove(0).value);
            }
        }

        Err("Expected identifier token".into())
    }
}
