use std::fmt::Write;

use crate::parser::{Function, Statement};

#[derive(Debug)]
pub enum CodegenError {
    WriteError(std::fmt::Error),
    UndeclaredVariable,
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

    // Make room on the stack
    let variable_count = program
        .body
        .iter()
        .filter(|s| {
            matches!(
                s,
                Statement::Initialize { name: _, value: _ }
                    | Statement::Assign { name: _, value: _ }
            )
        })
        .count();

    // TODO: allow other types than i32
    let stack_size = ((variable_count as f64 * 4.0) / 16.0).ceil() as usize * 16;
    if stack_size > 0 {
        writeln!(&mut output, "sub sp, sp, #{stack_size}\n")?;
    }

    let mut variables = Vec::<String>::new();
    let mut labels = Vec::<String>::new();

    for statement in program.body {
        match statement {
            Statement::Funcall { name, args } => {
                assert!(
                    name == "printf",
                    "We don't support actual functions instead wiring to printf"
                );
                assert!(args.len() == 1, "We only support single string arg");

                labels.push(args[0].clone());

                let n = labels.len();
                let len = args[0].len();
                writeln!(
                    &mut output,
                    "\
                    ; fd 1 = stdout\n\
                    mov x0, #1\n\
                    ; x1: address of the string\n\
                    adrp x1, label_{n}@PAGE\n\
                    add x1, x1, label_{n}@PAGEOFF\n\
                    ; x2: length of the string\n\
                    mov x2, #{len}\n\
                    ; x16: 4 = syscall write\n\
                    mov x16, #4\n\
                    svc 0x80\n",
                )?
            }

            Statement::Return { return_value } => writeln!(
                &mut output,
                "\
                ; syscall exit with code in x0\n\
                mov x0, #{return_value}\n\
                mov x16, #1\n\
                svc 0x80\n",
            )?,

            Statement::Initialize { name, value } => {
                variables.push(name.clone());
                let sp_offset = stack_size - variables.len() * 4;

                writeln!(
                    &mut output,
                    "\
                    ; {name} = {value}\n\
                    mov w8, #{value}\n\
                    str w8, [sp, #{sp_offset}]\n",
                )?
            }

            Statement::Assign { name, value } => {
                variables.push(name.clone());
                let sp_lhs_offset = stack_size - variables.len() * 4;

                match variables.iter().position(|x| *x == value) {
                    None => return Err(CodegenError::UndeclaredVariable),
                    Some(variable_idx) => {
                        let sp_rhs_offset = stack_size - (variable_idx + 1) * 4;
                        writeln!(
                            &mut output,
                            "\
                            ; {name} = {value}\n\
                            ldr w8, [sp, #{sp_rhs_offset}]\n\
                            str w8, [sp, #{sp_lhs_offset}]\n",
                        )?
                    }
                }
            }
        }
    }

    for (i, label) in labels.iter().enumerate() {
        let n = i + 1;
        write!(&mut output, "label_{n}: .ascii {label}")?;
    }

    Ok(output)
}
