use std::{collections::HashMap, fmt::Write};

use crate::parser::{Expr, Function, Statement};

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

    // TODO: allow other types than i32
    // prepare the stack frame lookup table
    let mut variables = HashMap::new();
    for statement in program.body.iter() {
        match statement {
            Statement::Declare { name }
            | Statement::Assign { name, value: _ }
            | Statement::Initialize { name, value: _ } => {
                let sp_offset = (variables.len() + 1) * 4;

                // TODO: check duplicate declaration
                if variables.get(name) == None {
                    variables.insert(name, sp_offset);
                }
            }

            _ => {}
        }
    }

    // TODO: check if function actually calls something
    // We reserve 16 bytes on the stack for storing x29, x30
    let stack_size = ((variables.len() as f64 * 4.0) / 16.0).ceil() as usize * 16 + 16;
    let sp_x29 = stack_size - 16;

    // TODO: also process in first pass?
    let mut labels = Vec::<String>::new();

    writeln!(
        &mut output,
        "\
            sub sp, sp, #{stack_size}\n\
            stp x29, x30, [sp, #{sp_x29}]\n\
            add x29, sp, #{sp_x29}\n"
    )?;

    for statement in program.body.iter() {
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

            Statement::Return {
                return_value: Expr::LitInt(n),
            } => {
                writeln!(
                    &mut output,
                    "\
                    ldp x29, x30, [sp, #{sp_x29}]\n\
                    add sp, sp, #{stack_size}\n"
                )?;

                writeln!(
                    &mut output,
                    "\
                    ; return {n}\n\
                    mov w0, #{n}\n\
                    ret\n",
                )?
            }

            Statement::Declare { name: _ } => {
                // no op in second pass
            }

            Statement::Initialize {
                name,
                value: Expr::LitInt(n),
            } => {
                let sp_offset = sp_x29
                    - variables
                        .get(&name)
                        .ok_or(CodegenError::UndeclaredVariable)?;

                writeln!(
                    &mut output,
                    "\
                    ; {name} = {n}\n\
                    mov w8, #{n}\n\
                    str w8, [sp, #{sp_offset}]\n",
                )?
            }

            Statement::Assign {
                name,
                value: Expr::Identifier(rhs),
            } => {
                let sp_lhs_offset = sp_x29
                    - variables
                        .get(&name)
                        .ok_or(CodegenError::UndeclaredVariable)?;

                let sp_rhs_offset = sp_x29
                    - variables
                        .get(&rhs)
                        .ok_or(CodegenError::UndeclaredVariable)?;

                writeln!(
                    &mut output,
                    "\
                    ; {name} = {rhs}\n\
                    ldr w8, [sp, #{sp_rhs_offset}]\n\
                    str w8, [sp, #{sp_lhs_offset}]\n",
                )?
            }

            x => todo!("Implement statement {:?}", x),
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

// fn expect_expr_type(expr: &Expr, t: Expr){}
