use std::fmt::{Display, Formatter};

use crate::{AAst, FnDef, Instruction, Operand, Program};

impl Display for AAst {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.nodes.fmt_node(0, f);
        Ok(())
    }
}

pub trait DisplayNode {
    fn fmt_node(&self, indent: usize, f: &mut Formatter<'_>);
    fn write(&self, indent: usize, string: &str, f: &mut Formatter<'_>) {
        write!(f, "{}{}", " ".repeat(indent), string).unwrap();
    }
    fn writeln(&self, indent: usize, string: &str, f: &mut Formatter<'_>) {
        writeln!(f, "{}{}", " ".repeat(indent), string).unwrap();
    }
}

impl DisplayNode for AAst {
    fn fmt_node(&self, indent: usize, f: &mut Formatter<'_>) {
        self.nodes.fmt_node(indent, f);
    }
}

impl DisplayNode for Program {
    fn fmt_node(&self, indent: usize, f: &mut Formatter<'_>) {
        self.writeln(indent, "Program(", f);
        self.main.fmt_node(indent + 1, f);
        self.writeln(indent, ")", f);
    }
}

impl DisplayNode for FnDef {
    fn fmt_node(&self, indent: usize, f: &mut Formatter<'_>) {
        self.writeln(indent, "Function(", f);
        self.writeln(indent + 1, &format!("name = {}", self.name), f);
        self.writeln(indent + 1, "instructions = [", f);
        self.body
            .iter()
            .for_each(|instr| instr.fmt_node(indent + 1, f));
        self.writeln(indent + 1, "]", f);
        self.writeln(indent, ")", f);
    }
}

impl DisplayNode for Instruction {
    fn fmt_node(&self, indent: usize, f: &mut Formatter<'_>) {
        self.write(indent, "Instruction [", f);
        match self {
            Instruction::Mov { src, dst } => {
                self.write(0, "Mov(src: ", f);
                src.fmt_node(0, f);
                self.write(0, ", dest: ", f);
                dst.fmt_node(0, f);
                self.write(0, ")", f);
            }
        }
        self.writeln(0, "],", f);
    }
}

impl DisplayNode for Operand {
    fn fmt_node(&self, indent: usize, f: &mut Formatter<'_>) {
        match self {
            Operand::Immediate(value) => self.write(indent, &format!("Immediate({})", value), f),
            Operand::Register => self.write(indent, "Register", f),
        }
    }
}
