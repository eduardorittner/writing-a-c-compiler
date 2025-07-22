use std::ops::Index;

use lex::Token;

macro_rules! node_type(
    ($type:ident, $typeId:ident) => {

        impl<'a> From<&'a Node> for &'a $type {
            fn from(node: &'a Node) -> &'a $type {
                match &node.kind {
                    NodeKind::$type(inner) => inner,
                    _ => panic!(),
                }
            }
        }

        impl Index<$typeId> for Tree {
            type Output = $type;

            // SAFETY: Since the ast is immutable and we only construct valid `NodeId`s, all `NodeId`s
            // are always valid
            fn index(&self, index: $typeId) -> &Self::Output {
                self.nodes.get(index.0).unwrap().into()
            }
        }
    };
);

/// An AST Node
///
/// A node has an optional reference to a token, since every ast node has at most one direct token, and may have zero.
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
pub enum NodeKind {
    Program(Program),
    FnDef(FnDef),
    Stmt(Stmt),
    Expr(Expr),
    Ident(Ident),
}

// TODO every node should contain a reference to its underlying token

pub struct Program(FnDefId);

pub struct FnDef {
    name: IdentId,
    body: StmtId,
}

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

pub enum Expr {
    Constant { value: i64, token: Token },
}

pub struct Ident {
    token: Token,
}

pub struct NodeId(usize);
pub struct ProgramId(usize);
pub struct FnDefId(usize);
pub struct StmtId(usize);
pub struct ExprId(usize);
pub struct IdentId(usize);

node_type!(Program, ProgramId);
node_type!(FnDef, FnDefId);
node_type!(Stmt, StmtId);
node_type!(Expr, ExprId);
node_type!(Ident, IdentId);

pub struct Tree {
    nodes: Vec<Node>,
}

<<<<<<< Updated upstream
||||||| Stash base
// TODO fix this
impl<T> Tree {
    pub fn push(&mut self, node: T)
    where
        T: Into<Node>,
    {
        self.nodes.push(node.into())
    }
}

=======
impl Tree {
    pub fn push<T>(&mut self, node: T)
    where
        T: Into<Node>,
    {
        self.nodes.push(node.into())
    }
}

>>>>>>> Stashed changes
impl Index<NodeId> for Tree {
    type Output = Node;

    // SAFETY: Since the ast is immutable and we only construct valid `NodeId`s, all `NodeId`s
    // are always valid
    fn index(&self, index: NodeId) -> &Self::Output {
        self.nodes.get(index.0).unwrap()
    }
}
