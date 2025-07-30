use ast::Tree;
use lex::Token;
use lower::Lower;
use tracing::{Level, span};

pub mod fmt;
pub mod lower;

pub struct AAst {
    nodes: Program,
}

pub enum NodeKind {
    Program(Program),
    FnDef(FnDef),
    Instructions(Vec<Instruction>),
    Operand(Operand),
}

pub struct Program {
    main: FnDef,
}

pub struct FnDef {
    name: String,
    body: Vec<Instruction>,
}

pub enum Instruction {
    Mov { src: Operand, dst: Operand },
}

pub enum Operand {
    Immediate(i64),
    Register,
}

pub fn lower(input: &Tree) -> AAst {
    let _ = span!(Level::TRACE, "tacky lowering");

    AAst {
        nodes: match input.program_node().kind {
            ast::NodeKind::Program(program) => program.lower(input),
            _ => unreachable!(),
        },
    }
}
