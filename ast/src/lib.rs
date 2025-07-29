use std::{fmt::Display, ops::Index};

use lex::{Token, TokenizedOutput};

pub mod fmt;

macro_rules! node_type(
    ($type:ident, $typeId:ident) => {

        impl From<usize> for $typeId {
            fn from(value: usize) -> Self {
                Self(value)
            }
        }

        impl From<$typeId> for usize {
            fn from(value: $typeId) -> Self {
                value.0
            }
        }

        impl<'a> From<&'a Node> for &'a $type {
            fn from(node: &'a Node) -> &'a $type {
                match &node.kind {
                    NodeKind::$type(inner) => inner,
                    _ => panic!(),
                }
            }
        }

        impl From<$type> for Node {
            fn from(ttype: $type) -> Node {
                Node {
                    kind: NodeKind::$type(ttype)
                }
            }
        }

        impl<'src> Index<$typeId> for Tree<'src> {
            type Output = $type;

            // SAFETY: Since the ast is immutable and we only construct valid `NodeId`s, all `NodeId`s
            // are always valid
            fn index(&self, index: $typeId) -> &Self::Output {
                self.nodes.get(index.0).unwrap().into()
            }
        }

        impl<'src> Index<&$typeId> for Tree<'src> {
            type Output = $type;

            // SAFETY: Since the ast is immutable and we only construct valid `NodeId`s, all `NodeId`s
            // are always valid
            fn index(&self, index: &$typeId) -> &Self::Output {
                self.nodes.get(index.0).unwrap().into()
            }
        }
    };
);

/// An AST Node
///
/// A node has an optional reference to a token, since every ast node has at most one direct token, and may have zero.
#[derive(Debug)]
pub struct Node {
    kind: NodeKind,
}

/// A NodeKind is a enum of all possible AST nodes
///
/// All variants contain a struct with the same node which holds the actual data, this is done so a
/// function can specify exactly what AST nodes it consumes/produces. Any reference to other nodes
/// is done through an Id, which is just the index of that NodeKind inside the `Tree` vector. Ids are
/// also typed to again allow for explictly declaring what each node holds, even though they're all
/// stored contiguously. Trying to access a node as another variant will result in a panic, this
/// should never happen if we construct the AST correctly, since it's immutable.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum NodeKind {
    Program(Program),
    FnDef(FnDef),
    Stmt(Stmt),
    Expr(Expr),
    Ident(Ident),
    Constant(Constant),
}

// TODO every node should contain a reference to its underlying token

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Program(FnDefId);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct FnDef {
    name: IdentId,
    body: StmtId,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Stmt {
    Return {
        expr: ExprId,
        token: Token,
    },
    If {
        // If
        cond: ExprId,
        // Then
        cond_true: StmtId,
        // Else
        cond_false: Option<StmtId>,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Expr {
    Constant { value: i64, token: Token },
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Ident {
    token: Token,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Constant {
    pub value: i64,
    pub token: Token,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct NodeId(usize);
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ProgramId(usize);
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct FnDefId(usize);
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct StmtId(usize);
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ExprId(usize);
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct IdentId(usize);
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ConstantId(usize);

node_type!(Program, ProgramId);
node_type!(FnDef, FnDefId);
node_type!(Stmt, StmtId);
node_type!(Expr, ExprId);
node_type!(Ident, IdentId);
node_type!(Constant, ConstantId);

/// An AST representation
///
/// Nodes are pushed once they're complete, which means that subnodes appear before their parent
/// nodes.
///
/// For example, a program like
///
/// ```c
/// int main() {
///     return 0;
/// }
/// ```
///
/// will result in a tree like
///
/// [type "int", expr "0", return, function "main"]
#[derive(Debug)]
pub struct Tree<'src> {
    tokens: &'src TokenizedOutput<'src>,
    nodes: Vec<Node>,
}

impl<'src> Tree<'src> {
    pub fn new(tokens: &'src TokenizedOutput<'src>) -> Tree<'src> {
        Tree {
            tokens,
            nodes: Vec::new(),
        }
    }

    pub fn with_capacity(tokens: &'src TokenizedOutput<'src>, cap: usize) -> Tree<'src> {
        Tree {
            tokens,
            nodes: Vec::with_capacity(cap),
        }
    }

    pub fn push<T, Id>(&mut self, node: T) -> Id
    where
        T: Into<Node>,
        Id: From<usize>,
    {
        self.nodes.push(node.into());
        (self.nodes.len() - 1).into()
    }

    fn program_node(&self) -> &Program {
        let program_index = self
            .nodes
            .iter()
            .position(|n| match n.kind {
                NodeKind::Program(_) => true,
                _ => false,
            })
            .unwrap();

        self.index(ProgramId::from(program_index))
    }
}

impl Index<NodeId> for Tree<'_> {
    type Output = Node;

    // SAFETY: Since the ast is immutable and we only construct valid `NodeId`s, all `NodeId`s
    // are always valid
    fn index(&self, index: NodeId) -> &Self::Output {
        self.nodes.get(index.0).unwrap()
    }
}
