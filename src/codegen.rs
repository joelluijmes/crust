use std::fmt::Write;

use crate::parser::{Function, Statement};

#[derive(Debug)]
pub enum CodegenError {
    WriteError(std::fmt::Error),
}

impl From<std::fmt::Error> for CodegenError {
    fn from(v: std::fmt::Error) -> Self {
        Self::WriteError(v)
    }
}

pub fn generate_program(program: Function) -> Result<String, CodegenError> {
    let mut output = String::new();

    write!(
        &mut output,
        ".global _main\n\
        .align 2\n\
        \n\
        _main:\n"
    )?;

    for statement in program.body {
        match statement {
            Statement::Return { return_value } => write!(
                &mut output,
                "\t; syscall exit with code in x0\n\
                \tmov x0, {return_value}\n\
                \tmov x16, 1\n\
                \tsvc 0x80\n",
            )?,
            _ => todo!(),
        }
    }

    Ok(output)
}
