use std::{
    error::Error,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use crate::asm_gen::{
    AsmAst,
    Instruction::{self, Mov, Ret},
    Operand::{self, Imm, Register},
};

pub struct CodeEmitter {
    asm_ast: AsmAst,
    code: String,
    source: PathBuf,
}

impl CodeEmitter {
    pub fn new(asm_ast: AsmAst, source: &PathBuf) -> Self {
        Self {
            asm_ast,
            code: String::new(),
            source: source.clone(),
        }
    }

    pub fn emit(&mut self) -> Result<(), Box<dyn Error>> {
        let functions = match &self.asm_ast.clone() {
            AsmAst::Program(funcs) => funcs,
        }
        .to_owned();

        functions.iter().for_each(|f| {
            let asm_str = format!(
                r#"
                    .globl {}
                {}:
                    {}
            "#,
                f.name,
                f.name,
                self.emit_instructions(&f.instructions)
            );

            self.code.push_str(&format!("{asm_str}"));
        });

        self.code
            .push_str(r#".section .note.GNU-stack,"",@progbits"#);

        self.write_to_file()?;

        Ok(())
    }

    pub fn emit_instructions(&self, instructions: &Vec<Instruction>) -> String {
        let mut instruction_str = String::new();

        for instruction in instructions {
            match instruction {
                Mov(src, dst) => instruction_str.push_str(&format!(
                    "movl {}, {}",
                    self.emit_operand(src),
                    self.emit_operand(dst)
                )),
                Ret => instruction_str.push_str("\nret"),
            }
        }

        instruction_str
    }

    pub fn emit_operand(&self, operand: &Operand) -> String {
        // let operand_str = String::new();

        match operand {
            Imm(c) => format!("${c}"),
            Register => "%eax".to_string(),
        }
    }

    fn write_to_file(&mut self) -> Result<(), Box<dyn Error>> {
        let path_str = Path::new(&self.source)
            .with_extension("")
            .as_os_str()
            .to_owned();

        let asm_path = Path::new(&path_str).with_extension("s");
        let mut file = File::create_new(&asm_path)?;
        let buf = *&self.code.as_bytes();

        file.write_all(buf)?;

        Ok(())
    }
}
