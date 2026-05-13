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
    let mut stack_size = ((variable_count as f64 * 4.0) / 16.0).ceil() as usize * 16;

    // TODO: check if function actually calls something
    // We reserve 16 bytes on the stack for storing x29, x30
    stack_size += 16;

    let mut variables = vec!["x29".to_string(), "x30".to_string()];
    let mut labels = Vec::<String>::new();

    let sp_x29 = stack_size - 16;

    writeln!(
        &mut output,
        "\
            sub sp, sp, #{stack_size}\n\
            stp x29, x30, [sp, #{sp_x29}]\n\
            mov x29, sp\n"
    )?;

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
                writeln!(
                    &mut output,
                    "\
                    ; printf(...)\n\
                    adrp x0, label_{n}@PAGE\n\
                    add x0, x0, label_{n}@PAGEOFF\n\
                    bl _printf\n"
                )?
            }

            Statement::Return { return_value } => {
                writeln!(
                    &mut output,
                    "\
                    ldp x29, x30, [sp, #{sp_x29}]\n\
                    add sp, sp, #{stack_size}\n"
                )?;

                writeln!(
                    &mut output,
                    "\
                    ; return {return_value}\n\
                    mov w0, #{return_value}\n\
                    ret\n",
                )?
            }

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

    if labels.len() > 0 {
        writeln!(&mut output, ".section __TEXT,__cstring,cstring_literals")?;
    }

    for (i, label) in labels.iter().enumerate() {
        let n = i + 1;
        writeln!(&mut output, "label_{n}: .asciz {label}")?;
    }

    Ok(output)
}
