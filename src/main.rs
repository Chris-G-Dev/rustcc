use asm_gen::AsmGenerator;
use clap::Parser as ClapParser;
use code_emission::CodeEmitter;
use lexer::Lexer;
use parser::Parser;
use std::error::Error;
use std::fs;
use std::io::Read;
use std::path::PathBuf;
use std::process::Command;

mod asm_gen;
mod code_emission;
mod lexer;
mod parser;

#[derive(ClapParser, Debug)]
#[command(name = "C Compiler", about = "C Compiler built in Rust")]
struct CliConfig {
    // Flag to determine if compiler should only run up to lexer
    #[arg(short = 'l', long = "lex")]
    lex: bool,

    #[arg(short = 'p', long = "parse")]
    parse: bool,

    #[arg(short = 'c', long = "codegen")]
    codegen: bool,

    // Input file to compile
    #[arg(value_name = "FILE")]
    file: String,
}

// TODO: Implement flag logic - cutoff points; .i / .s file outputs

fn main() -> Result<(), Box<dyn Error>> {
    let args = CliConfig::parse();

    let abs_path = fs::canonicalize(PathBuf::from(&args.file))?;
    let abs_path_stemmed = abs_path.with_extension("");

    // Preprocess
    let preprocessed_file = preprocess(&abs_path)?;

    // Lexing
    let tokens = Lexer::new(preprocessed_file).lex()?;

    // Parsing
    let ast = Parser::new(tokens).parse()?;

    // Assembly Generation
    let asm_ast = AsmGenerator::new(ast).generate()?;

    // Code emission
    CodeEmitter::new(asm_ast, &abs_path).emit()?;

    assemble_and_link(abs_path_stemmed)?;

    Ok(())
}

fn preprocess(source: &PathBuf) -> Result<String, Box<dyn Error>> {
    let mut preprocessed_file_path = source.with_extension("i");
    let mut preprocessed_file = String::new();

    if source.exists() {
        preprocessed_file_path = source.with_extension("i");

        Command::new("gcc")
            .arg("-E")
            .arg("-P")
            .arg(source)
            .arg("-o")
            .arg(&preprocessed_file_path)
            .output()?;
    }

    fs::File::options()
        .read(true)
        .open(preprocessed_file_path)?
        .read_to_string(&mut preprocessed_file)?;

    Ok(preprocessed_file)
}

fn assemble_and_link(source_stemmed: PathBuf) -> Result<(), Box<dyn Error>> {
    Command::new("gcc")
        .arg(source_stemmed.with_extension("s"))
        .arg("-o")
        .arg(source_stemmed.with_extension(""))
        .output()?;

    fs::remove_file(source_stemmed.with_extension("i"))?;
    fs::remove_file(source_stemmed.with_extension("s"))?;

    Ok(())
}
