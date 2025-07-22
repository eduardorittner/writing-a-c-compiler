use ast::Tree;

use crate::{FnDef, NodeKind, Program};

trait Lower {
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
            ast::NodeKind::Stmt(stmt) => todo!(),
            ast::NodeKind::Expr(expr) => todo!(),
            ast::NodeKind::Ident(ident) => todo!(),
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
        let a = tree[self.name];
        FnDef {
            name: tree[self.name],
            body: todo!(),
        }
    }
}

impl Lower for ast::Stmt {
    fn lower(&self, tree: &Tree) -> NodeKind {
        todo!()
    }
}

impl Lower for ast::Expr {
    fn lower(&self, tree: &Tree) -> NodeKind {
        todo!()
    }
}

impl Lower for ast::Ident {
    fn lower(&self, tree: &Tree) -> NodeKind {
        todo!()
    }
}
