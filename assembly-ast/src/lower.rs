use ast::Tree;

use crate::{FnDef, Instruction, NodeKind, Operand, Program};

pub trait Lower {
    type Output;

    fn lower(&self, tree: &Tree) -> Self::Output;
}

impl Lower for ast::Node {
    type Output = NodeKind;

    fn lower(&self, tree: &Tree) -> NodeKind {
        self.kind.lower(tree)
    }
}

impl Lower for ast::NodeKind {
    type Output = NodeKind;

    fn lower(&self, tree: &Tree) -> NodeKind {
        match self {
            ast::NodeKind::Program(program) => NodeKind::Program(program.lower(tree)),
            ast::NodeKind::FnDef(fn_def) => NodeKind::FnDef(fn_def.lower(tree)),
            ast::NodeKind::Stmt(stmt) => NodeKind::Instructions(stmt.lower(tree)),
            ast::NodeKind::Expr(expr) => NodeKind::Operand(Operand::Immediate(expr.lower(tree))),
            ast::NodeKind::Ident(ident) => todo!(),
            ast::NodeKind::Constant(constant) => {
                NodeKind::Operand(Operand::Immediate(constant.lower(tree)))
            }
        }
    }
}

impl Lower for ast::Program {
    type Output = Program;

    fn lower(&self, tree: &Tree) -> Program {
        Program {
            main: tree[self.main].lower(tree),
        }
    }
}

impl Lower for ast::FnDef {
    type Output = FnDef;

    fn lower(&self, tree: &Tree) -> FnDef {
        FnDef {
            name: tree
                .tokens
                .token_text(tree[self.name].token.handle)
                .to_string(),
            body: tree[self.body].lower(tree),
        }
    }
}

impl Lower for ast::Stmt {
    type Output = Vec<Instruction>;

    fn lower(&self, tree: &Tree) -> Vec<Instruction> {
        let mut instrs = Vec::new();
        match self {
            ast::Stmt::Return { expr, .. } => {
                instrs.push(Instruction::Mov {
                    src: Operand::Immediate(tree[expr].lower(tree)),
                    dst: Operand::Register,
                });
            }
            ast::Stmt::If { .. } => todo!(),
        };
        instrs
    }
}

impl Lower for ast::Expr {
    type Output = i64;

    fn lower(&self, tree: &Tree) -> i64 {
        match self {
            ast::Expr::Constant { constant } => tree[constant].lower(tree),
        }
    }
}

impl Lower for ast::Constant {
    type Output = i64;

    fn lower(&self, _tree: &Tree) -> i64 {
        self.value
    }
}
