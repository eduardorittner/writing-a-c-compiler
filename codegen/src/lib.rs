use std::fmt::Write;
use tracing::info;

pub mod error;
pub use error::*;

use x86::{FnDef, Instruction, Operand, Program, X86};

pub struct Codegen<'input> {
    input: &'input X86,
    output: String,
}

impl<'input> Codegen<'input> {
    pub fn new(input: &'input X86) -> Self {
        Self {
            input,
            output: String::new(),
        }
    }

    pub fn emit(&'input mut self) -> &'input str {
        self.input.nodes.fmt(&mut self.output);
        self.emit_footer();
        &self.output
    }

    pub fn emit_from_input(input: &'input X86) -> CodegenResult<String> {
        let mut codegen = Codegen::new(input);
        codegen.input.nodes.fmt(&mut codegen.output);
        codegen.emit_footer();
        Ok(codegen.output.clone())
    }

    pub fn output(&'input self) -> &'input str {
        &self.output
    }

    fn emit_footer(&mut self) {
        // Disables executable stack to silence linker warning
        #[cfg(target_os = "linux")]
        self.output
            .push_str("\n.section .note.GNU-stack,\"\",@progbits\n");
    }
}

/// Very similar to the `Display` trait, but we can't impl it for `X86` since both are defined in
/// other crates.
trait Format {
    fn fmt(&self, string: &mut String);
}

impl Format for Program {
    fn fmt(&self, string: &mut String) {
        self.main.fmt(string);
    }
}

impl Format for FnDef {
    fn fmt(&self, string: &mut String) {
        info!("Formatting function");
        let name = if cfg!(target_os = "macos") {
            &("_".to_string() + &self.name)
        } else {
            &self.name
        };
        let _ = writeln!(string, ".globl {}", name);
        let _ = writeln!(string, "{}:", name);
        self.body.iter().for_each(|instr| instr.fmt(string));
    }
}

impl Format for Instruction {
    fn fmt(&self, string: &mut String) {
        match self {
            Instruction::Mov { src, dst } => {
                let _ = write!(string, "    movl ");
                src.fmt(string);
                let _ = write!(string, ", ");
                dst.fmt(string);
                let _ = writeln!(string);
            }
            Instruction::Ret => {
                let _ = writeln!(string, "    ret");
            }
        }
    }
}

impl Format for Operand {
    fn fmt(&self, string: &mut String) {
        match self {
            Operand::Immediate(value) => {
                let _ = write!(string, "${}", value);
            }
            Operand::Register => {
                let _ = write!(string, "%eax");
            }
        }
    }
}
