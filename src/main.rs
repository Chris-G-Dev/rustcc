use clap::Parser as ClapParser;
use lexer::Lexer;
use parser::Parser;
use std::error::Error;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::Command;

mod lexer;
mod parser;

#[derive(ClapParser, Debug)]
#[command(name = "C Compiler", about = "C Compiler built in Rust")]
struct CliConfig {
    // Flag to determine if compiler should only run up to lexer
    #[arg(short = 'l', long = "lex")]
    lex: bool,

    // Input file to compile
    #[arg(value_name = "FILE")]
    file: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = CliConfig::parse();

    // Preprocess

    let preprocessed_file = preprocess(&args.file)?;

    // Lexing

    let mut lexer = Lexer::new(preprocessed_file);
    let tokens = lexer.lex()?;

    // for token in &tokens {
    //     eprintln!("{:?} {:?}", token.kind, token.value);
    // }

    // Parsing

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    println!("{ast:?}");

    // TODO: assembly generation
    // TODO: code emission

    Ok(())
}

fn preprocess(source: &String) -> Result<String, Box<dyn Error>> {
    let path = Path::new(source);
    let mut preprocessed_file_path = PathBuf::new();

    if path.exists() {
        preprocessed_file_path = path.with_extension("i");

        Command::new("gcc")
            .arg("-E")
            .arg("-P")
            .arg(source)
            .arg("-o")
            .arg(&preprocessed_file_path)
            .output()?;
    }

    let preprocessed_file_path = preprocessed_file_path
        .to_str()
        .ok_or("Failed to preprocess file.")?
        .to_owned();

    let mut preprocessed_file = String::new();

    fs::File::options()
        .read(true)
        .open(preprocessed_file_path)?
        .read_to_string(&mut preprocessed_file)?;

    Ok(preprocessed_file)
}
