// #![allow(dead_code, unused_imports)]
use crate::parser::{
    AST, Expression as ParsedExpression, Function as ParsedFunction, Statement as ParsedStatement,
};
use std::error::Error;
// use std::io::{self, Write};

#[derive(Debug, Clone)]
pub enum AsmAst {
    Program(Vec<Function>),
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub instructions: Vec<Instruction>,
}

#[derive(Debug, Copy, Clone)]
pub enum Operand {
    Imm(i32),
    Register,
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Mov(Operand, Operand),
    Ret,
}

pub struct AsmGenerator {
    parsed_functions: Vec<ParsedFunction>,
}

impl AsmGenerator {
    pub fn new(ast: AST) -> Self {
        let parsed_functions = match ast {
            AST::Program(funcs) => funcs,
        };

        Self { parsed_functions }
    }

    pub fn generate(&self) -> Result<AsmAst, Box<dyn Error>> {
        let mut asm_functions = vec![];

        self.parsed_functions
            .iter()
            .try_for_each(|f| -> Result<(), Box<dyn Error>> {
                asm_functions.push(self.function_gen(f)?);
                Ok(())
            })?;

        Ok(AsmAst::Program(asm_functions))
    }

    fn function_gen(&self, function: &ParsedFunction) -> Result<Function, Box<dyn Error>> {
        Ok(Function {
            name: function.name.to_owned(),
            instructions: self.instructions_gen(&function.body)?,
        })
    }

    fn instructions_gen(&self, body: &ParsedStatement) -> Result<Vec<Instruction>, Box<dyn Error>> {
        let instructions = match body {
            ParsedStatement::Return(expression) => {
                let (src, destination) = &self.operand_gen(expression)?;

                vec![
                    Instruction::Mov(src.to_owned(), destination.to_owned()),
                    Instruction::Ret,
                ]
            }
        };

        Ok(instructions)
    }

    fn operand_gen(
        &self,
        expression: &ParsedExpression,
    ) -> Result<(Operand, Operand), Box<dyn Error>> {
        match expression {
            ParsedExpression::Constant(c) => Ok((Operand::Imm(*c), Operand::Register)),
        }
    }

    // pub fn parse(&mut self) -> Result<AST, Box<dyn Error>> {
    //     let mut functions = vec![];

    //     while !self.tokens.is_empty() {
    //         functions.push(self.parse_function()?);
    //     }

    //     // TODO: Remove once pretty print functionality is implemented
    //     // let mut functions_debug = functions.clone();
    //     // println!("AST: {:?}", AST::Program(functions_debug));

    //     Ok(AST::Program(functions))
    // }

    // pub fn take_token(&mut self) -> Result<Token, Box<dyn Error>> {
    //     Ok(self.tokens.remove(0))
    // }

    // pub fn parse_function(&mut self) -> Result<Function, Box<dyn Error>> {
    //     let token = self.take_token()?;

    //     if token.kind != TokenType::KeywordInt {
    //         return Err("Expected type at function start".into());
    //     }

    //     let name = self.expect_identifier()?;

    //     self.expect_stream(&[
    //         TokenType::LParens,
    //         TokenType::KeywordVoid,
    //         TokenType::RParens,
    //         TokenType::LBrace,
    //     ])?;

    //     // TODO: Handle body
    //     let body = self.parse_statement()?; // assume function body is a single statement for now

    //     self.expect(TokenType::RBrace)?;

    //     Ok(Function { name, body })
    // }

    // pub fn parse_statement(&mut self) -> Result<Statement, Box<dyn Error>> {
    //     let statement = match self.take_token()?.kind {
    //         TokenType::KeywordReturn => Statement::Return(self.parse_expression()?),
    //         other => return Err(format!("Expected Statement; found {other:?}").into()),
    //     };

    //     self.expect(TokenType::Semicolon)?;

    //     Ok(statement)
    // }

    // pub fn parse_expression(&mut self) -> Result<Expression, Box<dyn Error>> {
    //     let expression = match self.take_token() {
    //         Ok(T) if T.kind == TokenType::Constant => Expression::Constant(T.value.parse::<i32>()?),
    //         other => return Err(format!("Expected Expression; found {other:?}").into()),
    //     };

    //     Ok(expression)
    // }

    // pub fn expect(&mut self, expected: TokenType) -> Result<(), Box<dyn Error>> {
    //     if let Ok(token) = self.take_token() {
    //         if token.kind != expected {
    //             return Err(format!("Unexpected token: {}", token.value).into());
    //         }
    //     }

    //     Ok(())
    // }

    // pub fn expect_stream(&mut self, expected_tokens: &[TokenType]) -> Result<(), Box<dyn Error>> {
    //     expected_tokens
    //         .iter()
    //         .try_for_each(|expected_token| self.expect(*expected_token))?;
    //     eprintln!("Huh????");
    //     io::stdout().flush().unwrap();
    //     Ok(())
    // }

    // pub fn expect_identifier(&mut self) -> Result<String, Box<dyn Error>> {
    //     if let Some(token) = self.tokens.first() {
    //         if token.kind == TokenType::Identifier {
    //             return Ok(self.tokens.remove(0).value);
    //         }
    //     }

    //     Err("Expected identifier token".into())
    // }
}
