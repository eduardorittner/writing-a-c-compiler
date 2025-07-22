use lex::Token;

pub mod lower;

pub enum NodeKind {
    Program(Program),
    FnDef(FnDef),
    Instruction(Instruction),
    Operand(Operand),
}

pub struct Program {
    main: FnDef,
}

pub struct FnDef {
    name: Token,
    body: Vec<Instruction>,
}

pub enum Instruction {
    Mov { src: Operand, dst: Operand },
}

pub enum Operand {
    Immediate(i64),
    Register,
}
