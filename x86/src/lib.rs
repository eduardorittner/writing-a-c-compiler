use ast::Tree;
use lower::Lower;
use tracing::{Level, span};

pub mod fmt;
pub mod lower;

pub struct X86 {
    pub nodes: Program,
}

pub enum NodeKind {
    Program(Program),
    FnDef(FnDef),
    Instructions(Vec<Instruction>),
    Operand(Operand),
}

pub struct Program {
    pub main: FnDef,
}

pub struct FnDef {
    pub name: String,
    pub body: Vec<Instruction>,
}

pub enum Instruction {
    Mov { src: Operand, dst: Operand },
    Ret,
}

pub enum Operand {
    Immediate(i64),
    Register,
}

pub fn lower(input: &Tree) -> X86 {
    let _ = span!(Level::TRACE, "X86 lowering");

    X86 {
        nodes: match input.program_node().kind {
            ast::NodeKind::Program(program) => program.lower(input),
            _ => unreachable!(),
        },
    }
}
