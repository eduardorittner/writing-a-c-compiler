use std::fmt::Write;

use tacky::{FnDef, Instruction, Operand, Program, Tacky};
pub struct Codegen<'input> {
    input: &'input Tacky,
    output: String,
}

impl<'input> Codegen<'input> {
    pub fn new(input: &'input Tacky) -> Self {
        Self {
            input,
            output: String::new(),
        }
    }

    pub fn emit(&mut self) {
        self.input.nodes.fmt(&mut self.output);
    }

    pub fn output(&'input self) -> &'input str {
        &self.output
    }

    fn emit_function(&mut self, name: &str) {
        #[cfg(target_os = "linux")]
        self.output.push_str(name);

        #[cfg(target_os = "macos")]
        self.output.push_str("_" + name);
    }
}

/// Very similar to the `Display` trait, but we can't impl it for `Tacky` since both are defined in
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
        let _ = writeln!(string, ".globl {}", &self.name);
        let _ = writeln!(string, "{}:", &self.name);
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
