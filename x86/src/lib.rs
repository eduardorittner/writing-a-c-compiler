use ast::Tree;
use lower::Lower;
use tracing::{Level, span};

pub mod fmt;
pub mod lower;

#[derive(Debug, PartialEq)]
pub struct X86 {
    pub nodes: Program,
}

#[derive(Debug, PartialEq)]
pub enum NodeKind {
    Program(Program),
    FnDef(FnDef),
    Instructions(Vec<Instruction>),
    Operand(Operand),
}

#[derive(Debug, PartialEq)]
pub struct Program {
    pub main: FnDef,
}

#[derive(Debug, PartialEq)]
pub struct FnDef {
    pub name: String,
    pub body: Vec<Instruction>,
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Mov { src: Operand, dst: Operand },
    Ret,
}

#[derive(Debug, PartialEq)]
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
