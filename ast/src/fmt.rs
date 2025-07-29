use std::fmt::{Display, Formatter};

use crate::{Constant, Expr, FnDef, Ident, Program, Stmt, Tree};

trait DisplayNode {
    fn fmt_node(&self, indent: usize, tree: &Tree, f: &mut Formatter<'_>);
    fn write(&self, indent: usize, string: &str, f: &mut Formatter<'_>) {
        write!(f, "{}{}", " ".repeat(indent), string).unwrap();
    }
    fn writeln(&self, indent: usize, string: &str, f: &mut Formatter<'_>) {
        writeln!(f, "{}{}", " ".repeat(indent), string).unwrap();
    }
}

impl<'src> Display for Tree<'src> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // find program definition and go from there
        let prog_node = self.program_node();

        prog_node.fmt_node(0, self, f);

        Ok(())
    }
}

impl DisplayNode for Program {
    fn fmt_node(&self, indent: usize, tree: &Tree, f: &mut Formatter<'_>) {
        self.writeln(indent, "Program(", f);

        tree[self.main].fmt_node(indent + 1, tree, f);

        self.writeln(indent, ")", f);
    }
}

impl DisplayNode for FnDef {
    fn fmt_node(&self, indent: usize, tree: &Tree, f: &mut Formatter<'_>) {
        self.writeln(indent, "Function(", f);

        self.write(indent, "Name = ", f);
        tree[self.name].fmt_node(indent + 1, tree, f);
        self.writeln(0, ",", f);

        self.write(indent, "Body = ", f);
        tree[self.body].fmt_node(indent + 1, tree, f);

        self.writeln(indent, ")", f);
    }
}

impl DisplayNode for Stmt {
    fn fmt_node(&self, indent: usize, tree: &Tree, f: &mut Formatter<'_>) {
        match self {
            Stmt::Return { expr, .. } => {
                self.writeln(0, "Return(", f);
                tree[expr].fmt_node(indent + 1, tree, f);
                self.writeln(indent, ")", f);
            }
            Stmt::If {
                cond,
                cond_true,
                cond_false,
            } => {
                self.writeln(indent, "If(", f);
                tree[cond].fmt_node(indent + 1, tree, f);
                self.writeln(indent, ") Then (", f);
                tree[cond_true].fmt_node(indent + 1, tree, f);

                match cond_false {
                    Some(cond_false) => {
                        self.writeln(indent, ") Else (", f);
                        tree[cond_false].fmt_node(indent + 1, tree, f);
                        self.writeln(indent, ")", f);
                    }
                    None => {
                        self.writeln(indent, ")", f);
                    }
                }
            }
        }
    }
}

impl DisplayNode for Constant {
    fn fmt_node(&self, _indent: usize, _tree: &Tree, f: &mut Formatter<'_>) {
        self.write(0, &format!("{}", self.value), f);
    }
}

impl DisplayNode for Expr {
    fn fmt_node(&self, indent: usize, tree: &Tree, f: &mut Formatter<'_>) {
        match self {
            Expr::Constant { constant, .. } => {
                self.write(indent, "Constant(", f);
                tree[constant].fmt_node(indent + 1, tree, f);
                self.writeln(indent, ")", f);
            }
        }
    }
}

impl DisplayNode for Ident {
    fn fmt_node(&self, _indent: usize, tree: &Tree, f: &mut Formatter<'_>) {
        self.write(
            0,
            &format!("{}", tree.tokens.token_text(self.token.handle)),
            f,
        );
    }
}
